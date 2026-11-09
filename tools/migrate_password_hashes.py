#!/usr/bin/env python3
"""
One-time migration: wrap legacy SHA-256 password hashes in scrypt.

Reads every /secrets/*/password_hash parameter. Each legacy value (64 hex chars,
i.e. SHA-256 of the password) is replaced with scrypt(<that value>), so users keep
their passwords and the desktop app needs no change. Already-migrated values are
skipped. Anything else is reported and left alone.

Previous values stay in SSM parameter history (get-parameter-history) if you need
to roll back.

Usage:
    python migrate_password_hashes.py           # dry run: report only
    python migrate_password_hashes.py --apply   # write the new records

After --apply succeeds, set ALLOW_LEGACY_PASSWORD_HASHES=false on the gateway Lambda.
"""

import sys

import boto3

from password_hashing import REGION, is_legacy_record, is_scrypt_record, make_record

PREFIX = "/secrets/"
SUFFIX = "/password_hash"


def password_hash_parameter_names(ssm):
    paginator = ssm.get_paginator("describe_parameters")
    pages = paginator.paginate(
        ParameterFilters=[{"Key": "Name", "Option": "BeginsWith", "Values": [PREFIX]}]
    )
    for page in pages:
        for param in page["Parameters"]:
            if param["Name"].endswith(SUFFIX):
                yield param["Name"]


def main():
    apply = "--apply" in sys.argv[1:]
    ssm = boto3.client("ssm", region_name=REGION)

    counts = {"migrated": 0, "would_migrate": 0, "already_scrypt": 0, "unrecognized": 0}
    for name in sorted(password_hash_parameter_names(ssm)):
        value = ssm.get_parameter(Name=name, WithDecryption=True)["Parameter"]["Value"]
        if is_scrypt_record(value):
            counts["already_scrypt"] += 1
            continue
        if not is_legacy_record(value):
            counts["unrecognized"] += 1
            print(f"? {name}: unrecognized format, left unchanged")
            continue
        if apply:
            ssm.put_parameter(
                Name=name, Value=make_record(value), Type="SecureString", Overwrite=True
            )
            counts["migrated"] += 1
            print(f"✓ {name}")
        else:
            counts["would_migrate"] += 1
            print(f"- {name}: would migrate")

    print()
    print(", ".join(f"{k}={v}" for k, v in counts.items()))
    if not apply and counts["would_migrate"]:
        print("Dry run only. Re-run with --apply to write the new records.")


if __name__ == "__main__":
    main()
