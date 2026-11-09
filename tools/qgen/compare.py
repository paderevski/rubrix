#!/usr/bin/env python3
"""Compare the app's current design (baseline arm) with the new pipeline on identical specs, then audit BOTH arms.

The pipeline optimizes against lint/comments/exec/solve in its own loop, so those cannot fairly judge it. The audit therefore adds
checks it never saw: an adjudicator and a leakage judge (different prompts), and a blind solver on a different model.
Deterministic hygiene (lint, comments) is still reported because it is the outcome you care about, but it is the pipeline's target.

usage: compare.py [--specs 8] [--per-spec 2] [--model sonnet] [--audit-solve-model opus] [--out-dir DIR]
"""
import argparse, collections, json, random, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import bank, generate, review  # noqa: E402


def make_specs(n, per_spec, seed):
    rng = random.Random(seed)
    subj = bank.Subject()
    pool = [t for t in subj.with_questions() if t.startswith("T0") and int(t[1:]) <= 14 and t != "T014"]
    topics = rng.sample(pool, min(n, len(pool)))
    diffs = ["easy", "medium", "hard"]
    return [{"topics": [t], "difficulty": diffs[i % 3], "count": per_spec, "notes": None} for i, t in enumerate(topics)]


def coverage_specs(per_topic_levels=("easy", "medium", "hard", "hard")):
    """Every topic the app can offer (those with bank questions), one question per listed difficulty."""
    subj = bank.Subject()
    return [{"topics": [t], "difficulty": d, "count": 1, "notes": None} for t in sorted(subj.with_questions()) for d in per_topic_levels]


