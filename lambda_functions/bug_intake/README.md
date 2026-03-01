# Bug Intake Lambda (GitHub Issue Router)

This Lambda receives the canonical bug report envelope from Catie, computes a fingerprint, and either:

- creates a new GitHub issue, or
- deduplicates by adding a comment to an existing open issue with the same fingerprint.

It returns a JSON payload shaped for the desktop app:

- `report_id`
- `issue_id`
- `issue_url`

## Expected Input

The request body should follow the schema in [docs/BUG_REPORT_SCHEMA.json](../../docs/BUG_REPORT_SCHEMA.json).

## Required Environment Variables

- `GITHUB_OWNER`
- `GITHUB_REPO`
- `GITHUB_TOKEN` (GitHub fine-grained PAT or GitHub App token with Issues write access)

## Optional Environment Variables

- `INGEST_API_KEY` (if set, request must include header `x-api-key` with matching value)
- `DEFAULT_LABELS` (comma-separated, default: `bug`)
- `RAW_REPORTS_S3_BUCKET` (if set, raw incoming report JSON is stored to S3)
- `RAW_REPORTS_S3_PREFIX` (default: `bug-reports/`)

## API Gateway Notes

Configure an HTTP API or REST API integration that forwards the request body to this Lambda.

If you enable `INGEST_API_KEY`, set the same value in Catie via `BUG_REPORT_API_KEY`.

## Sample Success Response

```json
{
  "status": "created",
  "report_id": "bug_1740750000000_ab12cd34",
  "issue_id": "123",
  "issue_url": "https://github.com/ORG/REPO/issues/123",
  "fingerprint": "1a2b3c4d5e6f7a8b"
}
```

## Deployment (zip)

From this folder:

```bash
zip function.zip lambda_handler.py
```

Then upload `function.zip` to Lambda (Python 3.11 runtime recommended).

AWS CLI automation script: [deploy_example.sh](deploy_example.sh)

For full AWS setup (IAM roles, API Gateway, required services, smoke tests), see [docs/BUG_REPORT_DEPLOYMENT.md](../../docs/BUG_REPORT_DEPLOYMENT.md).
