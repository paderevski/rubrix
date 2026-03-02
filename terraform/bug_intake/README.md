# Terraform: Bug Intake Stack

This provisions the bug-reporting backend for Catie:

- DynamoDB table for bug reports (`report_id` PK, TTL enabled)
- Python Lambda (`lambda_functions/bug_intake/lambda_handler.py`)
- HTTP API Gateway route: `POST /bug-report`
- IAM role + policies for Lambda execution + DynamoDB access

## Prereqs

- Terraform >= 1.5
- AWS credentials configured (`aws configure` or env vars)

## Quick Start

```bash
cd terraform/bug_intake
cp terraform.tfvars.example terraform.tfvars
# edit terraform.tfvars
terraform init
terraform plan
terraform apply
```

### Using the helper script (recommended with AWS login sessions)

If your AWS auth comes from `aws login` / `aws sso login`, use:

```bash
cd terraform/bug_intake
./tf.sh plan
./tf.sh apply
```

`tf.sh` exports credentials from the active AWS CLI session and then runs Terraform.

After apply, copy output `bug_report_url` into your app env:

- `src-tauri/.env` -> `BUG_REPORT_URL=<output bug_report_url>`
- optionally set `BUG_REPORT_API_KEY` to the same value as `ingest_api_key`

## Notes

- The Lambda package is built automatically from:
  - `../../lambda_functions/bug_intake/lambda_handler.py`
- `ingest_api_key` can be blank to disable header enforcement.
- `raw_reports_s3_bucket` is optional and only used if set.
