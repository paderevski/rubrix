#!/usr/bin/env python3
"""
View and update per-user token budgets used by the Bedrock gateway.

Budgets live in DynamoDB table `catieBedrockUsage` (us-east-1), in the item
{user_id: <user>, record_id: "summary"} with fields budget_tokens,
remaining_tokens and total_tokens. The gateway rejects requests (HTTP 429)
once remaining_tokens reaches 0. Users without a record get the default
budget (1,000,000) on their first request.

Usage:
    python user_budget.py list
    python user_budget.py get <username>
    python user_budget.py set <username> <budget>    # new total budget; remaining = budget - used
    python user_budget.py add <username> <tokens>    # grant extra tokens (negative to subtract)
    python user_budget.py reset <username>           # remaining = budget (clears usage from remaining)

Examples:
    python user_budget.py get alice
    python user_budget.py add alice 500000
"""

import re
import sys
from datetime import datetime, timezone

import boto3

REGION = "us-east-1"
TABLE_NAME = "catieBedrockUsage"
DEFAULT_BUDGET = 1000000

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


def get_record(user: str):
    item = ddb.get_item(TableName=TABLE_NAME, Key=key(user)).get("Item")
    if not item:
        return None
    return {
        "budget": num(item, "budget_tokens", DEFAULT_BUDGET),
        "remaining": num(item, "remaining_tokens", DEFAULT_BUDGET),
        "used": num(item, "total_tokens", 0),
        "updated_at": item.get("updated_at", {}).get("S", "-"),
    }


def write_record(user: str, budget: int, remaining: int, used: int):
    ddb.update_item(
        TableName=TABLE_NAME,
        Key=key(user),
        UpdateExpression=(
            "SET budget_tokens = :b, remaining_tokens = :r, "
            "total_tokens = :u, updated_at = :now"
        ),
        ExpressionAttributeValues={
            ":b": {"N": str(budget)},
            ":r": {"N": str(max(remaining, 0))},
            ":u": {"N": str(used)},
            ":now": {"S": datetime.now(timezone.utc).isoformat()},
        },
    )


def require_user(user: str):
    """Avoid creating usage records for mistyped usernames."""
    try:
        ssm.get_parameter(Name=f"/secrets/{user}/password_hash")
    except ssm.exceptions.ParameterNotFound:
        sys.exit(f"Error: user '{user}' not found (no password_hash in SSM)")


def show(user: str, record):
    if record is None:
        print(f"{user}: no usage record yet (default budget {DEFAULT_BUDGET:,} applies on first use)")
        return
    print(f"{user}:")
    print(f"  budget:    {record['budget']:,}")
    print(f"  used:      {record['used']:,}")
    print(f"  remaining: {record['remaining']:,}")
    print(f"  updated:   {record['updated_at']}")


def current(user: str) -> dict:
    return get_record(user) or {
        "budget": DEFAULT_BUDGET,
        "remaining": DEFAULT_BUDGET,
        "used": 0,
    }


def parse_int(text: str) -> int:
    try:
        return int(text.replace(",", ""))
    except ValueError:
        sys.exit(f"Error: '{text}' is not an integer")


def list_all():
    rows = []
    paginator = ddb.get_paginator("scan")
    for page in paginator.paginate(
        TableName=TABLE_NAME,
        FilterExpression="record_id = :s",
        ExpressionAttributeValues={":s": {"S": "summary"}},
    ):
        for item in page["Items"]:
            rows.append(
                (
                    item["user_id"]["S"],
                    num(item, "budget_tokens", DEFAULT_BUDGET),
                    num(item, "total_tokens", 0),
                    num(item, "remaining_tokens", DEFAULT_BUDGET),
                )
            )
    if not rows:
        print("No usage records found")
        return
    width = max(len(r[0]) for r in rows)
    print(f"{'user'.ljust(width)}  {'budget':>12}  {'used':>12}  {'remaining':>12}")
    for user, budget, used, remaining in sorted(rows):
        print(f"{user.ljust(width)}  {budget:>12,}  {used:>12,}  {remaining:>12,}")


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
    rec = current(user)

    if cmd == "set" and len(args) == 3:
        budget = parse_int(args[2])
        if budget < 0:
            sys.exit("Error: budget must be >= 0")
        write_record(user, budget, budget - rec["used"], rec["used"])
    elif cmd == "add" and len(args) == 3:
        delta = parse_int(args[2])
        write_record(
            user,
            max(rec["budget"] + delta, 0),
            rec["remaining"] + delta,
            rec["used"],
        )
    elif cmd == "reset" and len(args) == 2:
        write_record(user, rec["budget"], rec["budget"], rec["used"])
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
