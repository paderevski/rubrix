#!/usr/bin/env python3
"""
Verify user credentials in the Lambda secret store.

This script reports whether the stored password record is scrypt or legacy
SHA-256, and checks if the provided password matches.

Usage:
    python verify_user.py <username> [password]

Example:
    python verify_user.py alice mypassword123
"""

import sys
import boto3
import re

from password_hashing import REGION, client_hash, is_scrypt_record, verify

ssm = boto3.client("ssm", region_name=REGION)


def sanitize_user(username: str) -> str:
    """Make username safe for SSM parameter paths."""
    sanitized = re.sub(r"[^A-Za-z0-9._-]", "_", username)
    if sanitized.lower().startswith("ssm"):
        sanitized = f"user_{sanitized}"
    return sanitized


def verify_user(username: str, password: str = None):
    """Verify user exists and optionally check password"""
    safe_user = sanitize_user(username)
    if safe_user != username:
        print(f"INFO: Normalized username '{username}' -> '{safe_user}'")
    try:
        # Get stored password hash
        hash_response = ssm.get_parameter(
            Name=f"/secrets/{safe_user}/password_hash", WithDecryption=True
        )
        stored_hash = hash_response["Parameter"]["Value"]
        print(f"✓ User '{username}' found")
        if is_scrypt_record(stored_hash):
            print("Password record: scrypt")
        else:
            print("Password record: LEGACY SHA-256 (run migrate_password_hashes.py)")

        # Check if Bedrock API key exists
        try:
            key_response = ssm.get_parameter(
                Name=f"/secrets/{safe_user}/secret", WithDecryption=True
            )
            key_value = key_response["Parameter"]["Value"]
            print(f"✓ Bedrock API key slot exists (length: {len(key_value)})")
        except ssm.exceptions.ParameterNotFound:
            print(
                f"✗ WARNING: Bedrock API key NOT found at /secrets/{safe_user}/secret"
            )

        # Verify password if provided
        if password:
            if verify(client_hash(password), stored_hash):
                print("✓ Password MATCHES!")
            else:
                print("✗ Password does NOT match")
                return False

        return True

    except ssm.exceptions.ParameterNotFound:
        print(f"✗ User '{username}' not found")
        print(f"   (Missing parameter: /secrets/{safe_user}/password_hash)")
        return False
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return False


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    username = sys.argv[1]
    password = sys.argv[2] if len(sys.argv) > 2 else None

    success = verify_user(username, password)
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
