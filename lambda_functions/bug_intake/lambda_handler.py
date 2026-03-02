import base64
import hashlib
import json
import os
import re
import urllib.parse
import urllib.request
from decimal import Decimal
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


def _to_dynamo_safe(value):
    if isinstance(value, float):
        return Decimal(str(value))
    if isinstance(value, list):
        return [_to_dynamo_safe(v) for v in value]
    if isinstance(value, dict):
        return {k: _to_dynamo_safe(v) for k, v in value.items()}
    return value


def _store_bug_report_if_configured(
    payload: dict,
    event_id: str,
    fingerprint: str,
    status: str,
    issue_id: str | None,
    issue_url: str | None,
):
    table_name = os.environ.get("BUG_REPORTS_TABLE")
    if not table_name or boto3 is None:
        return None

    now_iso = datetime.now(timezone.utc).isoformat()
    severity = str(payload.get("bug", {}).get("severity", "medium")).lower()
    reporter = (
        payload.get("reporter", {}) if isinstance(payload.get("reporter"), dict) else {}
    )
    app = payload.get("app", {}) if isinstance(payload.get("app"), dict) else {}
    context = (
        payload.get("context", {}) if isinstance(payload.get("context"), dict) else {}
    )

    ttl_days_raw = os.environ.get("BUG_REPORTS_TTL_DAYS", "90")
    ttl_days = 90
    try:
        ttl_days = max(1, int(ttl_days_raw))
    except Exception:
        ttl_days = 90

    ttl_epoch = int(datetime.now(timezone.utc).timestamp()) + ttl_days * 24 * 60 * 60

    item = {
        "report_id": event_id,
        "created_at": now_iso,
        "occurred_at": payload.get("occurred_at"),
        "status": status,
        "fingerprint": fingerprint,
        "schema_version": payload.get("schema_version"),
        "event_type": payload.get("event_type"),
        "severity": severity,
        "reporter_username": reporter.get("username"),
        "reporter_email": reporter.get("email"),
        "app_version": app.get("version"),
        "app_package": app.get("package_name"),
        "active_tab": context.get("active_tab"),
        "selected_subject": context.get("selected_subject"),
        "issue_id": issue_id,
        "issue_url": issue_url,
        "ttl": ttl_epoch,
        "payload": payload,
    }

    dynamo = boto3.resource("dynamodb")
    table = dynamo.Table(table_name)
    table.put_item(Item=_to_dynamo_safe(item))
    return f"dynamodb://{table_name}/{event_id}"


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

            db_location = None
            try:
                db_location = _store_bug_report_if_configured(
                    payload=payload,
                    event_id=event_id,
                    fingerprint=fingerprint,
                    status="deduplicated",
                    issue_id=(
                        str(existing.get("number")) if existing.get("number") else None
                    ),
                    issue_url=existing.get("html_url"),
                )
            except Exception as db_error:
                print(f"DynamoDB persistence failed for {event_id}: {db_error}")

            return _response(
                200,
                {
                    "status": "deduplicated",
                    "report_id": event_id,
                    "issue_id": str(existing["number"]),
                    "issue_url": existing.get("html_url"),
                    "fingerprint": fingerprint,
                    "raw_json_url": raw_location,
                    "db_record_url": db_location,
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

        db_location = None
        try:
            db_location = _store_bug_report_if_configured(
                payload=payload,
                event_id=event_id,
                fingerprint=fingerprint,
                status="created",
                issue_id=str(created.get("number")) if created.get("number") else None,
                issue_url=created.get("html_url"),
            )
        except Exception as db_error:
            print(f"DynamoDB persistence failed for {event_id}: {db_error}")

        return _response(
            200,
            {
                "status": "created",
                "report_id": event_id,
                "issue_id": str(created.get("number")),
                "issue_url": created.get("html_url"),
                "fingerprint": fingerprint,
                "raw_json_url": raw_location,
                "db_record_url": db_location,
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
