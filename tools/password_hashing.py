"""
Password record format shared by the user tools and the gateway Lambda.

The desktop app sends the password over HTTPS. SSM stores a slow, salted scrypt
of it, so reading the parameter is not enough to log in:

    scrypt$<N>$<r>$<p>$<salt base64>$<hash base64>

Must stay in sync with verifyPassword() in
lambda_functions/bedrock_gateway/lambda_handler.js.
"""

import base64
import hashlib
import hmac
import os

REGION = "us-east-1"  # where the gateway Lambda reads parameters

SCRYPT_N = 2**15
SCRYPT_R = 8
SCRYPT_P = 1
SCRYPT_DKLEN = 32
SCRYPT_MAXMEM = 256 * 1024 * 1024
SALT_BYTES = 16


def make_record(password: str) -> str:
    """Build a stored scrypt record for a password."""
    salt = os.urandom(SALT_BYTES)
    digest = hashlib.scrypt(
        password.encode(),
        salt=salt,
        n=SCRYPT_N,
        r=SCRYPT_R,
        p=SCRYPT_P,
        maxmem=SCRYPT_MAXMEM,
        dklen=SCRYPT_DKLEN,
    )
    return "$".join(
        [
            "scrypt",
            str(SCRYPT_N),
            str(SCRYPT_R),
            str(SCRYPT_P),
            base64.b64encode(salt).decode(),
            base64.b64encode(digest).decode(),
        ]
    )


def is_scrypt_record(stored: str) -> bool:
    return stored.startswith("scrypt$") and len(stored.split("$")) == 6


def verify(password: str, stored: str) -> bool:
    """Check a password against a stored scrypt record."""
    if not is_scrypt_record(stored):
        return False
    _, n, r, p, salt_b64, hash_b64 = stored.split("$")
    expected = base64.b64decode(hash_b64)
    actual = hashlib.scrypt(
        password.encode(),
        salt=base64.b64decode(salt_b64),
        n=int(n),
        r=int(r),
        p=int(p),
        maxmem=SCRYPT_MAXMEM,
        dklen=len(expected),
    )
    return hmac.compare_digest(actual, expected)
