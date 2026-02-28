# Bug Report Workflow (AWS Ingest)

Catie now supports in-app bug submission via `Help -> Submit Bug`.

## Flow

1. User fills out **Submit Bug** modal.
2. Frontend calls Tauri command: `submit_bug_report`.
3. Rust backend enriches payload with:
   - `event_id`
   - `schema_version`
   - app metadata (`product_name`, `package_name`, `version`)
   - optional diagnostics
4. Backend POSTs JSON to `BUG_REPORT_URL`.
5. Backend returns `event_id` and upstream metadata to UI.

## Environment Variables

Set in `src-tauri/.env` or deployment environment:

- `BUG_REPORT_URL` (required)
- `BUG_REPORT_API_KEY` (optional; sent as `x-api-key`)
- `BUG_REPORT_BEARER_TOKEN` (optional; sent as bearer token)

Sample AWS router implementation: [lambda_functions/bug_intake/lambda_handler.py](../lambda_functions/bug_intake/lambda_handler.py)
Deployment notes: [lambda_functions/bug_intake/README.md](../lambda_functions/bug_intake/README.md)
Full deployment runbook: [BUG_REPORT_DEPLOYMENT.md](BUG_REPORT_DEPLOYMENT.md)

## Canonical Schema

Schema definition: [BUG_REPORT_SCHEMA.json](BUG_REPORT_SCHEMA.json)

Envelope constants:

- `schema_version`: `catie.bug_report.v1`
- `event_type`: `bug_report.submitted`

## Suggested AWS Response Shape

Your endpoint can return any JSON, but these keys are automatically detected and surfaced in UI:

- ID keys: `id`, `issue_id`, `report_id`, `post_id`, `ticket_id`
- URL keys: `url`, `issue_url`, `post_url`, `ticket_url`, `canny_url`

Example response:

```json
{
  "report_id": "bug_2026_000123",
  "issue_url": "https://github.com/your-org/your-repo/issues/42",
  "status": "accepted"
}
```
