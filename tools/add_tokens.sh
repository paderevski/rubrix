#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <username> <tokens_to_add>"
  exit 1
fi

RAW_USER="$1"
TOKENS_TO_ADD="$2"
AWS_REGION="${AWS_REGION:-us-east-1}"
USAGE_TABLE_NAME="${USAGE_TABLE_NAME:-catieBedrockUsage}"

if ! [[ "$TOKENS_TO_ADD" =~ ^[0-9]+$ ]]; then
  echo "Error: tokens_to_add must be a non-negative integer"
  exit 1
fi

if [[ "$TOKENS_TO_ADD" == "0" ]]; then
  echo "No-op: tokens_to_add is 0"
  exit 0
fi

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

NOW_UTC="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
KEY_JSON=$(cat <<EOF
{"user_id":{"S":"$SAFE_USER"},"record_id":{"S":"summary"}}
EOF
)

VALUES_JSON=$(cat <<EOF
{
  ":add":{"N":"$TOKENS_TO_ADD"},
  ":zero":{"N":"0"},
  ":now":{"S":"$NOW_UTC"}
}
EOF
)

aws dynamodb update-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --update-expression "SET remaining_tokens = if_not_exists(remaining_tokens, :zero) + :add, budget_tokens = if_not_exists(budget_tokens, :zero) + :add, total_tokens = if_not_exists(total_tokens, :zero), updated_at = :now" \
  --expression-attribute-values "$VALUES_JSON" \
  --return-values ALL_NEW \
  --output json >/dev/null

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

echo "Added $TOKENS_TO_ADD tokens for user_id=$SAFE_USER"
echo "remaining_tokens: ${remaining:-N/A}"
echo "budget_tokens:    ${budget:-N/A}"
