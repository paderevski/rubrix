#!/usr/bin/env python3
"""Distractor quality check for AP CS A multiple-choice questions.

Two independent stages per question:

  judge   (LLM)   per distractor: is the stated misconception a concrete student error, would that error produce
                  exactly this choice, would a partly prepared student pick it, does the choice text narrate its own
                  flaw. Per question: how many distinct error kinds the distractors cover.
  mutate  (LLM+Java)  for "what is printed" stems that run: an LLM proposes code edits that implement each
                  distractor's stated misconception; the edited program is run and its output compared with the
                  distractor. A match is evidence the distractor is derivable from a specific error, and it does not
                  depend on a judge's opinion. A mismatch is weak evidence only (the proposed edit may be wrong).

Usage:
  distractors.py LABEL=FILE [LABEL=FILE ...] [--n 20] [--seed 1] [--model sonnet] [-j 8] [--out res.json]
FILE: a questions JSON ({"questions": [...]}, {"baseline": [...]}, a list of questions, or llm-log mined entries).
"""
import argparse, json, os, random, re, sys
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import verify  # noqa: E402
import review  # noqa: E402

JUDGE_SYSTEM = "You are an experienced AP Computer Science A teacher who writes and reviews multiple-choice exam items. Be exact and critical."

JUDGE_PROMPT = """Review the DISTRACTORS (wrong choices) of this AP CS A multiple-choice question. The correct choice is marked.

For each distractor give:
- "error": the specific student error or misconception it represents, in a few words ("" if none can be named)
- "grounded": 2 = the error is concrete and following it from the stem really yields exactly this choice;
              1 = plausible but vague, or you cannot confirm it yields exactly this choice;
              0 = no identifiable error (throwaway value or statement)
- "plausible": 2 = a partly prepared student would readily pick it; 1 = only a careless or very weak student; 0 = nobody would
- "strong": true if a student who traces almost all of the stem correctly and slips once at a subtle point would pick it
            (a near-miss that needs the full trace to rule out); false if only a clearly weaker student would
- "narrates": true if the choice TEXT itself (including comments inside it) explains why it is wrong or what it does

Also give "kinds": how many DISTINCT error types the distractors cover (two distractors that are the same mistake count once),
and "note": one short sentence on the weakest distractor, or "" if all are good.
Judge from the stem and the choices; do not simply trust the author's explanations.

Output ONLY JSON: {{"distractors":{{"<letter>":{{"error":"","grounded":0,"plausible":0,"strong":false,"narrates":false}}}},"kinds":0,"note":""}}

QUESTION:
{question}

AUTHOR EXPLANATIONS OF THE DISTRACTORS:
{explanations}
"""

MUTATE_PROMPT = """Below is a Java program (the code from an AP CS A question) and, for each wrong answer choice, the student
misconception the author says leads to it. For each wrong choice, give the code edits that make the program do exactly what
a student with that misconception would compute, so that running the edited program prints that choice's value.

Rules:
- Edit the real logic (an operator, a bound, an initial value, a removed or added statement). Never hard-code the answer
  and never add a println of a constant.
- Each edit is ["old text", "new text"]; "old text" must appear EXACTLY ONCE in the program.
- If the misconception is not expressible as an edit (mental arithmetic slip, misreading output format), use null.
- Keep the program compiling.

Output ONLY JSON: {{"<letter>": [["old","new"], ...] or null, ...}}

PROGRAM:
```java
{source}
```

WRONG CHOICES:
{choices}
"""


# ---------------------------------------------------------------- helpers
def jparse(text):
    m = re.search(r"\{.*\}", text, re.S)
    if not m:
        raise ValueError("no JSON in reply")
    return json.loads(m.group(0))


def load_questions(path):
    d = json.load(open(path))
    if isinstance(d, dict):
        d = d.get("questions") or next(v for v in d.values() if isinstance(v, list))
    out = []
    for i, e in enumerate(d):
        q = dict(e["q"]) if "q" in e and "answers" not in e else dict(e)
        q.setdefault("id", f"q{i}")
        for k, a in enumerate(q["answers"]):
            a.setdefault("id", "abcdefgh"[k])
        out.append(q)
    return out


def explanations(q):
    L = review.letters(q)
    return "\n".join(f"{L[a['id']]}: {a.get('explanation', '').strip() or '(none)'}" for a in q["answers"] if not a.get("is_correct"))


# ---------------------------------------------------------------- stages
def judge(q, model):
    text, cost = review.llm(JUDGE_PROMPT.format(question=review.render(q, with_key=True), explanations=explanations(q)),
                            model, system=JUDGE_SYSTEM)
    d = jparse(text)
    L = review.letters(q)
    wrong = {L[a["id"]] for a in q["answers"] if not a.get("is_correct")}
    rows = {k.upper(): v for k, v in d.get("distractors", {}).items() if k.upper() in wrong}
    return {"rows": rows, "kinds": d.get("kinds"), "note": d.get("note", ""), "cost": cost}


def apply_edits(source, edits):
    for old, new in edits:
        if source.count(old) != 1:
            return None
        source = source.replace(old, new)
    return source


