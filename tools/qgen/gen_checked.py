#!/usr/bin/env python3
"""Generate with the app's prompt, run the free layers (lint, comments, exec) on each question, and regenerate any
flagged question up to --retries times. Reports first-attempt flag rate and what is left flagged.

  gen_checked.py OUT.json [--notes NOTES_FILE] [--model haiku] [--retries 2] [--seed 1] [-j 8]
Uses the 60-question coverage specs (15 topics x easy/medium/hard/hard, 1 question each).
"""
import argparse, json, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
sys.path.insert(0, str(Path(__file__).parents[1] / "verify_java"))
import bank, compare, generate, review  # noqa: E402

FREE = ("lint", "comments", "exec")
CTX = {"model": "sonnet", "solve_n": 1, "adj_explanations": False, "choice_counts": {4, 5}, "blocks": {}}


def free_flags(q):
    out = []
    for name in FREE:
        try:
            r = review.LAYERS[name](q, CTX)
        except Exception as e:
            continue
        if r["verdict"] == "flag":
            out.append(f"{name}: {str(r.get('detail', ''))[:120]}")
    return out


def one(spec, args, subj):
    history = []
    cost = 0.0
    for attempt in range(args.retries + 1):
        qs = generate.run_baseline(spec, args, subj, f"{args.seed}-r{attempt}")
        q = qs[0]
        cost += q["review"].get("cost", 0)
        if q["review"]["state"] == "unparsed":
            history.append(["unparsed: " + q["review"].get("error", "")[:80]])
            continue
        flags = free_flags(q)
        history.append(flags)
        if not flags:
            break
    q["gen"] = {"attempts": len(history), "history": history, "cost": round(cost, 4), "spec": spec}
    return q


ap = argparse.ArgumentParser()
ap.add_argument("out")
ap.add_argument("--notes")
ap.add_argument("--model", default="haiku")
ap.add_argument("--retries", type=int, default=2)
ap.add_argument("--seed", type=int, default=1)
ap.add_argument("-j", "--jobs", type=int, default=8)
a = ap.parse_args()
notes = Path(a.notes).read_text().strip() if a.notes else None
subj = bank.Subject()
specs = [dict(s, notes=notes) for s in compare.coverage_specs()]
with ThreadPoolExecutor(a.jobs) as ex:
    res = list(ex.map(lambda s: one(s, a, subj), specs))
for i, q in enumerate(res):
    q["id"] = f"chk-{i}"
first = sum(1 for q in res if q["gen"]["history"][0])
left = [q for q in res if q["gen"]["history"][-1]]
print(f"{len(res)} specs; flagged on first attempt: {first}; still flagged after retries: {len(left)}; "
      f"avg attempts {sum(q['gen']['attempts'] for q in res) / len(res):.2f}; cost ${sum(q['gen']['cost'] for q in res):.2f}")
for q in res:
    if q["gen"]["history"][0]:
        print(f"  {q['id']} attempts={q['gen']['attempts']} first={q['gen']['history'][0]} final={q['gen']['history'][-1]}")
json.dump([q for q in res if "text" in q], open(a.out, "w"), indent=1)
