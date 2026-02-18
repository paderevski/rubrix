import boto3
import re

ssm = boto3.client("ssm")


def get_secret(user: str):
    safe_user = re.sub(r"[^A-Za-z0-9._-]", "_", user)
    if safe_user.lower().startswith("ssm"):
        safe_user = f"user_{safe_user}"
    response = ssm.get_parameter(
        Name=f"/secrets/{safe_user}/secret", WithDecryption=True
    )
    return response["Parameter"]["Value"]


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: python get_secret.py <user>")
        sys.exit(1)

    user = sys.argv[1]

    try:
        secret_value = get_secret(user)
        print(secret_value)
    except ssm.exceptions.ParameterNotFound:
        print(f"Secret not found for user: {user}", file=sys.stderr)
        sys.exit(1)
