#!/usr/bin/env python3
"""Layered quality review for generated AP CS A questions. Every layer is optional.

  lint        deterministic schema/hygiene checks (free)
  exec        run the Java: auto-inferred check, or a verification block from --blocks (free)
  draft       an LLM drafts a verification block from the stem + choices (key hidden), then exec runs it
  solve       blind solve: an LLM answers the question without seeing the key; --solve-n votes.
              `--layers solve --solve-n 1` is the plain "just answer the question" baseline
  adjudicate  an LLM sees the key and must name any other defensible choice / inconsistent explanation
  comments    free heuristic: stem/choice code comments that explain rather than label (leak candidates)
  leak        an LLM judges whether the stem or choices give away what the code does (comments, prose, telling names)

Verdicts per layer: ok | flag | skip. A question is flagged if any enabled layer flags it.
LLM layers call `claude -p` headlessly (model via --model) and report real cost per layer.

usage:
  review.py INPUT.json [--layers lint,exec,solve] [--blocks SIDECAR.json] [--model sonnet] [-j 8] [--out results.json]
INPUT is a bank ({"questions": [...]}) or a list of {"q": question, ...} entries (e.g. mined from llm_log_*.txt).
"""
import argparse, json, os, re, subprocess, sys
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import verify  # noqa: E402

SOLVER_SYSTEM = "You are an expert AP Computer Science A teacher answering exam questions. Be accurate and concise."
LEAK_RE = re.compile(r"//\s*(?:\w+\s+)?(?:becomes|is now|evaluates to|equals|gives|results in|now)\b", re.I)


# ---------------------------------------------------------------- LLM backend

def llm(prompt, model, system=SOLVER_SYSTEM, timeout=240):
    """One headless Claude call. Returns (text, cost_usd). Tools disabled: pure text in, text out."""
    cmd = ["claude", "-p", "--model", model, "--output-format", "json", "--tools", "",
           "--no-session-persistence", "--disable-slash-commands", "--system-prompt", system]
    for attempt in range(2):
        try:
            r = subprocess.run(cmd, input=prompt, capture_output=True, text=True, timeout=timeout)
            d = json.loads(r.stdout)
            if d.get("is_error"):
                raise RuntimeError(str(d.get("result"))[:200])
            return d["result"], float(d.get("total_cost_usd") or 0)
        except Exception as e:  # retry once on any transport/parse failure
            err = e
    raise RuntimeError(f"llm call failed: {err}")


def letters(q):
    return {a["id"]: chr(ord("A") + i) for i, a in enumerate(q["answers"])}


def render(q, with_key=False):
    L = letters(q)
    out = [q["text"].strip(), ""]
    for a in q["answers"]:
        out.append(f"{L[a['id']]}. {a['text'].strip()}" + ("   <-- marked correct" if with_key and a.get("is_correct") else ""))
    return "\n".join(out)


def keyed(q):
    return [a["id"] for a in q["answers"] if a.get("is_correct")]


# ---------------------------------------------------------------- layers

def layer_lint(q, ctx):
    problems = []
    n = len(q["answers"])
    if n not in ctx["choice_counts"]:
        problems.append(f"{n} choices (expected one of {sorted(ctx['choice_counts'])})")
    if len(keyed(q)) != 1:
        problems.append(f"{len(keyed(q))} choices marked correct")
    # collapse spaces/tabs but keep line breaks: choices that are file layouts differ only in where the lines break
    texts = ["\n".join(re.sub(r"[ \t]+", " ", ln).strip() for ln in a["text"].strip().split("\n")).lower() for a in q["answers"]]
    if len(set(texts)) != len(texts):
        problems.append("duplicate choice text")
    if any(not (a.get("explanation") or "").strip() for a in q["answers"]):
        problems.append("a choice has no explanation")
    if not (q.get("explanation") or "").strip():
        problems.append("no question-level explanation")
    for block in verify.java_blocks(q["text"]):
        if LEAK_RE.search(block):
            problems.append("stem code comment may leak the solution")
    if q["text"].count("```") % 2:
        problems.append("unbalanced code fence in stem")
    return {"verdict": "flag" if problems else "ok", "detail": "; ".join(problems), "cost": 0.0}


