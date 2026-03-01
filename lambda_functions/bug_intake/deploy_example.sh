#!/usr/bin/env bash
set -euo pipefail

# Example deployment script for bug intake Lambda + API Gateway (HTTP API)
# Fill in required variables before running.

AWS_REGION="us-east-1"
FUNCTION_NAME="catie-bug-intake"
ROLE_NAME="catie-bug-intake-role"
POLICY_NAME="catie-bug-intake-s3-policy"
API_NAME="catie-bug-intake-api"
STAGE_NAME="prod"
ROUTE_PATH="/bug-report"

GITHUB_OWNER="YOUR_GITHUB_OWNER"
GITHUB_REPO="YOUR_GITHUB_REPO"
GITHUB_TOKEN="YOUR_GITHUB_TOKEN"

INGEST_API_KEY="YOUR_SHARED_INGEST_KEY"
DEFAULT_LABELS="bug,from-catie"

# Optional S3 archival (leave empty to disable)
RAW_REPORTS_S3_BUCKET=""
RAW_REPORTS_S3_PREFIX="bug-reports/"

if [[ "$GITHUB_OWNER" == "YOUR_GITHUB_OWNER" || "$GITHUB_REPO" == "YOUR_GITHUB_REPO" || "$GITHUB_TOKEN" == "YOUR_GITHUB_TOKEN" ]]; then
  echo "Set GITHUB_OWNER, GITHUB_REPO, and GITHUB_TOKEN first."
  exit 1
fi

TMP_DIR="$(mktemp -d)"
cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

zip -q "$TMP_DIR/function.zip" lambda_handler.py

echo "Creating IAM role trust policy..."
cat > "$TMP_DIR/trust-policy.json" <<'JSON'
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": { "Service": "lambda.amazonaws.com" },
      "Action": "sts:AssumeRole"
    }
  ]
}
JSON

ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"

if aws iam get-role --role-name "$ROLE_NAME" >/dev/null 2>&1; then
  echo "Role exists: $ROLE_NAME"
else
  aws iam create-role \
    --role-name "$ROLE_NAME" \
    --assume-role-policy-document "file://$TMP_DIR/trust-policy.json" >/dev/null
fi

aws iam attach-role-policy \
  --role-name "$ROLE_NAME" \
  --policy-arn "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole" >/dev/null

ROLE_ARN="arn:aws:iam::$ACCOUNT_ID:role/$ROLE_NAME"

echo "Waiting for IAM role propagation..."
sleep 12

if aws lambda get-function --function-name "$FUNCTION_NAME" >/dev/null 2>&1; then
  echo "Updating existing Lambda function code..."
  aws lambda update-function-code \
    --function-name "$FUNCTION_NAME" \
    --zip-file "fileb://$TMP_DIR/function.zip" >/dev/null
else
  echo "Creating Lambda function..."
  aws lambda create-function \
    --function-name "$FUNCTION_NAME" \
    --runtime python3.11 \
    --handler lambda_handler.lambda_handler \
    --zip-file "fileb://$TMP_DIR/function.zip" \
    --role "$ROLE_ARN" \
    --timeout 20 \
    --memory-size 256 >/dev/null
fi

if [[ -n "$RAW_REPORTS_S3_BUCKET" ]]; then
  echo "Applying optional S3 put policy..."
  cat > "$TMP_DIR/s3-policy.json" <<JSON
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowPutBugReports",
      "Effect": "Allow",
      "Action": ["s3:PutObject"],
      "Resource": "arn:aws:s3:::$RAW_REPORTS_S3_BUCKET/$RAW_REPORTS_S3_PREFIX*"
    }
  ]
}
JSON

  aws iam put-role-policy \
    --role-name "$ROLE_NAME" \
    --policy-name "$POLICY_NAME" \
    --policy-document "file://$TMP_DIR/s3-policy.json" >/dev/null
fi