def audit(q, ctx_main, ctx_solve):
    """Held-out checks on one delivered-or-not question. Returns {layer: verdict/detail}."""
    if "text" not in q:
        return {}
    res = {}
    for name, fn, ctx in [("lint", review.layer_lint, ctx_main), ("comments", review.layer_comments, ctx_main), ("leak", review.layer_leak, ctx_main),
                          ("adjudicate", review.layer_adjudicate, ctx_main), ("solve_audit", review.layer_solve, ctx_solve)]:
        try:
            res[name] = fn(q, ctx)
        except Exception as e:  # a failed call is "skip", never a crash and never a silent pass
            res[name] = {"verdict": "skip", "detail": f"audit error: {str(e)[:120]}", "cost": 0.0}
    return res


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--resume", action="store_true", help="skip generation and reuse generated.json in --out-dir")
    ap.add_argument("--arms", default="baseline,pipeline", help="comma list of: baseline, pipeline")
    ap.add_argument("--coverage", action="store_true", help="all app-offered topics x easy/medium/hard/hard, 1 question each (60 questions)")
    ap.add_argument("--specs", type=int, default=8)
    ap.add_argument("--per-spec", type=int, default=2)
    ap.add_argument("--model", default="sonnet")
    ap.add_argument("--audit-model", default="sonnet")
    ap.add_argument("--audit-solve-model", default="opus")
    ap.add_argument("--rounds", type=int, default=2)
    ap.add_argument("--choices", type=int, default=5)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("-j", "--jobs", type=int, default=8)
    ap.add_argument("--out-dir", default=".")
    a = ap.parse_args()
    out = Path(a.out_dir)
    out.mkdir(parents=True, exist_ok=True)
    specs = coverage_specs() if a.coverage else make_specs(a.specs, a.per_spec, a.seed)
    arm_names = [x for x in a.arms.split(",") if x]
    print("specs:", [(s["topics"][0], s["difficulty"]) for s in specs], flush=True)

    class Args:  # the knobs generate.py expects
        model, rounds, choices, examples, jobs = a.model, a.rounds, a.choices, 3, 4
        loop_layers = "lint,comments,exec,solve"

    arms = {}
    if a.resume and (out / "generated.json").exists():
        arms = json.load(open(out / "generated.json"))
        print("resumed: reusing", {k: len(v) for k, v in arms.items()}, flush=True)
        # regenerate only specs whose every entry failed (e.g. calls lost to a rate limit); keep everything that worked
        runners = {"baseline": (generate.run_baseline, bank.Subject(with_blocks=False)), "pipeline": (generate.run_pipeline, bank.Subject(with_blocks=True))}
        for arm, entries in list(arms.items()):
            groups = {}
            for q in entries:
                groups.setdefault(int(q["id"].split("-")[1]) // 100, []).append(q)
            redo = sorted(i for i, g in groups.items() if all(q["review"]["state"] == "unparsed" for q in g))
            print(f"{arm}: regenerating {len(redo)} failed specs", flush=True)
            fn, subj = runners[arm]
            with ThreadPoolExecutor(max_workers=a.jobs) as pool:
                fut = {i: pool.submit(fn, specs[i], Args, subj, a.seed, i * 100) for i in redo}
                for i, f in fut.items():
                    groups[i] = f.result()
            arms[arm] = [q for i in sorted(groups) for q in groups[i]]
    else:
        subj_new, subj_old = bank.Subject(with_blocks=True), bank.Subject(with_blocks=False)
        with ThreadPoolExecutor(max_workers=a.jobs) as pool:
            futs = {}
            if "baseline" in arm_names:
                futs["baseline"] = [pool.submit(generate.run_baseline, sp, Args, subj_old, a.seed, i * 100) for i, sp in enumerate(specs)]
            if "pipeline" in arm_names:
                futs["pipeline"] = [pool.submit(generate.run_pipeline, sp, Args, subj_new, a.seed, i * 100) for i, sp in enumerate(specs)]
            for arm, fl in futs.items():
                arms[arm] = [q for f in fl for q in f.result()]
    json.dump(arms, open(out / "generated.json", "w"), indent=1, ensure_ascii=False)

    ctx_main = {"model": a.audit_model, "solve_n": 1, "choice_counts": {a.choices}, "adj_explanations": False, "blocks": {}}
    ctx_solve = dict(ctx_main, model=a.audit_solve_model)
    audits = {}
    with ThreadPoolExecutor(max_workers=a.jobs) as pool:
        for arm, qs in arms.items():
            audits[arm] = list(pool.map(lambda q: audit(q, ctx_main, ctx_solve), qs))
    json.dump({arm: [{"id": q["id"], "audit": {k: {x: y for x, y in v.items() if x != "block"} for k, v in au.items()}} for q, au in zip(arms[arm], audits[arm])] for arm in arms},
              open(out / "audit.json", "w"), indent=1)

    want = sum(sp["count"] for sp in specs)
    names = list(arms)
    print("\n" + f"{'':<38}" + "".join(f"{n:>12}" for n in names))
    def row(label, fn):
        print(f"{label:<38}" + "".join(f"{fn(arm):>12}" for arm in names))
    delivered = lambda arm: [(q, au) for q, au in zip(arms[arm], audits[arm]) if "text" in q]
    row("questions requested", lambda arm: want)
    row("questions produced", lambda arm: len(delivered(arm)))
    row("calls lost to unparseable output", lambda arm: sum(1 for q in arms[arm] if q["review"]["state"] == "unparsed"))
    for layer, label in [("lint", "structure defects (lint)"), ("comments", "explanatory comments"), ("leak", "leakage (judge)"),
                         ("adjudicate", "second defensible answer / wrong key"), ("solve_audit", "blind solver (opus) disagrees")]:
        row(label, lambda arm, l=layer: f"{sum(1 for _, au in delivered(arm) if au[l]['verdict'] == 'flag')}")
    row("ANY defect (audit layers)", lambda arm: f"{sum(1 for _, au in delivered(arm) if any(v['verdict'] == 'flag' for v in au.values()))} / {len(delivered(arm))}")
    row("key-level defect (adjud. or solve)", lambda arm: f"{sum(1 for _, au in delivered(arm) if au['adjudicate']['verdict'] == 'flag' or au['solve_audit']['verdict'] == 'flag')} / {len(delivered(arm))}")
    row("author cost per produced question", lambda arm: f"${sum(q['review'].get('cost', 0) for q in arms[arm]) / max(1, len(delivered(arm))):.3f}")
    if "pipeline" in arms:
        p = arms["pipeline"]
        print(f"\npipeline states: {dict(collections.Counter(q['review']['state'] for q in p))}; avg revision rounds {sum(q['review'].get('rounds', 0) for q in p) / max(1, len(p)):.2f}")
    print(f"audit cost: ${sum(v['cost'] for arm in names for au in audits[arm] for v in au.values()):.2f}")
    for arm in names:
        print(f"\nflagged by audit ({arm}, delivered):")
        for q, au in delivered(arm):
            fl = [(k, v["detail"][:170]) for k, v in au.items() if v["verdict"] == "flag"]
            if fl:
                print(f"  {q['id']} {q.get('topics')} {q.get('difficulty')}", *[f"\n     {k}: {d}" for k, d in fl])


if __name__ == "__main__":
    main()