def layer_exec(q, ctx):
    r = verify.verify(q, ctx["blocks"].get(q.get("id")))
    st = r["status"]
    verdict = {"pass": "ok", "FAIL": "flag", "unmatched": "flag"}.get(st, "skip")
    return {"verdict": verdict, "detail": f"{st}: {r.get('reason', '')}".strip(": "), "cost": 0.0, "exec_status": st}


DRAFT_PROMPT = """You are writing a machine-checkable `verification` block for an AP CS A multiple-choice question so a harness can run the Java and
find which choice the code supports. You are NOT told the answer; do not guess it, write the harness faithfully from the stem and choices.

Choose ONE kind and output ONLY a JSON object (no prose). Kinds (all string fields are Java/source text):
- {{"kind":"run","members":"<static methods the stem refers to but does not show>","prelude":"<stub classes>","before":"<declarations>","after":"<statements>","count_lines":true|false,"claims":{{"a":"<python expr over out (stdout str) and lines (int)>"}}}}
    runs the stem; compares printed output / a variable named in the question / claims to the choices.
- {{"kind":"state"|"run","observe":"<variable>"}} compares that variable's final value with the choices.
- {{"kind":"compile","expect":"compiles"|"fails","prelude":"<stub classes>","before":"<declarations>","template":"int x = {{}};"}}  each choice is a statement (template wraps it, {{}} = choice).
- {{"kind":"substitute","expect_output":"<intended stdout>","cases":[{{"before":"","after":"","expect_output":""}}],"expect":"passes"|"fails","prelude":"","before":""}}  each choice replaces /* missing code */.
- {{"kind":"patch","variants":{{"a":[["old text","new text"]]}},"expect":"compiles" | "expect_output":"..."}}  each choice describes a change to the stem code; give the edit per choice.
- {{"kind":"witness","cases":{{"a":{{"before":"","after":"System.out.println(...);","expect_output":"<INTENDED correct result>"}}}}}}  each choice names an input condition; the keyed choice is where the code misbehaves.
- {{"kind":"oracle","loops":"for (...) for (...)","subject":"<java expr: the stem's result>","show":"<expr>","choices":{{"a":"<java expr equal to subject iff this choice is true>"}},"fragment":"<optional override>","prelude":""}}  prose choices tested over an input grid.
- {{"kind":"equivalent","after":"System.out.println(x);"}} each choice is a code segment that must leave the same result as the stem.
- {{"kind":"none","reason":"..."}}  if nothing here is executable (purely conceptual).
Rules: choice ids are a, b, c, ... in the order listed. Prefer the simplest kind that works.

QUESTION:
{question}
"""


def layer_draft(q, ctx):
    view = {"answers": [{"id": a["id"], "text": a["text"]} for a in q["answers"]], "text": q["text"]}
    text, cost = llm(DRAFT_PROMPT.format(question=render(view)), ctx["model"])
    m = re.search(r"\{.*\}", text, re.S)
    try:
        block = json.loads(m.group(0))
    except Exception:
        return {"verdict": "skip", "detail": "draft: model did not return valid JSON", "cost": cost}
    r = verify.verify(q, block)
    st = r["status"]
    verdict = {"pass": "ok", "FAIL": "flag"}.get(st, "skip")
    if st == "FAIL" and re.search(r": \[\], key is", r.get("reason", "")):
        # no choice satisfied the drafted harness, not even the key: the harness is the likely culprit, not the question
        verdict, st = "skip", "harness rejected every choice (draft unreliable)"
    # an unmatched/FAIL after an LLM-written harness is a lead, not proof: the harness itself may be wrong
    return {"verdict": verdict, "detail": f"draft/{block.get('kind')}: {st} {r.get('reason', '')}".strip(), "cost": cost, "block": block}


def layer_solve(q, ctx):
    L = letters(q)
    inv = {v: k for k, v in L.items()}
    votes, cost = [], 0.0
    for _ in range(ctx["solve_n"]):
        text, c = llm("Answer this question. Work it out, then finish with a final line exactly of the form `ANSWER: <letter>`.\n\n" + render(q),
                      ctx["model"])
        cost += c
        m = re.findall(r"ANSWER:\s*\**\(?([A-Z])\b", text)
        votes.append(inv.get(m[-1]) if m else None)
    top, n = Counter(votes).most_common(1)[0]
    key = keyed(q)[0] if keyed(q) else None
    agree = top == key and n > ctx["solve_n"] / 2
    return {"verdict": "ok" if agree else "flag",
            "detail": "" if agree else f"solver(s) answered {[L.get(v, v) for v in votes]}, key is {L.get(key)}", "cost": cost, "votes": votes}


