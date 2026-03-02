#!/usr/bin/env bash
set -euo pipefail

# Helper for Terraform in this folder when using AWS CLI login-based credentials.
# Usage:
#   ./tf.sh init
#   ./tf.sh plan
#   ./tf.sh apply
#   ./tf.sh destroy

ACTION="${1:-plan}"
shift || true

if ! command -v aws >/dev/null 2>&1; then
  echo "error: aws CLI not found in PATH"
  exit 1
fi

if ! command -v terraform >/dev/null 2>&1; then
  echo "error: terraform not found in PATH"
  exit 1
fi

if ! aws sts get-caller-identity >/dev/null 2>&1; then
  echo "error: AWS session not active. Run AWS login first (e.g., aws configure sso / aws sso login)."
  exit 1
fi

# Export short-lived env creds from current AWS CLI login session.
eval "$(aws configure export-credentials --format env)"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

case "$ACTION" in
  init)
    terraform init "$@"
    ;;
  plan)
    terraform init -input=false >/dev/null
    terraform plan "$@"
    ;;
  apply)
    terraform init -input=false >/dev/null
    terraform apply "$@"
    ;;
  destroy)
    terraform init -input=false >/dev/null
    terraform destroy "$@"
    ;;
  fmt)
    terraform fmt "$@"
    ;;
  validate)
    terraform init -backend=false -input=false >/dev/null
    terraform validate "$@"
    ;;
  *)
    echo "usage: ./tf.sh {init|plan|apply|destroy|fmt|validate} [terraform args...]"
    exit 2
    ;;
esac
