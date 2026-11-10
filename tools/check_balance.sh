#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <username>"
  exit 1
fi

RAW_USER="$1"
AWS_REGION="${AWS_REGION:-us-east-1}"
USAGE_TABLE_NAME="${USAGE_TABLE_NAME:-catieBedrockUsage}"

normalize_user() {
  local user="$1"
  local sanitized
  sanitized="$(printf '%s' "$user" | sed -E 's/[^A-Za-z0-9._-]/_/g')"

  if [[ -z "$sanitized" ]]; then
    printf ''
    return
  fi

  local lowered
  lowered="$(printf '%s' "$sanitized" | tr '[:upper:]' '[:lower:]')"
  if [[ "$lowered" == ssm* ]]; then
    printf 'user_%s' "$sanitized"
  else
    printf '%s' "$sanitized"
  fi
}

SAFE_USER="$(normalize_user "$RAW_USER")"
if [[ -z "$SAFE_USER" ]]; then
  echo "Error: username is empty after normalization"
  exit 1
fi

KEY_JSON=$(cat <<EOF
{"user_id":{"S":"$SAFE_USER"},"record_id":{"S":"summary"}}
EOF
)

ITEM_JSON=$(aws dynamodb get-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --output json)

HAS_ITEM=$(aws dynamodb get-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --query 'Item != null' \
  --output text)

if [[ "$HAS_ITEM" != "True" ]]; then
  echo "No summary row found for user_id=$SAFE_USER in $USAGE_TABLE_NAME ($AWS_REGION)."
  exit 0
fi

remaining=$(aws dynamodb get-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --query 'Item.remaining_tokens.N' \
  --output text)

budget=$(aws dynamodb get-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --query 'Item.budget_tokens.N' \
  --output text)

used=$(aws dynamodb get-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --query 'Item.total_tokens.N' \
  --output text)

updated_at=$(aws dynamodb get-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --query 'Item.updated_at.S' \
  --output text)

echo "user_id:         $SAFE_USER"
echo "table:           $USAGE_TABLE_NAME"
echo "region:          $AWS_REGION"
echo "remaining_tokens: ${remaining:-N/A}"
echo "budget_tokens:    ${budget:-N/A}"
echo "total_tokens:     ${used:-N/A}"
echo "updated_at:       ${updated_at:-N/A}"
