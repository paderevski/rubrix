#!/usr/bin/env python3
"""Generate the 60-question coverage set with the app's unchanged prompt plus extra "Additional Instructions"
(the app's own user-instructions slot), so a prompt idea can be tried without touching the repo.

  gen_variant.py NOTES_FILE OUT.json [--model haiku] [-j 8]
"""
import argparse, json, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
sys.path.insert(0, str(Path(__file__).parents[1] / "verify_java"))
import bank, compare, generate  # noqa: E402

ap = argparse.ArgumentParser()
ap.add_argument("notes_file")
ap.add_argument("out")
ap.add_argument("--model", default="haiku")
ap.add_argument("-j", "--jobs", type=int, default=8)
a = ap.parse_args()
notes = Path(a.notes_file).read_text().strip()
subj = bank.Subject()
specs = [dict(s, notes=notes) for s in compare.coverage_specs()]
args = argparse.Namespace(model=a.model)
with ThreadPoolExecutor(a.jobs) as ex:
    groups = list(ex.map(lambda s: generate.run_baseline(s, args, subj, 0), specs))
qs = [q for g in groups for q in g]
for i, q in enumerate(qs):
    q["id"] = f"var-{i}"
ok = [q for q in qs if q["review"]["state"] != "unparsed"]
json.dump(ok, open(a.out, "w"), indent=1)
print(f"{len(ok)}/{len(specs)} produced, cost ${sum(q['review'].get('cost', 0) for q in qs):.2f}")
