#!/usr/bin/env python3
"""
Delete a user from the Lambda secret store.

Removes /secrets/{user}/password_hash and /secrets/{user}/secret from SSM, which
immediately stops the user from logging in. Usage history in DynamoDB
(`catieBedrockUsage`: the budget summary plus per-request prompt/response logs)
is kept unless --purge-usage is given.

Usage:
    python delete_user.py <username> [--purge-usage] [-y|--yes]

Examples:
    python delete_user.py alice
    python delete_user.py alice --purge-usage --yes
"""

import re
import sys

import boto3

REGION = "us-east-1"
TABLE_NAME = "catieBedrockUsage"

ssm = boto3.client("ssm", region_name=REGION)
ddb = boto3.client("dynamodb", region_name=REGION)


def sanitize_user(username: str) -> str:
    """Make username safe for SSM parameter paths."""
    sanitized = re.sub(r"[^A-Za-z0-9._-]", "_", username)
    if sanitized.lower().startswith("ssm"):
        sanitized = f"user_{sanitized}"
    return sanitized


def existing_params(user: str):
    names = [f"/secrets/{user}/password_hash", f"/secrets/{user}/secret"]
    found = []
    for name in names:
        try:
            ssm.get_parameter(Name=name)
            found.append(name)
        except ssm.exceptions.ParameterNotFound:
            pass
    return found


def usage_keys(user: str):
    keys = []
    paginator = ddb.get_paginator("query")
    for page in paginator.paginate(
        TableName=TABLE_NAME,
        KeyConditionExpression="user_id = :u",
        ExpressionAttributeValues={":u": {"S": user}},
        ProjectionExpression="user_id, record_id",
    ):
        keys.extend(page["Items"])
    return keys


def purge_usage(keys):
    for i in range(0, len(keys), 25):
        batch = [{"DeleteRequest": {"Key": k}} for k in keys[i : i + 25]]
        while batch:
            resp = ddb.batch_write_item(RequestItems={TABLE_NAME: batch})
            batch = resp.get("UnprocessedItems", {}).get(TABLE_NAME, [])


def main():
    args = sys.argv[1:]
    flags = {a for a in args if a.startswith("-")}
    positional = [a for a in args if not a.startswith("-")]
    if len(positional) != 1 or not flags <= {"--purge-usage", "-y", "--yes"}:
        print(__doc__)
        sys.exit(1)

    username = positional[0]
    user = sanitize_user(username)
    if user != username:
        print(f"INFO: Normalized username '{username}' -> '{user}'")
    purge = "--purge-usage" in flags
    assume_yes = bool(flags & {"-y", "--yes"})

    params = existing_params(user)
    keys = usage_keys(user) if purge else []
    if not params and not keys:
        sys.exit(f"User '{user}' not found; nothing to delete")

    print(f"About to delete user '{user}':")
    for name in params:
        print(f"  SSM parameter: {name}")
    if purge:
        print(f"  DynamoDB records in {TABLE_NAME}: {len(keys)}")
    else:
        print(f"  (usage history in {TABLE_NAME} will be kept)")

    if not assume_yes and input("Type the username to confirm: ").strip() != user:
        sys.exit("Aborted")

    if params:
        ssm.delete_parameters(Names=params)
        print(f"✓ Deleted {len(params)} SSM parameter(s)")
    if keys:
        purge_usage(keys)
        print(f"✓ Deleted {len(keys)} usage record(s)")
    print(f"User '{user}' deleted.")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
