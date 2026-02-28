# Bug Intake Deployment Runbook (AWS + GitHub)

This guide covers the full deployment for Catie bug intake:

- Lambda function (`lambda_functions/bug_intake/lambda_handler.py`)
- API Gateway endpoint used by Catie (`BUG_REPORT_URL`)
- IAM role and permissions
- Optional S3 archival of raw JSON bug reports
- End-to-end smoke test

## 1) Create/prepare required services

## 1.1 GitHub token

Create one of the following:

- Fine-grained PAT with access to your target repo and permission: **Issues (Read/Write)**
- OR GitHub App installation token with equivalent issue permissions

Set token value aside for Lambda env variable `GITHUB_TOKEN`.

## 1.2 Lambda function

- Runtime: Python 3.11
- Handler: `lambda_handler.lambda_handler`
- Source file: `lambda_functions/bug_intake/lambda_handler.py`

Package (from repo root):

```bash
cd lambda_functions/bug_intake
zip function.zip lambda_handler.py
```

Upload `function.zip` to your Lambda.

## 1.3 API Gateway

Create HTTP API (recommended) with one route:

- Method: `POST`
- Path: `/bug-report`
- Integration: your Lambda function

Deploy to a stage (for example `prod`) and note endpoint URL like:

`https://abc123.execute-api.us-east-1.amazonaws.com/prod/bug-report`

Set this URL in app env as `BUG_REPORT_URL`.

## 2) Lambda environment variables

Required:

- `GITHUB_OWNER` = your org/user
- `GITHUB_REPO` = target repository
- `GITHUB_TOKEN` = GitHub token

Optional:

- `INGEST_API_KEY` = shared secret expected in `x-api-key`
- `DEFAULT_LABELS` = comma-separated labels, e.g. `bug,from-catie`
- `RAW_REPORTS_S3_BUCKET` = bucket for raw payload archival
- `RAW_REPORTS_S3_PREFIX` = key prefix, default `bug-reports/`

## 3) IAM role and permissions

Attach AWS managed policy for logs:

- `AWSLambdaBasicExecutionRole`

If raw archival is enabled, add inline policy (replace placeholders):

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowPutBugReports",
      "Effect": "Allow",
      "Action": ["s3:PutObject"],
      "Resource": "arn:aws:s3:::YOUR_BUCKET_NAME/bug-reports/*"
    }
  ]
}
```

If you store secrets in Secrets Manager/SSM (recommended for production), add read-only access to exact ARN(s), e.g.:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowReadBugIntakeSecret",
      "Effect": "Allow",
      "Action": ["secretsmanager:GetSecretValue"],
      "Resource": "arn:aws:secretsmanager:us-east-1:123456789012:secret:bug-intake/*"
    }
  ]
}
```

## 4) API Gateway auth and CORS

For Tauri desktop calls, CORS is usually not required. If you also call from web browser contexts, enable CORS for `POST` and relevant origins.

Recommended protection:

- Use `INGEST_API_KEY` in Lambda and set same value in app as `BUG_REPORT_API_KEY`
- Optionally add API Gateway auth layer (Lambda authorizer/JWT/IAM)

## 5) Configure Catie app

In `src-tauri/.env` (or deployment env):

- `BUG_REPORT_URL` = API Gateway route URL
- `BUG_REPORT_API_KEY` = same value as Lambda `INGEST_API_KEY` (if used)
- `BUG_REPORT_BEARER_TOKEN` = optional alternate auth header

## 6) Smoke test endpoint directly

Use this sample payload (matches schema constants):

```bash
curl -X POST "https://YOUR_API_ID.execute-api.us-east-1.amazonaws.com/prod/bug-report" \
  -H "Content-Type: application/json" \
  -H "x-api-key: YOUR_INGEST_API_KEY" \
  -d '{
    "schema_version": "catie.bug_report.v1",
    "event_type": "bug_report.submitted",
    "event_id": "bug_test_001",
    "occurred_at": "2026-02-28T12:00:00Z",
    "app": {
      "product_name": "Catie",
      "package_name": "catie",
      "version": "0.9.0"
    },
    "reporter": { "username": "tester", "email": "tester@example.com" },
    "bug": {
      "title": "Test bug",
      "description": "Submit bug test",
      "steps_to_reproduce": ["open app", "click submit bug"],
      "expected_behavior": "request accepted",
      "actual_behavior": "n/a",
      "severity": "low"
    },
    "context": {
      "selected_subject": "Computer Science",
      "selected_topics": ["recursion"],
      "question_count": 3,
      "active_tab": "generate",
      "status": "Ready",
      "is_authenticated": true,
      "is_dev_mode": true,
      "app_zoom": 1.0,
      "preview_visible": true,
      "streaming_chars": 0,
      "user_agent": "smoke-test",
      "captured_at": "2026-02-28T12:00:00Z"
    }
  }'
```

Expected response includes one of:

- New issue path: `status: created`, `issue_id`, `issue_url`
- Dedup path: `status: deduplicated`, `issue_id`, `issue_url`

## 7) Verify in app

- Start app and use `Help -> Submit Bug`
- Confirm success dialog contains `Event ID`
- Confirm issue created/updated in GitHub

## 8) Operational recommendations

- Rotate GitHub token regularly
- Keep Lambda timeout at 10-20s (GitHub API calls are external)
- Add CloudWatch alarms on Lambda errors and API 5xx rates
- Add dead-letter handling if you later make flow async
- If volume grows, fan-in to SQS first and process asynchronously

## Reference docs

- Workflow: [BUG_REPORT_WORKFLOW.md](BUG_REPORT_WORKFLOW.md)
- Schema: [BUG_REPORT_SCHEMA.json](BUG_REPORT_SCHEMA.json)
- Lambda source: [../lambda_functions/bug_intake/lambda_handler.py](../lambda_functions/bug_intake/lambda_handler.py)
- Lambda quick notes: [../lambda_functions/bug_intake/README.md](../lambda_functions/bug_intake/README.md)
