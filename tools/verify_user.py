#!/usr/bin/env python3
"""
Verify user credentials in the Lambda secret store.

This script retrieves and displays the stored password hash and checks
if the provided password matches.

Usage:
    python verify_user.py <username> [password]

Example:
    python verify_user.py alice mypassword123
"""

import sys
import hashlib
import boto3

ssm = boto3.client("ssm")


def hash_password(password: str) -> str:
    """Hash password using SHA256 (same algorithm as client)"""
    return hashlib.sha256(password.encode()).hexdigest()


def verify_user(username: str, password: str = None):
    """Verify user exists and optionally check password"""
    try:
        # Get stored password hash
        hash_response = ssm.get_parameter(
            Name=f"/secrets/{username}/password_hash",
            WithDecryption=True
        )
        stored_hash = hash_response["Parameter"]["Value"]
        print(f"✓ User '{username}' found")
        print(f"Stored password hash: {stored_hash}")
        
        # Check if Bedrock API key exists
        try:
            key_response = ssm.get_parameter(
                Name=f"/secrets/{username}/secret",
                WithDecryption=True
            )
            key_value = key_response["Parameter"]["Value"]
            print(f"✓ Bedrock API key exists (length: {len(key_value)})")
            print(f"Key preview: {key_value[:20]}...")
        except ssm.exceptions.ParameterNotFound:
            print(f"✗ WARNING: Bedrock API key NOT found at /secrets/{username}/secret")
        
        # Verify password if provided
        if password:
            computed_hash = hash_password(password)
            print(f"\nComputed password hash: {computed_hash}")
            
            if computed_hash == stored_hash:
                print("✓ Password MATCHES!")
            else:
                print("✗ Password does NOT match")
                return False
        
        return True
        
    except ssm.exceptions.ParameterNotFound:
        print(f"✗ User '{username}' not found")
        print(f"   (Missing parameter: /secrets/{username}/password_hash)")
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
