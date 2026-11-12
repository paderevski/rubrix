#!/usr/bin/env python3
"""
View and update per-user dollar budgets used by the Bedrock gateway.

Budgets live in DynamoDB table `catieBedrockUsage` (us-east-1), in the item
{user_id: <user>, record_id: "summary"}. Amounts are stored in nano-dollars
(1e-9 USD) so the gateway's per-token prices are whole numbers:

    budget_nanousd   total budget granted
    spent_nanousd    cost of all requests so far (the gateway adds to it atomically)
    total_tokens     tokens used, for reference

remaining = budget - spent. The gateway rejects requests (HTTP 429) once it
reaches 0. Users without a record get the gateway's default budget
(USAGE_DEFAULT_BUDGET_USD, $1.00 unless configured) on their first request.

The older token fields (budget_tokens, remaining_tokens) are still updated by
the gateway for old app versions but are no longer enforced; this tool ignores them.

Usage:
    python user_budget.py list
    python user_budget.py get <username>
    python user_budget.py set <username> <usd>     # new total budget, e.g. 2.50
    python user_budget.py add <username> <usd>     # grant extra dollars (negative to subtract)
    python user_budget.py reset <username>         # spent = 0 (full budget available again)

Examples:
    python user_budget.py get alice
    python user_budget.py add alice 1.00
"""

import re
import sys
from datetime import datetime, timezone
from decimal import Decimal, InvalidOperation

import boto3

REGION = "us-east-1"
TABLE_NAME = "catieBedrockUsage"
NANOUSD_PER_USD = 1_000_000_000
# Keep in sync with USAGE_DEFAULT_BUDGET_USD in the gateway (lambda_handler.js).
DEFAULT_BUDGET_NANOUSD = 1 * NANOUSD_PER_USD

ddb = boto3.client("dynamodb", region_name=REGION)
ssm = boto3.client("ssm", region_name=REGION)


def sanitize_user(username: str) -> str:
    """Normalize usernames exactly like the gateway and add_user.py."""
    sanitized = re.sub(r"[^A-Za-z0-9._-]", "_", username)
    if sanitized.lower().startswith("ssm"):
        sanitized = f"user_{sanitized}"
    return sanitized


def key(user: str) -> dict:
    return {"user_id": {"S": user}, "record_id": {"S": "summary"}}


def num(item: dict, field: str, default=None):
    value = item.get(field)
    return int(value["N"]) if value and "N" in value else default


def usd(nano: int) -> str:
    return f"${nano / NANOUSD_PER_USD:,.4f}"


def summarize(item: dict) -> dict:
    budget = num(item, "budget_nanousd", DEFAULT_BUDGET_NANOUSD)
    spent = num(item, "spent_nanousd", 0)
    return {
        "budget": budget,
        "spent": spent,
        "remaining": budget - spent,
        "tokens": num(item, "total_tokens", 0),
        "updated_at": item.get("updated_at", {}).get("S", "-"),
    }


def get_record(user: str):
    item = ddb.get_item(TableName=TABLE_NAME, Key=key(user)).get("Item")
    return summarize(item) if item else None


def update(user: str, expression: str, values: dict):
    values = {**values, ":now": {"S": datetime.now(timezone.utc).isoformat()}}
    ddb.update_item(
        TableName=TABLE_NAME,
        Key=key(user),
        UpdateExpression=expression,
        ExpressionAttributeValues=values,
    )


def require_user(user: str):
    """Avoid creating usage records for mistyped usernames."""
    try:
        ssm.get_parameter(Name=f"/secrets/{user}/password_hash")
    except ssm.exceptions.ParameterNotFound:
        sys.exit(f"Error: user '{user}' not found (no password_hash in SSM)")


def show(user: str, record):
    if record is None:
        print(f"{user}: no usage record yet (default budget {usd(DEFAULT_BUDGET_NANOUSD)} applies on first use)")
        return
    print(f"{user}:")
    print(f"  budget:    {usd(record['budget'])}")
    print(f"  spent:     {usd(record['spent'])}")
    print(f"  remaining: {usd(record['remaining'])}")
    print(f"  tokens:    {record['tokens']:,}")
    print(f"  updated:   {record['updated_at']}")


def parse_usd(text: str) -> int:
    """Dollar amount -> nano-dollars."""
    try:
        return int(Decimal(text.replace(",", "").lstrip("$")) * NANOUSD_PER_USD)
    except InvalidOperation:
        sys.exit(f"Error: '{text}' is not a dollar amount")


def list_all():
    rows = []
    paginator = ddb.get_paginator("scan")
    for page in paginator.paginate(
        TableName=TABLE_NAME,
        FilterExpression="record_id = :s",
        ExpressionAttributeValues={":s": {"S": "summary"}},
    ):
        for item in page["Items"]:
            rows.append((item["user_id"]["S"], summarize(item)))
    if not rows:
        print("No usage records found")
        return
    width = max(len(r[0]) for r in rows)
    print(f"{'user'.ljust(width)}  {'budget':>11}  {'spent':>11}  {'remaining':>11}  {'tokens':>12}")
    for user, r in sorted(rows):
        print(
            f"{user.ljust(width)}  {usd(r['budget']):>11}  {usd(r['spent']):>11}  "
            f"{usd(r['remaining']):>11}  {r['tokens']:>12,}"
        )


def main():
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        sys.exit(1)
    cmd = args[0]

    if cmd == "list" and len(args) == 1:
        list_all()
        return

    if cmd not in ("get", "set", "add", "reset") or len(args) < 2:
        print(__doc__)
        sys.exit(1)

    user = sanitize_user(args[1])
    if user != args[1]:
        print(f"INFO: Normalized username '{args[1]}' -> '{user}'")

    if cmd == "get" and len(args) == 2:
        show(user, get_record(user))
        return

    require_user(user)

    # spent_nanousd is never SET from a value read earlier, so these commands cannot
    # overwrite a charge the gateway records while they run.
    if cmd == "set" and len(args) == 3:
        budget = parse_usd(args[2])
        if budget < 0:
            sys.exit("Error: budget must be >= 0")
        update(user, "SET budget_nanousd = :b, updated_at = :now", {":b": {"N": str(budget)}})
    elif cmd == "add" and len(args) == 3:
        delta = parse_usd(args[2])
        update(
            user,
            "SET budget_nanousd = if_not_exists(budget_nanousd, :default) + :d, updated_at = :now",
            {":d": {"N": str(delta)}, ":default": {"N": str(DEFAULT_BUDGET_NANOUSD)}},
        )
    elif cmd == "reset" and len(args) == 2:
        update(user, "SET spent_nanousd = :zero, updated_at = :now", {":zero": {"N": "0"}})
    else:
        print(__doc__)
        sys.exit(1)

    show(user, get_record(user))


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
