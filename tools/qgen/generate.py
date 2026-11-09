#!/usr/bin/env python3
"""Question generation pipelines, independent of the app. Reads the repo's knowledge base; writes only where --out points.

  --arm baseline   the app's current design: its own prompt.txt, rendered as the Rust app does, one call returning N questions,
                   parsed leniently. Run on whatever --model you pick, so it can stand in for the gateway model.
  --arm pipeline   one question per call from a rubric built from observed failures; the author emits the question, a
                   verification block and a trace; checks run the real Java and report concrete evidence; the author revises.

Pipeline checks are individually switchable (--loop-layers lint,comments,exec,solve). Each failed check becomes specific feedback.

usage:
  generate.py --arm pipeline --topics T006 --difficulty medium --count 3 --out out.json
  generate.py --arm baseline --topics T006,ST010 --difficulty hard --count 3
"""
import argparse, json, random, re, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / "verify_java"))
import bank, mine_logs, review, verify  # noqa: E402

STRUCTURES = {
    "value_trace": "determine a value, state, or output by tracing code",
    "code_selection": "choose the code that produces a specified result",
    "algorithm_selection": "choose the algorithm or procedure that accomplishes a goal",
    "equivalence": "determine which expression or code segment is equivalent",
    "error_analysis": "identify what is wrong, or which change fixes the code",
    "code_completion": "choose the missing statement or expression",
    "conceptual": "determine which statement about a concept is true",
    "modification": "determine what change would produce a specified result",
}

CHOICE = {"type": "object", "properties": {
    "id": {"type": "string"}, "text": {"type": "string"}, "is_correct": {"type": "boolean"},
    "misconception": {"type": "string"}, "explanation": {"type": "string"}}, "required": ["id", "text", "is_correct", "explanation"]}
AUTHOR_SCHEMA = {"type": "object", "properties": {
    "plan": {"type": "string"}, "structure": {"type": "string"}, "text": {"type": "string"},
    "answers": {"type": "array", "items": CHOICE}, "explanation": {"type": "string"},
    "trace": {"type": "array", "items": {"type": "object", "properties": {"line": {"type": "string"}, "state": {"type": "object", "additionalProperties": {"type": "string"}}}, "required": ["line", "state"]}},
    "verification": {"type": "object"},
    "difficulty": {"type": "string"}, "cognitive_level": {"type": "string"},
    "topics": {"type": "array", "items": {"type": "string"}}, "subtopics": {"type": "array", "items": {"type": "string"}}},
    "required": ["plan", "text", "answers", "explanation", "verification", "difficulty", "cognitive_level", "topics"]}
AUTHOR_SYSTEM = "You are an expert AP Computer Science A item writer and reviewer. You are exact: you compute every value and test every choice."


# ---------------------------------------------------------------- model calls

def llm_json(prompt, model, schema=None, system=AUTHOR_SYSTEM, timeout=600):
    """One headless call. Returns (structured object | text, cost_usd)."""
    cmd = ["claude", "-p", "--model", model, "--output-format", "json", "--tools", "", "--no-session-persistence",
           "--disable-slash-commands", "--system-prompt", system]
    if schema:
        cmd += ["--json-schema", json.dumps(schema)]
    err = None
    for _ in range(2):
        try:
            r = subprocess.run(cmd, input=prompt, capture_output=True, text=True, timeout=timeout)
            d = json.loads(r.stdout)
            if d.get("is_error"):
                raise RuntimeError(str(d.get("result"))[:200])
            out = d.get("structured_output") if schema else d.get("result")
            if out is None:
                raise RuntimeError("no structured output returned")
            return out, float(d.get("total_cost_usd") or 0)
        except Exception as e:
            err = e
    raise RuntimeError(f"model call failed: {err}")


# ---------------------------------------------------------------- pipeline

def author_prompt(subj, spec, structure, examples, args):
    t = (HERE / "prompts" / "author.md").read_text()
    code = bank.DIFFICULTY_CODE[spec["difficulty"]]
    extra = ("- If this structure truly cannot be applied to the topic, use the closest structure that can and record it in `structure`.\n"
             + (f"- Teacher's notes: {spec['notes']}\n" if spec.get("notes") else "")
             + f"- Use these ids: topics {json.dumps(spec['topics'])}, difficulty {code}.")
    ex = "\n\n".join(f"### Example {i + 1}\n```json\n{subj.example_json(e, with_block=True)}\n```" for i, e in enumerate(examples))
    return (t.replace("<<TOPICS>>", subj.labels(spec["topics"])).replace("<<DIFFICULTY>>", bank.DIFFICULTY_DESC[spec["difficulty"]])
            .replace("<<STRUCTURE>>", structure).replace("<<STRUCTURE_HELP>>", STRUCTURES[structure])
            .replace("<<NCHOICES>>", str(args.choices)).replace("<<EXTRA>>", extra)
            .replace("<<EXAMPLES>>", ex or "(none available)")
            .replace("<<ERRORS>>", ", ".join(subj.common_errors[:12]))
            .replace("<<BLOCK_SPEC>>", (HERE / "prompts" / "block_spec.md").read_text()))


