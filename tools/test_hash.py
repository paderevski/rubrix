#!/usr/bin/env python3
"""
Test that Python and Rust produce the same SHA256 hashes.
"""

import hashlib
import sys


def hash_password_python(password: str) -> str:
    """How Python scripts hash (add_user.py, verify_user.py)"""
    return hashlib.sha256(password.encode()).hexdigest()


def test_hashes():
    test_cases = [
        ("test", "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"),
        (
            "mypassword123",
            "6e659deaa85842cdabb5c6305fcc40033ba43772ec00d45c2a3c921741a5e377",
        ),
        ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        ("alice", "2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90"),
    ]

    print("Testing password hashing (should match Rust):\n")
    all_passed = True
    for password, expected in test_cases:
        hash_val = hash_password_python(password)
        matches = hash_val == expected
        status = "✓" if matches else "✗"
        print(f"{status} Password: '{password}'")
        print(f"  Hash:     {hash_val}")
        print(f"  Expected: {expected}")
        if not matches:
            all_passed = False
        print()

    if all_passed:
        print("All tests passed! ✓")
    else:
        print("Some tests failed! ✗")
        sys.exit(1)

    # Test with command line arg if provided
    if len(sys.argv) > 1:
        password = sys.argv[1]
        hash_val = hash_password_python(password)
        print(f"\nYour password: '{password}'")
        print(f"Hash:          {hash_val}")


if __name__ == "__main__":
    test_hashes()