def mutate(q, model):
    """Returns None if not applicable, else {"letter": status}; statuses: reproduced | diverged | no_edit | bad_edit | error."""
    if not verify.asks_output(q["text"]):
        return None
    blocks = verify.java_blocks(q["text"])
    source = verify.compose(blocks) if blocks else None
    if not source:
        return None
    base = verify.run_plain(source)
    if "stdout" not in base:
        return None
    key = next((a for a in q["answers"] if a.get("is_correct")), None)
    if not key or verify.norm_val(base["stdout"]) != verify.norm_val(verify.ans_exact(key)) \
            and verify.norm_val(base["stdout"]) != verify.norm_val(verify.ans_code(key)):
        return None  # only mutate when the baseline program demonstrably prints the key
    L = review.letters(q)
    wrong = [a for a in q["answers"] if not a.get("is_correct")]
    choices = "\n".join(f"{L[a['id']]}. {verify.ans_code(a)}   <- misconception: {a.get('explanation', '').strip()[:300]}" for a in wrong)
    text, cost = review.llm(MUTATE_PROMPT.format(source=source, choices=choices), model, system=JUDGE_SYSTEM)
    edits = jparse(text)
    res = {}
    for a in wrong:
        k = L[a["id"]]
        e = edits.get(k) or edits.get(k.lower())
        if not e:
            res[k] = "no_edit"
            continue
        if any(re.search(r"println\s*\(\s*(?:\"[^\"]*\"|\d+(?:\.\d+)?)\s*\)", new) for _, new in e):
            res[k] = "bad_edit"  # printing a constant is not a derivation
            continue
        mutated = apply_edits(source, e)
        if mutated is None:
            res[k] = "bad_edit"
            continue
        out = verify.run_plain(mutated)
        if "stdout" not in out:
            res[k] = "error"
            continue
        want = {verify.norm_val(verify.ans_exact(a)), verify.norm_val(verify.ans_code(a))}
        res[k] = "reproduced" if verify.norm_val(out["stdout"]) in want else "diverged"
    return {"status": res, "cost": cost}


def run_one(item, model):
    label, q = item
    r = {"label": label, "id": q["id"], "topics": q.get("topics")}
    for name, fn in (("judge", judge), ("mutate", mutate)):
        try:
            r[name] = fn(q, model)
        except Exception as e:
            r[name] = {"error": str(e)}
    return r


# ---------------------------------------------------------------- report
def summarize(results, labels):
    lines = []
    for lab in labels:
        rs = [r for r in results if r["label"] == lab]
        rows = [v for r in rs if "rows" in r["judge"] for v in r["judge"]["rows"].values()]
        n = len(rows)
        if not n:
            continue
        g = [v.get("grounded", 0) for v in rows]
        p = [v.get("plausible", 0) for v in rows]
        kinds = [r["judge"]["kinds"] for r in rs if r["judge"].get("kinds") is not None]
        nq = len(kinds)
        mut = [s for r in rs if r["mutate"] and "status" in r["mutate"] for s in r["mutate"]["status"].values()]
        lines.append(f"\n== {lab}: {len(rs)} questions, {n} distractors")
        lines.append(f"  grounded 2 (concrete, yields this choice) {sum(x == 2 for x in g):>3} ({100 * sum(x == 2 for x in g) // n}%)   "
                     f"1 (vague) {sum(x == 1 for x in g)}   0 (throwaway) {sum(x == 0 for x in g)} ({100 * sum(x == 0 for x in g) // n}%)")
        lines.append(f"  plausible 2 {sum(x == 2 for x in p)} ({100 * sum(x == 2 for x in p) // n}%)   1 {sum(x == 1 for x in p)}   0 {sum(x == 0 for x in p)}")
        lines.append(f"  near-miss traps for strong students: {sum(bool(v.get('strong')) for v in rows)} ({100 * sum(bool(v.get('strong')) for v in rows) // n}%); "
                     f"questions with at least one: {sum(1 for r in rs if any(v.get('strong') for v in r['judge'].get('rows', {}).values()))}/{len(rs)}")
        lines.append(f"  choice text narrates its own flaw: {sum(bool(v.get('narrates')) for v in rows)} ({100 * sum(bool(v.get('narrates')) for v in rows) // n}%)")
        lines.append(f"  distinct error kinds per question: avg {sum(kinds) / nq:.1f}; questions where distractors repeat an error type: "
                     f"{sum(1 for r in rs if r['judge'].get('kinds') is not None and r['judge']['kinds'] < len(r['judge'].get('rows', {})))}/{nq}")
        if mut:
            rep = sum(s == "reproduced" for s in mut)
            lines.append(f"  mutate-and-run: {len(mut)} distractors on {sum(1 for r in rs if r['mutate'] and 'status' in r['mutate'])} runnable output questions -> "
                         f"reproduced {rep} ({100 * rep // len(mut)}%), diverged {sum(s == 'diverged' for s in mut)}, "
                         f"no edit {sum(s == 'no_edit' for s in mut)}, bad/err {sum(s in ('bad_edit', 'error') for s in mut)}")
        lines.append(f"  cost ${sum(r['judge'].get('cost', 0) + (r['mutate'] or {}).get('cost', 0) for r in rs):.2f}")
    return "\n".join(lines)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("sources", nargs="+", help="LABEL=FILE")
    ap.add_argument("--n", type=int, default=20, help="questions sampled per source")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--model", default="sonnet")
    ap.add_argument("-j", "--jobs", type=int, default=8)
    ap.add_argument("--out")
    a = ap.parse_args()
    rng = random.Random(a.seed)
    items, labels = [], []
    for s in a.sources:
        lab, path = s.split("=", 1)
        qs = load_questions(path)
        qs = [q for q in qs if len(q["answers"]) >= 3]
        rng.shuffle(qs)
        items += [(lab, q) for q in qs[: a.n]]
        labels.append(lab)
    with ThreadPoolExecutor(a.jobs) as ex:
        results = list(ex.map(lambda it: run_one(it, a.model), items))
    print(summarize(results, labels))
    errs = sum(1 for r in results for k in ("judge", "mutate") if r[k] and "error" in r[k])
    if errs:
        print(f"\n({errs} stage errors; see --out)")
    if a.out:
        json.dump({"results": results, "questions": {q["id"]: q for _, q in items}}, open(a.out, "w"), indent=1)


if __name__ == "__main__":
    main()
