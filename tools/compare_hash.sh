#!/bin/bash
# Compare password hashing between Python and Rust

if [ $# -eq 0 ]; then
    echo "Usage: $0 <password>"
    exit 1
fi

PASSWORD="$1"

echo "Testing password: '$PASSWORD'"
echo ""

# Python hash
echo "Python hash:"
python3 -c "import hashlib; print(hashlib.sha256('$PASSWORD'.encode()).hexdigest())"
echo ""

# Rust hash using the auth module
echo "To test Rust hash, add this to your test:"
echo "cd src-tauri && cargo test --package rubrix --lib auth::tests::test_hash"
echo ""
echo "Or run manually:"
echo "python3 tools/test_hash.py '$PASSWORD'"
