#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <username> <new_remaining_tokens> [--sync-budget]"
  exit 1
fi

RAW_USER="$1"
NEW_REMAINING="$2"
SYNC_BUDGET=false

if [[ $# -ge 3 ]]; then
  case "$3" in
    --sync-budget)
      SYNC_BUDGET=true
      ;;
    *)
      echo "Unknown option: $3"
      echo "Usage: $0 <username> <new_remaining_tokens> [--sync-budget]"
      exit 1
      ;;
  esac
fi

AWS_REGION="${AWS_REGION:-us-east-1}"
USAGE_TABLE_NAME="${USAGE_TABLE_NAME:-catieBedrockUsage}"

if ! [[ "$NEW_REMAINING" =~ ^[0-9]+$ ]]; then
  echo "Error: new_remaining_tokens must be a non-negative integer"
  exit 1
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

if [[ "$SYNC_BUDGET" == true ]]; then
  VALUES_JSON=$(cat <<EOF
{
  ":remaining":{"N":"$NEW_REMAINING"},
  ":budget":{"N":"$NEW_REMAINING"},
  ":zero":{"N":"0"},
  ":now":{"S":"$NOW_UTC"}
}
EOF
)

  UPDATE_EXPRESSION="SET remaining_tokens = :remaining, budget_tokens = :budget, total_tokens = if_not_exists(total_tokens, :zero), updated_at = :now"
else
  VALUES_JSON=$(cat <<EOF
{
  ":remaining":{"N":"$NEW_REMAINING"},
  ":zero":{"N":"0"},
  ":now":{"S":"$NOW_UTC"}
}
EOF
)

  UPDATE_EXPRESSION="SET remaining_tokens = :remaining, budget_tokens = if_not_exists(budget_tokens, :remaining), total_tokens = if_not_exists(total_tokens, :zero), updated_at = :now"
fi

aws dynamodb update-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --update-expression "$UPDATE_EXPRESSION" \
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

used=$(aws dynamodb get-item \
  --region "$AWS_REGION" \
  --table-name "$USAGE_TABLE_NAME" \
  --key "$KEY_JSON" \
  --query 'Item.total_tokens.N' \
  --output text)

echo "Set balance for user_id=$SAFE_USER"
echo "remaining_tokens: ${remaining:-N/A}"
echo "budget_tokens:    ${budget:-N/A}"
echo "total_tokens:     ${used:-N/A}"