env_json=$(jq -n \
  --arg owner "$GITHUB_OWNER" \
  --arg repo "$GITHUB_REPO" \
  --arg token "$GITHUB_TOKEN" \
  --arg key "$INGEST_API_KEY" \
  --arg labels "$DEFAULT_LABELS" \
  --arg bucket "$RAW_REPORTS_S3_BUCKET" \
  --arg prefix "$RAW_REPORTS_S3_PREFIX" \
  '{Variables: {
      GITHUB_OWNER: $owner,
      GITHUB_REPO: $repo,
      GITHUB_TOKEN: $token,
      INGEST_API_KEY: $key,
      DEFAULT_LABELS: $labels,
      RAW_REPORTS_S3_BUCKET: $bucket,
      RAW_REPORTS_S3_PREFIX: $prefix
  }}')

aws lambda update-function-configuration \
  --function-name "$FUNCTION_NAME" \
  --environment "$env_json" >/dev/null

if aws apigatewayv2 get-apis --query "Items[?Name=='$API_NAME'].ApiId | [0]" --output text | grep -qv '^None$'; then
  API_ID="$(aws apigatewayv2 get-apis --query "Items[?Name=='$API_NAME'].ApiId | [0]" --output text)"
  echo "Using existing API: $API_ID"
else
  API_ID="$(aws apigatewayv2 create-api --name "$API_NAME" --protocol-type HTTP --query ApiId --output text)"
  echo "Created API: $API_ID"
fi

LAMBDA_ARN="arn:aws:lambda:$AWS_REGION:$ACCOUNT_ID:function:$FUNCTION_NAME"

if aws apigatewayv2 get-integrations --api-id "$API_ID" --query "Items[?IntegrationUri=='$LAMBDA_ARN'].IntegrationId | [0]" --output text | grep -qv '^None$'; then
  INTEGRATION_ID="$(aws apigatewayv2 get-integrations --api-id "$API_ID" --query "Items[?IntegrationUri=='$LAMBDA_ARN'].IntegrationId | [0]" --output text)"
else
  INTEGRATION_ID="$(aws apigatewayv2 create-integration \
    --api-id "$API_ID" \
    --integration-type AWS_PROXY \
    --integration-uri "$LAMBDA_ARN" \
    --payload-format-version 2.0 \
    --query IntegrationId --output text)"
fi

ROUTE_KEY="POST $ROUTE_PATH"
if aws apigatewayv2 get-routes --api-id "$API_ID" --query "Items[?RouteKey=='$ROUTE_KEY'].RouteId | [0]" --output text | grep -qv '^None$'; then
  ROUTE_ID="$(aws apigatewayv2 get-routes --api-id "$API_ID" --query "Items[?RouteKey=='$ROUTE_KEY'].RouteId | [0]" --output text)"
  aws apigatewayv2 update-route \
    --api-id "$API_ID" \
    --route-id "$ROUTE_ID" \
    --target "integrations/$INTEGRATION_ID" >/dev/null
else
  aws apigatewayv2 create-route \
    --api-id "$API_ID" \
    --route-key "$ROUTE_KEY" \
    --target "integrations/$INTEGRATION_ID" >/dev/null
fi

if aws apigatewayv2 get-stages --api-id "$API_ID" --query "Items[?StageName=='$STAGE_NAME'].StageName | [0]" --output text | grep -qv '^None$'; then
  aws apigatewayv2 update-stage \
    --api-id "$API_ID" \
    --stage-name "$STAGE_NAME" \
    --auto-deploy >/dev/null
else
  aws apigatewayv2 create-stage \
    --api-id "$API_ID" \
    --stage-name "$STAGE_NAME" \
    --auto-deploy >/dev/null
fi

aws lambda add-permission \
  --function-name "$FUNCTION_NAME" \
  --statement-id "${API_ID}-${STAGE_NAME}-invoke" \
  --action lambda:InvokeFunction \
  --principal apigateway.amazonaws.com \
  --source-arn "arn:aws:execute-api:$AWS_REGION:$ACCOUNT_ID:$API_ID/*/*" >/dev/null 2>&1 || true

ENDPOINT="https://${API_ID}.execute-api.${AWS_REGION}.amazonaws.com/${STAGE_NAME}${ROUTE_PATH}"

echo ""
echo "Deployment complete."
echo "BUG_REPORT_URL=$ENDPOINT"
echo "BUG_REPORT_API_KEY=$INGEST_API_KEY"
echo ""
echo "Set these in src-tauri/.env and restart Catie."
