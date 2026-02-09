import boto3

ssm = boto3.client("ssm")


def get_secret(user: str):
    response = ssm.get_parameter(Name=f"/secrets/{user}/secret", WithDecryption=True)
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