def normalize(q):
    for i, a in enumerate(q["answers"]):
        a["id"] = chr(ord("a") + i)
        a["is_correct"] = bool(a.get("is_correct"))
    return q


def real_rows(res):
    """[(source line, state-after dict)] from a verify() result that ran the tracer."""
    steps, final, lines = res["_res"]["steps"], res["_res"]["final"], res["_code"].split("\n")
    rows = []
    for i, s in enumerate(steps):
        after = steps[i + 1]["before"] if i + 1 < len(steps) else final
        n = s["line"]
        if 1 <= n <= len(lines) and lines[n - 1].strip() not in ("}", ""):
            rows.append((lines[n - 1].strip(), {k: v for k, v in after.items() if k != "args"}))
    return rows


def trace_mismatches(q, res):
    """Compare the author's claimed trace with the real execution; returns a short list of disagreements."""
    claimed, real = q.get("trace") or [], real_rows(res)
    nv = verify.norm_val  # ignores spacing, quotes and {} vs [] so `{4, 1}` equals `[4, 1]`
    out, p = [], 0
    for row in claimed:
        want = nv(row.get("line", ""))
        j = next((k for k in range(p, len(real)) if nv(real[k][0]) == want or want in nv(real[k][0])), None)
        if j is None:
            out.append(f"claimed line {row.get('line')!r} was not executed at that point")
            continue
        p = j + 1
        for var, val in (row.get("state") or {}).items():
            got = real[j][1].get(var)
            if got is not None and nv(got) != nv(val):
                out.append(f"after `{row['line']}` you claim {var}={val} but the program has {var}={got}")
    return out[:4]


def check(q, ctx):
    """Run the enabled layers; returns (findings, info). Findings are specific, actionable sentences."""
    layers, findings, info = ctx["layers"], [], {}
    if "lint" in layers:
        r = review.layer_lint(q, ctx)
        if r["verdict"] == "flag":
            findings.append("Structure: " + r["detail"])
    if "comments" in layers:
        r = review.layer_comments(q, ctx)
        if r["verdict"] == "flag":
            findings.append("Comments explain the code (remove them or use a placeholder): " + "; ".join(f"`{t}`" for _, t in r["comments"][:6]))
    if "exec" in layers:
        v = q.get("verification") or {}
        res = verify.verify(q, v) if v else {"status": "skip", "reason": "no verification block"}
        st, reason = res["status"], res.get("reason", "")
        info["exec"] = st
        pc = "; ".join(f"{k}: {x}" for k, x in (res.get("per_choice") or {}).items())
        if st == "pass":
            info["verified"] = True
            if "_res" in res and q.get("trace"):
                for m in trace_mismatches(q, res):
                    findings.append("Trace disagrees with the real run: " + m)
        elif st in ("FAIL", "unmatched"):
            findings.append(f"RUNNING THE JAVA CONTRADICTS YOUR KEY ({reason}). Per-choice evidence: {pc or 'n/a'}. Fix the question so exactly the key is correct.")
        elif st == "unverifiable":
            info["verified"] = False
        elif st in ("skip", "ran"):
            findings.append(f"Your verification block did not run ({st}: {reason}). Per-choice evidence: {pc or 'n/a'}. Fix the block so it runs on the stem as shown.")
    if "solve" in layers:
        r = review.layer_solve(q, ctx)
        info["solve_cost"] = r["cost"]
        if r["verdict"] == "flag":
            findings.append("An independent solver who did not see your key disagreed: " + r["detail"] + ". Check that the key is right and that no other choice is defensible.")
    return findings, info


def revise_prompt(base, q, findings):
    shown = {k: v for k, v in q.items() if not k.startswith("_")}
    return (base + "\n\n---\n## Your previous attempt had problems\nAutomated checks that actually ran your question found:\n"
            + "\n".join(f"- {f}" for f in findings)
            + "\n\nPrevious attempt:\n```json\n" + json.dumps(shown, indent=1, ensure_ascii=False)
            + "\n```\nFix every problem without introducing new ones. Re-verify every choice, update `verification` and `trace` to match the new code, "
            "and return the complete corrected object. If a problem cannot be fixed, write a different question.")


