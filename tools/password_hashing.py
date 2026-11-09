"""
Password record format shared by the user tools and the gateway Lambda.

The desktop app sends SHA-256(password) as hex ("client hash"). SSM stores a slow,
salted scrypt of that value, so reading the parameter is not enough to log in:

    scrypt$<N>$<r>$<p>$<salt base64>$<hash base64>

Must stay in sync with verifyPasswordHash() in
lambda_functions/bedrock_gateway/lambda_handler.js.
"""

import base64
import hashlib
import hmac
import os
import re

REGION = "us-east-1"  # where the gateway Lambda reads parameters

SCRYPT_N = 2**15
SCRYPT_R = 8
SCRYPT_P = 1
SCRYPT_DKLEN = 32
SCRYPT_MAXMEM = 256 * 1024 * 1024
SALT_BYTES = 16

_LEGACY_RE = re.compile(r"^[0-9a-f]{64}$")


def client_hash(password: str) -> str:
    """What the desktop app sends: SHA-256 hex of the password."""
    return hashlib.sha256(password.encode()).hexdigest()


def make_record(client_hash_hex: str) -> str:
    """Build a stored scrypt record from a client hash."""
    salt = os.urandom(SALT_BYTES)
    digest = hashlib.scrypt(
        client_hash_hex.encode(),
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
    return stored.startswith("scrypt$")


def is_legacy_record(stored: str) -> bool:
    return bool(_LEGACY_RE.match(stored))


def verify(client_hash_hex: str, stored: str) -> bool:
    """Check a client hash against a stored record (scrypt or legacy SHA-256)."""
    if is_scrypt_record(stored):
        parts = stored.split("$")
        if len(parts) != 6:
            return False
        _, n, r, p, salt_b64, hash_b64 = parts
        expected = base64.b64decode(hash_b64)
        actual = hashlib.scrypt(
            client_hash_hex.encode(),
            salt=base64.b64decode(salt_b64),
            n=int(n),
            r=int(r),
            p=int(p),
            maxmem=SCRYPT_MAXMEM,
            dklen=len(expected),
        )
        return hmac.compare_digest(actual, expected)
    return hmac.compare_digest(client_hash_hex.encode(), stored.encode())
