import boto3

ssm = boto3.client("ssm")


def store_secret(
    user: str, secret_value: str, kms_key_id: str | None = None, overwrite: bool = True
):
    kwargs = dict(
        Name=f"/secrets/{user}/secret",
        Value=secret_value,
        Type="SecureString",
        Overwrite=overwrite,
    )
    if kms_key_id:
        kwargs["KeyId"] = kms_key_id  # custom KMS CMK
    ssm.put_parameter(**kwargs)


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 3:
        print(
            "Usage: python store_secret.py <user> <secret_value> [kms_key_id] [overwrite]"
        )
        sys.exit(1)

    user = sys.argv[1]
    secret_value = sys.argv[2]
    kms_key_id = sys.argv[3] if len(sys.argv) > 3 else None
    overwrite = (
        sys.argv[4].lower() in ("true", "1", "yes") if len(sys.argv) > 4 else True
    )

    store_secret(user, secret_value, kms_key_id, overwrite)
    print(f"Secret stored for user: {user}")
