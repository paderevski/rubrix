import base64
import hashlib
import json
import os
import re
import urllib.parse
import urllib.request
from datetime import datetime, timezone

try:
    import boto3
except Exception:  # pragma: no cover
    boto3 = None


def _response(status_code: int, payload: dict):
    return {
        "statusCode": status_code,
        "headers": {"Content-Type": "application/json"},
        "body": json.dumps(payload),
    }


def _header(headers: dict, key: str):
    if not headers:
        return None
    for k, v in headers.items():
        if k.lower() == key.lower():
            return v
    return None


def _parse_body(event: dict):
    body = event.get("body")
    if body is None:
        return {}
    if event.get("isBase64Encoded"):
        body = base64.b64decode(body).decode("utf-8")
    if isinstance(body, str):
        return json.loads(body)
    if isinstance(body, dict):
        return body
    raise ValueError("Invalid body format")


def _required_path(payload: dict, path: list[str]):
    cur = payload
    for key in path:
        if not isinstance(cur, dict) or key not in cur:
            raise ValueError(f"Missing required field: {'.'.join(path)}")
        cur = cur[key]
    return cur


def _compact(text: str):
    return re.sub(r"\s+", " ", (text or "").strip())


def _fingerprint(payload: dict):
    bug = payload.get("bug", {})
    app = payload.get("app", {})
    context = payload.get("context", {})

    parts = [
        _compact(str(bug.get("title", ""))).lower(),
        _compact(str(bug.get("description", ""))).lower(),
        "\n".join(
            [_compact(str(s)).lower() for s in bug.get("steps_to_reproduce", [])]
        ),
        _compact(str(app.get("version", ""))).lower(),
        _compact(str(context.get("selected_subject", ""))).lower(),
    ]
    digest = hashlib.sha256("\n".join(parts).encode("utf-8")).hexdigest()
    return digest[:16]


def _github_request(method: str, url: str, token: str, payload=None):
    headers = {
        "Accept": "application/vnd.github+json",
        "Authorization": f"Bearer {token}",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "catie-bug-intake-lambda",
    }

    data = None
    if payload is not None:
        data = json.dumps(payload).encode("utf-8")
        headers["Content-Type"] = "application/json"

    req = urllib.request.Request(url, data=data, method=method, headers=headers)
    with urllib.request.urlopen(req, timeout=15) as resp:
        body = resp.read().decode("utf-8")
        return json.loads(body) if body else {}


def _find_existing_issue(owner: str, repo: str, token: str, fingerprint: str):
    marker = f"catie-fingerprint:{fingerprint}"
    query = f'repo:{owner}/{repo} is:issue is:open in:body "{marker}"'
    url = "https://api.github.com/search/issues?q=" + urllib.parse.quote(query)
    data = _github_request("GET", url, token)
    items = data.get("items", []) if isinstance(data, dict) else []
    return items[0] if items else None


def _build_issue_body(payload: dict, event_id: str, fingerprint: str):
    app = payload.get("app", {})
    bug = payload.get("bug", {})
    context = payload.get("context", {})
    reporter = payload.get("reporter", {})

    steps = bug.get("steps_to_reproduce") or []
    steps_md = "\n".join([f"- {s}" for s in steps]) if steps else "- Not provided"

    metadata = {
        "event_id": event_id,
        "fingerprint": fingerprint,
        "schema_version": payload.get("schema_version"),
        "occurred_at": payload.get("occurred_at"),
        "app_version": app.get("version"),
        "selected_subject": context.get("selected_subject"),
        "selected_topics": context.get("selected_topics"),
        "severity": bug.get("severity"),
    }

    return (
        f"<!-- catie-fingerprint:{fingerprint} -->\n"
        f"<!-- catie-event-id:{event_id} -->\n\n"
        f"## Summary\n{bug.get('description', '')}\n\n"
        f"## Steps to Reproduce\n{steps_md}\n\n"
        f"## Expected Behavior\n{bug.get('expected_behavior') or 'Not provided'}\n\n"
        f"## Actual Behavior\n{bug.get('actual_behavior') or 'Not provided'}\n\n"
        f"## Reporter\n"
        f"- Username: {reporter.get('username') or 'unknown'}\n"
        f"- Email: {reporter.get('email') or 'not provided'}\n\n"
        f"## App Context\n"
        f"- Product: {app.get('product_name')}\n"
        f"- Version: {app.get('version')}\n"
        f"- Active Tab: {context.get('active_tab')}\n"
        f"- Subject: {context.get('selected_subject') or 'none'}\n"
        f"- Topics: {', '.join(context.get('selected_topics') or []) or 'none'}\n"
        f"- Questions in memory: {context.get('question_count')}\n"
        f"- Is authenticated: {context.get('is_authenticated')}\n\n"
        f"## Raw Metadata\n```json\n{json.dumps(metadata, indent=2)}\n```\n"
    )


