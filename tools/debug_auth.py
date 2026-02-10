#!/usr/bin/env python3
"""
Debug authentication setup - verify hashing matches between Python and what Lambda expects.

Usage: python debug_auth.py <username> <password>
"""

import sys
import hashlib
import boto3

ssm = boto3.client("ssm")


def hash_password(password: str) -> str:
    """Hash password using SHA256 (same as client)"""
    return hashlib.sha256(password.encode()).hexdigest()


def debug_auth(username: str, password: str):
    """Debug authentication setup"""
    print(f"Debugging authentication for user: {username}")
    print(f"Password: {password}")
    print()

    # 1. Hash the password
    computed_hash = hash_password(password)
    print(f"1. Computed password hash:")
    print(f"   {computed_hash}")
    print()

    # 2. Check what's stored in SSM
    print(f"2. Checking SSM Parameter Store...")
    try:
        hash_response = ssm.get_parameter(
            Name=f"/secrets/{username}/password_hash", WithDecryption=True
        )
        stored_hash = hash_response["Parameter"]["Value"]
        print(f"   Stored hash:   {stored_hash}")
        print()

        # 3. Compare
        if computed_hash == stored_hash:
            print("3. ✓ HASHES MATCH - Password should work!")
        else:
            print("3. ✗ HASHES DO NOT MATCH - Password will fail!")
            print()
            print("   This means:")
            print("   - The password was stored differently")
            print(
                "   - Run: python add_user.py {} <correct_password> <api_key>".format(
                    username
                )
            )
        print()

        # 4. Check if secret exists
        try:
            key_response = ssm.get_parameter(
                Name=f"/secrets/{username}/secret", WithDecryption=True
            )
            key_value = key_response["Parameter"]["Value"]
            print(f"4. ✓ Bedrock API key exists (length: {len(key_value)})")
            print(f"   Preview: {key_value[:30]}...")
        except ssm.exceptions.ParameterNotFound:
            print(f"4. ✗ WARNING: Bedrock API key NOT found!")
            print(f"   Lambda will fail even with correct password")
            print(f"   Run: python add_user.py {username} {password} <bedrock_api_key>")

    except ssm.exceptions.ParameterNotFound:
        print(f"   ✗ User '{username}' not found in Parameter Store")
        print()
        print(
            "   Run: python add_user.py {} {} <bedrock_api_key>".format(
                username, password
            )
        )
        return False

    return True


def main():
    if len(sys.argv) != 3:
        print(__doc__)
        sys.exit(1)

    username = sys.argv[1]
    password = sys.argv[2]

    debug_auth(username, password)


if __name__ == "__main__":
    main()