ADJ_PROMPT = """Review this AP CS A multiple-choice question. The choice marked correct is the author's key.
Judge it strictly and answer ONLY with JSON:
{{"key_correct": true|false,
  "other_defensible": ["<letters of any non-key choice that a knowledgeable expert could defend as correct, or []>"],
  "explanation_problems": ["<choice letter: what its explanation gets wrong, or []>"],
  "notes": "<one sentence>"}}

{question}

Explanations given by the author:
{explanations}
"""


def layer_adjudicate(q, ctx):
    L = letters(q)
    ex = "\n".join(f"{L[a['id']]}: {a.get('explanation', '')}" for a in q["answers"]) + "\nOverall: " + (q.get("explanation") or "")
    text, cost = llm(ADJ_PROMPT.format(question=render(q, with_key=True), explanations=ex), ctx["model"])
    try:
        d = json.loads(re.search(r"\{.*\}", text, re.S).group(0))
    except Exception:
        return {"verdict": "skip", "detail": "adjudicate: unparseable reply", "cost": cost}
    probs = []
    if d.get("key_correct") is False:
        probs.append("judge says key is wrong")
    if d.get("other_defensible"):
        probs.append(f"also defensible: {d['other_defensible']}")
    if d.get("explanation_problems") and ctx["adj_explanations"]:  # off by default: flagged ~54% of real questions, mostly nitpicks
        probs.append(f"explanations: {d['explanation_problems']}")
    return {"verdict": "flag" if probs else "ok", "detail": "; ".join(probs) + (f" [{d.get('notes', '')}]" if probs else ""), "cost": cost}


PLACEHOLDER_RE = re.compile(
    r"^(?:line\s*\d+|code (?:to be inserted|executed in main)|method body|implementation|todo|missing \w+|"
    r"initialization not shown|implementation not shown|some .{0,30}|your code here|\.\.\.|end of .{0,30})$", re.I)
COMMENT_RE = re.compile(r"//(.*)$|/\*(.*?)\*/")


def code_comments(q):
    """(where, text) for every comment in stem and choice code."""
    found = []
    sources = [("stem", b) for b in verify.java_blocks(q["text"])] + [("choice", a["text"]) for a in q["answers"]]
    for where, src in sources:
        for line in src.split("\n"):
            line = re.sub(r'"(?:[^"\\]|\\.)*"', '""', line)  # blank out string literals so "http://x" is not a comment
            for m in COMMENT_RE.finditer(line):
                text = (m.group(1) or m.group(2) or "").strip()
                if text:
                    found.append((where, text))
    return found


def layer_comments(q, ctx):
    explanatory = [(w, t) for w, t in code_comments(q)
                   if not PLACEHOLDER_RE.match(re.sub(r"[.:]+$", "", t)) and len(t.split()) >= 2]
    if not explanatory:
        return {"verdict": "ok", "detail": "", "cost": 0.0}
    return {"verdict": "flag", "cost": 0.0, "comments": explanatory,
            "detail": f"{len(explanatory)} explanatory comment(s): " + " | ".join(f"{t}" for _, t in explanatory[:3])}


LEAK_PROMPT = """You are auditing an AP Computer Science A multiple-choice question for LEAKAGE: text that does the student's thinking for them.

Flag each of these, quoting the exact text:
 1. COMMENT in the stem or choice code that states what a statement does, computes, or its value/intermediate result/final result
    (e.g. `// integer division`, `// x becomes 7`, `// counter has wrapped negative`, step-by-step narration of an algorithm).
 2. PROSE in the stem that states what the code does, prints, or returns when the question asks the student to work that out
    (e.g. "this method reverses the array" on a question asking what the method does).
 3. NAME that spells out the answer (e.g. a method called `isSortedAscending` on a question asking whether it sorts).

Do NOT flag:
 - placeholders and labels: `/* missing code */`, `// code to be inserted`, `// Line 3`, `/* TODO */`, `// method body`.
 - a statement of INTENDED behavior in a "what is wrong / which change fixes it / which code accomplishes this" question
   (the intent is the spec, the student must find the defect). A spec of a method the student cannot see is also fine.
 - a comment that only supplies necessary BACKGROUND the student could not derive (e.g. `// ASCII value 65`, a unit, a constant's meaning),
   as opposed to a comment that performs part of the computation (`// integer division`, `// x becomes 7`).
 - ordinary problem-setup prose (class descriptions, preconditions, scenario).

Answer ONLY with JSON:
{{"verdict":"clean"|"leak","leaks":[{{"where":"comment"|"prose"|"name","quote":"<exact text>","why":"<what it gives away>"}}]}}

QUESTION:
{question}
"""


