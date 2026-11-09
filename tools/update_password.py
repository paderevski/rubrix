#!/usr/bin/env python3
"""Update the password of an existing Lambda secret-store user."""

import argparse
import getpass
import sys

from add_user import hash_password, sanitize_user, ssm


def update_password(username: str, password: str):
    safe_user = sanitize_user(username)
    if safe_user != username:
        print(f"INFO: Normalized username '{username}' -> '{safe_user}'")

    parameter_name = f"/secrets/{safe_user}/password_hash"
    try:
        ssm.get_parameter(Name=parameter_name)
    except ssm.exceptions.ParameterNotFound:
        raise ValueError(f"User '{username}' not found") from None

    ssm.put_parameter(
        Name=parameter_name,
        Value=hash_password(password),
        Type="SecureString",
        Overwrite=True,
    )
    print(f"Password updated for user '{username}'.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("username", help="Existing username")
    args = parser.parse_args()

    if not args.username:
        parser.error("username must be non-empty")

    try:
        password = getpass.getpass("New password: ")
        if not password:
            raise ValueError("Password must be non-empty")
        confirmation = getpass.getpass("Confirm new password: ")
        if password != confirmation:
            raise ValueError("Passwords do not match")
        update_password(args.username, password)
    except (Exception, KeyboardInterrupt) as error:
        print(f"Error: {error or 'Cancelled'}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()