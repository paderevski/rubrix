import boto3
import json

ssm = boto3.client("ssm")


def lambda_handler(event, context):
    """
    Lambda handler for secret retrieval with pre-hashed password.

    Expected input (POST body):
    {
        "user": "username",
        "password_hash": "sha256_hash_of_password"
    }
    """
    try:
        body = json.loads(event.get("body", "{}"))
        user = body.get("user")
        password_hash = body.get("password_hash")

        if not user or not password_hash:
            return {
                "statusCode": 400,
                "body": json.dumps({"error": "Missing user or password_hash"}),
            }
    except json.JSONDecodeError:
        return {"statusCode": 400, "body": json.dumps({"error": "Invalid JSON"})}

    try:
        # Get stored hash from Parameter Store
        stored_hash_param = ssm.get_parameter(
            Name=f"/secrets/{user}/password_hash", WithDecryption=True
        )
        stored_hash = stored_hash_param["Parameter"]["Value"]

        # Simple string comparison
        if password_hash == stored_hash:
            # Password hash matches - get secret
            secret_param = ssm.get_parameter(
                Name=f"/secrets/{user}/secret", WithDecryption=True
            )
            return {
                "statusCode": 200,
                "headers": {"Content-Type": "application/json"},
                "body": json.dumps({"secret": secret_param["Parameter"]["Value"]}),
            }
        else:
            return {
                "statusCode": 401,
                "body": json.dumps({"error": "Invalid credentials"}),
            }

    except ssm.exceptions.ParameterNotFound:
        return {"statusCode": 404, "body": json.dumps({"error": "User not found"})}
    except Exception as e:
        print(f"Error: {str(e)}")
        return {
            "statusCode": 500,
            "body": json.dumps({"error": "Internal server error"}),
        }
