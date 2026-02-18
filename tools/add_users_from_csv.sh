#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DRY_RUN=0

if [[ "${1:-}" == "-n" || "${1:-}" == "--dry-run" ]]; then
  DRY_RUN=1
fi

export DRY_RUN
CSV_PATH="$SCRIPT_DIR/catie-users.csv"
ADD_USER="$SCRIPT_DIR/add_user.py"

if [[ ! -f "$CSV_PATH" ]]; then
  echo "Missing CSV: $CSV_PATH" >&2
  exit 1
fi

if [[ ! -f "$ADD_USER" ]]; then
  echo "Missing add_user.py: $ADD_USER" >&2
  exit 1
fi

python3 - <<'PY'
import csv
import os
import subprocess
import sys

script_dir = os.path.dirname(os.path.abspath(__file__))
csv_path = os.path.join(script_dir, "catie-users.csv")
add_user = os.path.join(script_dir, "add_user.py")
dry_run = os.environ.get("DRY_RUN") == "1"

with open(csv_path, newline="") as f:
    reader = csv.DictReader(f)
    if "Email Address" not in reader.fieldnames or "Password" not in reader.fieldnames:
        raise SystemExit("CSV missing required columns: Email Address, Password")

    count = 0
    for row in reader:
        email = (row.get("Email Address") or "").strip()
        password = (row.get("Password") or "").strip()
        if not email or not password:
            continue
        cmd = [sys.executable, add_user, email, password, "NONE"]
        if dry_run:
          print("DRY RUN:", " ".join(cmd))
        else:
          subprocess.check_call(cmd)
        count += 1

    if dry_run:
      print(f"Dry run complete for {count} users")
    else:
      print(f"Added {count} users")
PY