def _store_raw_if_configured(payload: dict, event_id: str):
    bucket = os.environ.get("RAW_REPORTS_S3_BUCKET")
    if not bucket or boto3 is None:
        return None

    prefix = os.environ.get("RAW_REPORTS_S3_PREFIX", "bug-reports/")
    key = f"{prefix}{event_id}.json"

    s3 = boto3.client("s3")
    s3.put_object(
        Bucket=bucket,
        Key=key,
        Body=json.dumps(payload).encode("utf-8"),
        ContentType="application/json",
    )
    return f"s3://{bucket}/{key}"


def lambda_handler(event, context):
    try:
        expected_ingest_key = os.environ.get("INGEST_API_KEY")
        provided_key = _header(event.get("headers", {}), "x-api-key")
        if expected_ingest_key and provided_key != expected_ingest_key:
            return _response(401, {"error": "Invalid ingest API key"})

        payload = _parse_body(event)

        _required_path(payload, ["schema_version"])
        _required_path(payload, ["event_type"])
        _required_path(payload, ["event_id"])
        _required_path(payload, ["app", "version"])
        _required_path(payload, ["bug", "title"])
        _required_path(payload, ["bug", "description"])
        _required_path(payload, ["bug", "severity"])

        owner = os.environ["GITHUB_OWNER"]
        repo = os.environ["GITHUB_REPO"]
        token = os.environ["GITHUB_TOKEN"]

        event_id = str(payload.get("event_id"))
        severity = str(payload.get("bug", {}).get("severity", "medium")).lower()
        if severity not in {"low", "medium", "high", "critical"}:
            severity = "medium"

        fingerprint = _fingerprint(payload)

        existing = _find_existing_issue(owner, repo, token, fingerprint)

        raw_location = _store_raw_if_configured(payload, event_id)

        if existing:
            comment_lines = [
                f"New bug report matched fingerprint `{fingerprint}`.",
                f"- Event ID: {event_id}",
                f"- Reported at: {datetime.now(timezone.utc).isoformat()}",
                f"- App version: {payload.get('app', {}).get('version')}",
            ]
            if raw_location:
                comment_lines.append(f"- Raw JSON: {raw_location}")

            comment_url = f"https://api.github.com/repos/{owner}/{repo}/issues/{existing['number']}/comments"
            _github_request(
                "POST",
                comment_url,
                token,
                {"body": "\n".join(comment_lines)},
            )

            return _response(
                200,
                {
                    "status": "deduplicated",
                    "report_id": event_id,
                    "issue_id": str(existing["number"]),
                    "issue_url": existing.get("html_url"),
                    "fingerprint": fingerprint,
                    "raw_json_url": raw_location,
                },
            )

        labels_env = os.environ.get("DEFAULT_LABELS", "bug")
        labels = [label.strip() for label in labels_env.split(",") if label.strip()]
        labels.append(f"severity:{severity}")
        labels = sorted(set(labels))

        title = str(payload.get("bug", {}).get("title", "Untitled bug"))
        issue_title = f"[Bug] {title}"
        issue_body = _build_issue_body(payload, event_id, fingerprint)

        issue_payload = {
            "title": issue_title,
            "body": issue_body,
            "labels": labels,
        }

        create_url = f"https://api.github.com/repos/{owner}/{repo}/issues"
        created = _github_request("POST", create_url, token, issue_payload)

        return _response(
            200,
            {
                "status": "created",
                "report_id": event_id,
                "issue_id": str(created.get("number")),
                "issue_url": created.get("html_url"),
                "fingerprint": fingerprint,
                "raw_json_url": raw_location,
            },
        )

    except KeyError as err:
        return _response(
            500, {"error": f"Missing required environment variable: {err}"}
        )
    except urllib.error.HTTPError as err:
        body = err.read().decode("utf-8", errors="ignore")
        return _response(
            502,
            {
                "error": "GitHub API request failed",
                "status": err.code,
                "details": body[:2000],
            },
        )
    except ValueError as err:
        return _response(400, {"error": str(err)})
    except Exception as err:
        return _response(500, {"error": f"Unhandled error: {str(err)}"})