def layer_leak(q, ctx):
    text, cost = llm(LEAK_PROMPT.format(question=render(q)), ctx["model"])
    try:
        d = json.loads(re.search(r"\{.*\}", text, re.S).group(0))
    except Exception:
        return {"verdict": "skip", "detail": "leak: unparseable reply", "cost": cost}
    leaks = d.get("leaks") or []
    return {"verdict": "flag" if leaks else "ok", "cost": cost, "leaks": leaks,
            "detail": "; ".join(f"[{l.get('where')}] {str(l.get('quote'))[:70]!r} - {l.get('why')}" for l in leaks)}


LAYERS = {"lint": layer_lint, "comments": layer_comments, "leak": layer_leak, "exec": layer_exec, "draft": layer_draft, "solve": layer_solve, "adjudicate": layer_adjudicate}
FREE = {"lint", "comments", "exec"}


# ---------------------------------------------------------------- driver

def load(path):
    d = json.load(open(path))
    if isinstance(d, dict):
        return d["questions"]
    out = []
    for i, e in enumerate(d):
        q = dict(e["q"])
        q.setdefault("id", f"{os.path.basename(e.get('log', 'x'))[8:16]}-{e.get('ts', '')[11:19]}-{i}")
        for k, a in enumerate(q["answers"]):
            a.setdefault("id", "abcdefgh"[k])
        out.append(q)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("input")
    ap.add_argument("--layers", default="lint,exec", help="comma list of: " + ",".join(LAYERS))
    ap.add_argument("--blocks")
    ap.add_argument("--model", default="sonnet")
    ap.add_argument("--solve-n", type=int, default=1)
    ap.add_argument("--choice-counts", default="4,5")
    ap.add_argument("--adj-explanations", action="store_true", help="let adjudicate also flag explanation problems (noisy)")
    ap.add_argument("-j", "--jobs", type=int, default=8)
    ap.add_argument("--limit", type=int)
    ap.add_argument("--out")
    a = ap.parse_args()
    layers = [l for l in a.layers.split(",") if l]
    unknown = [l for l in layers if l not in LAYERS]
    if unknown:
        sys.exit(f"unknown layer(s): {unknown}")
    qs = load(a.input)[: a.limit]
    ctx = {"model": a.model, "solve_n": a.solve_n, "adj_explanations": a.adj_explanations, "choice_counts": {int(x) for x in a.choice_counts.split(",")},
           "blocks": json.load(open(a.blocks)) if a.blocks else {}}

    def review(q):
        res = {}
        for name in layers:
            try:
                res[name] = LAYERS[name](q, ctx)
            except Exception as e:
                res[name] = {"verdict": "skip", "detail": f"layer error: {e}", "cost": 0.0}
        return {"id": q["id"], "flagged_by": [n for n, r in res.items() if r["verdict"] == "flag"], "layers": res}

    with ThreadPoolExecutor(max_workers=max(1, a.jobs)) as pool:
        results = list(pool.map(review, qs))

    print(f"{len(qs)} questions, layers: {', '.join(layers)}\n")
    print(f"{'layer':<11}{'ok':>6}{'flag':>6}{'skip':>6}{'cost $':>9}")
    for n in layers:
        c = Counter(r["layers"][n]["verdict"] for r in results)
        print(f"{n:<11}{c['ok']:>6}{c['flag']:>6}{c['skip']:>6}{sum(r['layers'][n]['cost'] for r in results):>9.2f}")
    print(f"{'total':<11}{'':>18}{sum(l['cost'] for r in results for l in r['layers'].values()):>9.2f}")
    flagged = [r for r in results if r["flagged_by"]]
    print(f"\nflagged: {len(flagged)} of {len(results)}")
    for r in flagged:
        for n in r["flagged_by"]:
            print(f"  {r['id']}  [{n}] {r['layers'][n]['detail'][:150]}")
    if a.out:
        json.dump(results, open(a.out, "w"), indent=1)


if __name__ == "__main__":
    main()