def pipeline_question(subj, spec, structure, idx, args, ctx, rng):
    examples = subj.select_examples(spec["topics"], spec["difficulty"], args.examples, rng)
    base = author_prompt(subj, spec, structure, examples, args)
    cost, history, q, prompt = 0.0, [], None, base
    for rnd in range(args.rounds + 1):
        try:
            q, c = llm_json(prompt, args.model, AUTHOR_SCHEMA)
        except Exception as e:
            return {"id": f"gen-{idx}", "review": {"state": "unparsed", "error": str(e)[:200], "cost": cost, "rounds": rnd}}
        cost += c
        q = normalize(q)
        findings, info = check(q, ctx)
        cost += info.get("solve_cost", 0.0)
        history.append(findings)
        if not findings:
            break
        prompt = revise_prompt(base, q, findings)
    state = "flagged" if findings else ("verified" if info.get("verified") else "judged")
    q.update({"id": f"gen-{idx}", "status": "draft", "structure": q.get("structure") or structure,
              "review": {"state": state, "rounds": len(history) - 1, "open_findings": findings, "history": history, "cost": round(cost, 4)}})
    q.setdefault("subtopics", [])
    return q


def run_pipeline(spec, args, subj, seed=0, start=0):
    rng = random.Random(f"{seed}-{spec['topics']}-{spec['difficulty']}")
    structures = rng.sample(list(STRUCTURES), min(spec["count"], len(STRUCTURES)))
    ctx = {"model": args.model, "solve_n": 1, "choice_counts": {args.choices}, "adj_explanations": False, "layers": set(args.loop_layers.split(",")), "blocks": {}}
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        futs = [pool.submit(pipeline_question, subj, spec, s, start + i, args, ctx, random.Random(f"{seed}-{i}-{s}")) for i, s in enumerate(structures)]
        return [f.result() for f in futs]


# ---------------------------------------------------------------- baseline (the current design)

def run_baseline(spec, args, subj, seed=0, start=0):
    rng = random.Random(f"{seed}-{spec['topics']}-{spec['difficulty']}-base")
    examples = subj.select_examples(spec["topics"], spec["difficulty"], 3, rng)
    prompt = subj.baseline_prompt(spec["topics"], spec["difficulty"], spec["count"], examples, spec.get("notes"))
    try:
        text, cost = llm_json(prompt, args.model, None, system="You are a helpful assistant.")
    except Exception as e:
        return [{"id": f"base-{start}", "review": {"state": "unparsed", "error": str(e)[:200], "cost": 0.0}}]
    arr = mine_logs.final_array(text, repair=False)  # the app has some salvage logic; plain parse is the honest floor
    if not arr:
        return [{"id": f"base-{start}", "review": {"state": "unparsed", "error": mine_logs.classify_failure(text), "cost": cost}}]
    out = []
    for i, q in enumerate(arr):
        for k, a in enumerate(q.get("answers", [])):
            a.setdefault("id", chr(ord("a") + k))
        q.update({"id": f"base-{start + i}", "topics": spec["topics"], "difficulty": bank.DIFFICULTY_CODE[spec["difficulty"]],
                  "review": {"state": "unreviewed", "cost": round(cost / len(arr), 4)}})
        out.append(q)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--arm", choices=["pipeline", "baseline"], default="pipeline")
    ap.add_argument("--topics", required=True, help="comma list of topic/subtopic ids, e.g. T006,ST010")
    ap.add_argument("--difficulty", choices=list(bank.DIFFICULTY_CODE), default="medium")
    ap.add_argument("--count", type=int, default=3)
    ap.add_argument("--notes")
    ap.add_argument("--model", default="sonnet")
    ap.add_argument("--rounds", type=int, default=2, help="max revision rounds (pipeline)")
    ap.add_argument("--loop-layers", default="lint,comments,exec,solve", help="checks fed back to the author (pipeline)")
    ap.add_argument("--choices", type=int, default=5)
    ap.add_argument("--examples", type=int, default=3)
    ap.add_argument("-j", "--jobs", type=int, default=6)
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--out")
    args = ap.parse_args()
    spec = {"topics": args.topics.split(","), "difficulty": args.difficulty, "count": args.count, "notes": args.notes}
    subj = bank.Subject(with_blocks=args.arm == "pipeline")
    qs = (run_pipeline if args.arm == "pipeline" else run_baseline)(spec, args, subj, args.seed)
    for q in qs:
        r = q["review"]
        print(f"{q['id']:<9} {r['state']:<10} rounds={r.get('rounds', '-')} cost=${r.get('cost', 0):.3f}  "
              + (q.get("text", "")[:70].replace("\n", " ") if "text" in q else r.get("error", "")))
        for f in r.get("open_findings", []):
            print("    open:", f[:160])
    print(f"total ${sum(q['review'].get('cost', 0) for q in qs):.3f}")
    if args.out:
        json.dump(qs, open(args.out, "w"), indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main()
