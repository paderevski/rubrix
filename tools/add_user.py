#!/usr/bin/env python3
"""
Add a user/password combination to the Lambda secret store.

This script stores TWO parameters in AWS SSM Parameter Store:
1. /secrets/{user}/password_hash - SHA256 hash of the password (for authentication)
2. /secrets/{user}/secret - The Bedrock API key (returned after successful auth)

Usage:
    python add_user.py <username> <password> <bedrock_api_key>

Example:
    python add_user.py alice mypassword123 "ABSKQmVkcm9ja0FQSUtleS0..."
"""

import sys
import hashlib
import boto3
import re

ssm = boto3.client("ssm")


def hash_password(password: str) -> str:
    """Hash password using SHA256 (same algorithm as client)"""
    return hashlib.sha256(password.encode()).hexdigest()


def sanitize_user(username: str) -> str:
    """Make username safe for SSM parameter paths."""
    sanitized = re.sub(r"[^A-Za-z0-9._-]", "_", username)
    if sanitized.lower().startswith("ssm"):
        sanitized = f"user_{sanitized}"
    return sanitized


def add_user(username: str, password: str, bedrock_api_key: str):
    """Store both password hash and Bedrock API key for a user"""
    password_hash = hash_password(password)
    safe_user = sanitize_user(username)

    if safe_user != username:
        print(f"INFO: Normalized username '{username}' -> '{safe_user}'")

    # Store password hash for validation
    ssm.put_parameter(
        Name=f"/secrets/{safe_user}/password_hash",
        Value=password_hash,
        Type="SecureString",
        Overwrite=True,
    )
    print(f"✓ Stored password hash for user: {username}")

    # Store Bedrock API key to return on successful auth
    ssm.put_parameter(
        Name=f"/secrets/{safe_user}/secret",
        Value=bedrock_api_key,
        Type="SecureString",
        Overwrite=True,
    )
    print(f"✓ Stored Bedrock API key for user: {username}")

    print(f"\nUser '{username}' is now configured for authentication.")
    print(f"Password hash (for verification): {password_hash}")


def main():
    if len(sys.argv) != 4:
        print(__doc__)
        sys.exit(1)

    username = sys.argv[1]
    password = sys.argv[2]
    bedrock_api_key = sys.argv[3]

    if not username or not password or not bedrock_api_key:
        print("Error: All arguments must be non-empty")
        sys.exit(1)

    try:
        add_user(username, password, bedrock_api_key)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
