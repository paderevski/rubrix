#!/usr/bin/env python3
"""Verify Java-based question bank entries by actually running the stem code.

Two ways a question gets verified:
  * auto   - rules infer the check from the stem and choices alone (tier 1)
  * block  - a `verification` block supplies what the stem leaves out: stub
             classes, off-screen declarations, input cases (tier 2). Blocks
             live in a sidecar JSON ({question_id: block}) passed with --blocks,
             or in the question's own `verification` field.

Block kinds (see verify_block docs below for fields):
  run / state  run the composed stem code; compare stdout (or a variable) to the choices
  compile      each choice is a statement (or a member, with "into"); compile it
  substitute   fill /* missing code */ with each choice; run it on cases
  witness      per-choice input cases; the keyed choice is the one where the code is wrong
  oracle       one program loops over inputs and compares the stem's result with a
               Java expression per choice (for choices that are prose descriptions)

A FAIL always means "the key is not the unique choice the code supports".
Anything the verifier cannot decide is a skip with a reason, never a FAIL.

usage:
  verify.py BANK.json [BANK2.json ...] [--blocks FILE]
  verify.py BANK.json --trace ID [--blocks FILE]
"""
import argparse, functools, json, os, re, shutil, subprocess, tempfile
from collections import Counter
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
TRACER = os.path.join(HERE, "Tracer.java")
TIMEOUT = 20      # compiling
RUN_TIMEOUT = 5   # running a stem; a student-sized program that takes longer is looping forever

CLASS_RE = re.compile(r"^\s*(?:(?:public|abstract|final)\s+)*(?:class|interface|enum)\s+\w+", re.M)
METHOD_RE = re.compile(r"^\s*(?:public|private|protected)\s+(?:static\s+)?[\w<>\[\]]+\s+\w+\s*\(")
MAIN_RE = re.compile(r"\bpublic\s+static\s+void\s+main\b")
HOLE_RE = re.compile(r"/\*\s*missing code\s*\*/")


# ---------------------------------------------------------------- source handling

def java_blocks(text):
    return re.findall(r"```java\n(.*?)```", text, re.S)


def classify(block):
    if MAIN_RE.search(block):
        return "program"
    if CLASS_RE.search(block):
        return "class"
    first = next((l for l in block.split("\n") if l.strip()), "")
    return "members" if METHOD_RE.match(first) else "stmts"


def unpublic(b):
    return re.sub(r"^(\s*)public\s+(?=class|interface|enum)", r"\1", b, flags=re.M)


IMPORTS = "import java.util.*;\nimport java.io.*;\n"


def compose(blocks, extra_stmts="", before="", prelude="", members="", skip_stmts=False):
    """Assemble stem code blocks into one runnable Main.java, or None if there is nothing to run.

    classes -> top-level (non-public); bare methods -> static members of Main;
    statement fragments -> body of main in stem order. `before` runs ahead of the
    fragments (off-screen declarations), `extra_stmts` after them, `prelude` is
    extra top-level source (stub classes, helpers), `members` extra static members of Main.
    """
    kinds = [(classify(b), b) for b in blocks]
    if any(k == "program" for k, _ in kinds):
        return IMPORTS + next(b for k, b in kinds if k == "program")
    classes = [unpublic(b) for k, b in kinds if k == "class"]
    mem = [members] if members else []
    for k, b in kinds:
        if k == "members":
            mem.append("\n".join(
                re.sub(r"^(\s*)(public|private|protected)\s+", r"\1\2 static ", l, count=1)
                if METHOD_RE.match(l) and " static " not in l else l
                for l in b.split("\n")))
    stmts = [] if skip_stmts else [b for k, b in kinds if k == "stmts"]
    stmts = ([before] if before else []) + stmts + ([extra_stmts] if extra_stmts else [])
    if not stmts:
        return None
    return (IMPORTS + "\n".join(classes) + "\n" + unpublic(prelude) + "\npublic class Main {\n" + "\n".join(mem)
            + "\n    public static void main(String[] args) throws Exception {\n" + "\n".join(stmts) + "\n    }\n}\n")


def classes_only(blocks, prelude=""):
    return IMPORTS + "\n".join(unpublic(b) for b in blocks if classify(b) == "class") + "\n" + unpublic(prelude) + "\n"


def _first_error(stderr):
    first = next((l for l in stderr.splitlines() if "error:" in l), stderr.strip()[:120])
    return re.sub(r"^.*?error:\s*", "", first)


def compile_ok(source):
    d = tempfile.mkdtemp(prefix="verify_")
    try:
        with open(os.path.join(d, "Main.java"), "w") as f:
            f.write(source)
        c = subprocess.run(["javac", "-Xlint:none", "Main.java"], cwd=d, capture_output=True, text=True, timeout=TIMEOUT)
        return (True, "") if c.returncode == 0 else (False, _first_error(c.stderr))
    finally:
        shutil.rmtree(d, ignore_errors=True)


def run_plain(source, files=None):
    """Compile and run (in a temp dir holding `files`); returns {"stdout"} or {"compile_error"} / {"run_error"}."""
    d = tempfile.mkdtemp(prefix="verify_")
    try:
        with open(os.path.join(d, "Main.java"), "w") as f:
            f.write(source)
        for name, content in (files or {}).items():
            with open(os.path.join(d, name), "w") as f:
                f.write(content)
        c = subprocess.run(["javac", "-Xlint:none", "Main.java"], cwd=d, capture_output=True, text=True, timeout=TIMEOUT)
        if c.returncode != 0:
            return {"compile_error": _first_error(c.stderr)}
        r = subprocess.run(["java", "-cp", d, "Main"], cwd=d, capture_output=True, text=True, timeout=RUN_TIMEOUT)
        if r.returncode != 0:
            return {"run_error": (r.stderr.strip().splitlines() or ["nonzero exit"])[0][:120], "stdout": r.stdout}
        return {"stdout": r.stdout}
    except subprocess.TimeoutExpired:
        return {"run_error": "timeout"}
    finally:
        shutil.rmtree(d, ignore_errors=True)


def run_tracer(source):
    """Compile with -g and run under the JDI Tracer: stdout plus a line-by-line trace."""
    d = tempfile.mkdtemp(prefix="verify_")
    try:
        with open(os.path.join(d, "Main.java"), "w") as f:
            f.write(source)
        c = subprocess.run(["javac", "-g", "-Xlint:none", "Main.java"], cwd=d, capture_output=True, text=True, timeout=TIMEOUT)
        if c.returncode != 0:
            return {"compile_error": _first_error(c.stderr)}
        r = subprocess.run(["java", "--add-modules", "jdk.jdi", TRACER, d, "Main", "2000"],
                           capture_output=True, text=True, timeout=RUN_TIMEOUT)
        if r.returncode != 0 or not r.stdout.strip():
            return {"run_error": (r.stderr or "tracer produced no output").strip()[:200]}
        return json.loads(r.stdout)
    except subprocess.TimeoutExpired:
        return {"run_error": "timeout"}
    finally:
        shutil.rmtree(d, ignore_errors=True)


# ---------------------------------------------------------------- text helpers

def norm(s):
    """Normalize an answer or stdout for comparison; 'X followed by Y' means separate lines."""
    s = re.sub(r"`\s*followed by\s*`", " ", s)
    return re.sub(r"\s+", " ", s.replace("`", "").strip())


def norm_val(s):
    """Loose value form so {0, 1} == [0, 1], "ab" == ab, spacing ignored."""
    return re.sub(r"[\s`\"\[\]{}()]", "", s)


def ans_exact(a):
    """Choice text with only the markdown backticks removed (inner/leading spaces kept)."""
    t = a["text"].strip()
    return t[1:-1] if len(t) >= 2 and t[0] == t[-1] == "`" else t


def ans_code(a):
    t = re.sub(r"```(?:java)?", "", a["text"]).strip()
    return t.strip("`").strip()


def prose_lines(text):
    return [l for l in text.strip().split("\n") if l.strip() and not l.startswith(("```", "|"))]


def asks_output(text):
    return bool(re.search(r"what is (?:printed|displayed|output)|printed as a result|output of", text, re.I))


def render_trace(result, code):
    lines = code.split("\n")
    steps = result["steps"]
    rows = []
    for i, s in enumerate(steps):
        after = steps[i + 1]["before"] if i + 1 < len(steps) else result["final"]
        after = {k: v for k, v in after.items() if k != "args"}
        n = s["line"]
        if not 1 <= n <= len(lines):
            continue
        src = lines[n - 1].strip()
        if src in ("}", ""):
            continue
        rows.append((n, src, ", ".join(f"{k}={v}" for k, v in after.items())))
    return rows


# ---------------------------------------------------------------- executors
# Each returns a result dict with status pass | FAIL | skip | ran | nondeterministic.

def keyed_ids(q):
    return [a["id"] for a in q["answers"] if a.get("is_correct")]


def decide(out, hits, keyed, what):
    """pass iff the choices satisfying the stem are exactly the keyed ones."""
    if hits == keyed:
        out["status"] = "pass"
    else:
        out["status"], out["reason"] = "FAIL", f"choices that {what}: {hits}, key is {keyed}"
    return out


def patched(blocks, v, patches):
    """Apply [[old, new], ...] edits to the stem blocks and to the block's own source fields."""
    texts = {"blocks": list(blocks), "prelude": v.get("prelude", ""), "members": v.get("members", ""), "before": v.get("before", "")}
    for old, new in patches:
        hit = False
        for k, val in texts.items():
            if k == "blocks":
                nb = [x.replace(old, new) for x in val]
                hit |= nb != val
                texts[k] = nb
            else:
                if old in val:
                    hit = True
                    texts[k] = val.replace(old, new)
        if not hit:
            raise ValueError(f"patch target not found: {old!r}")
    return texts["blocks"], {**v, "prelude": texts["prelude"], "members": texts["members"], "before": texts["before"]}


def build(blocks, v, before="", after=""):
    return compose(blocks, after, (v.get("before", "") + "\n" + before).strip(), v.get("prelude", ""), v.get("members", ""))


def run_case(blocks, v, case):
    """Run the composed program for one case; returns (normalized stdout | None, error | None)."""
    src = build(blocks, v, case.get("before", ""), case.get("after", ""))
    if src is None:
        return None, "nothing to run"
    res = run_plain(src, v.get("files"))
    if "compile_error" in res:
        return None, "compile error: " + res["compile_error"]
    if "run_error" in res:
        return None, "runtime error: " + res["run_error"]
    return norm(res["stdout"]), None


def eval_claims(claims, env):
    return [cid for cid, expr in claims.items() if eval(expr, {"__builtins__": {}}, env)]


def ex_run(q, blocks, v, ask):
    """Run the stem. Compare stdout, a variable's final value, a line count, or per-choice `claims` to the choices."""
    call = re.search(r"\bcall\s+(\w+\([^)]*\))", q["text"])
    if not call:  # a backticked statement in the prose, e.g. "After the statement `sort2D(data);` executes"
        prose_only = re.sub(r"```.*?```", "", q["text"], flags=re.S)
        defined = set(re.findall(r"\b(\w+)\s*\([^)]*\)\s*\{", "\n".join(blocks)))
        call = next((m for m in re.finditer(r"`(\w+)\(([^`]*)\)\s*;?`", prose_only) if m.group(1) in defined), None)
        if call:
            call = re.match(r"(.*)", call.group(0).strip("`").rstrip(";"))
    after_stmt = v.get("after") or ((call.group(1) + ";") if call else "")
    code = build(blocks, v, "", after_stmt)
    if code is None:
        return {"status": "skip", "reason": "class-only stem (needs harness)"}
    res = run_tracer(code)
    if "compile_error" in res:
        return {"status": "skip", "reason": "needs harness: " + res["compile_error"]}
    if "run_error" in res:
        return {"status": "skip", "reason": "run error: " + res["run_error"]}
    if res["stderr"].strip():
        return {"status": "skip", "reason": "stem throws at runtime: " + res["stderr"].strip().splitlines()[0][:100]}
    out = {"kind": "run", "stdout": res["stdout"], "steps": len(res["steps"]), "_res": res, "_code": code}
    if re.search(r"Math\.random|new Random", code):
        out["status"], out["reason"] = "nondeterministic", "stem uses randomness; answers not auto-checked"
        return out
    keyed = keyed_ids(q)
    stdout = res["stdout"]
    lines = len(stdout.rstrip("\n").split("\n")) if stdout.strip() else 0
    if v.get("claims"):
        out["kind"] = "claims"
        hits = eval_claims(v["claims"], {"out": stdout, "lines": lines})
        return decide(out, hits, keyed, "are true of the run")
    observe = v.get("observe")
    if v.get("count_lines"):
        real, hits, shown = str(lines), [a["id"] for a in q["answers"] if norm_val(ans_code(a)) == str(lines)], f"{lines} lines printed"
    else:
        if observe is None and not v.get("compare") and not asks_output(q["text"]):
            prose = " ".join(prose_lines(re.sub(r"```.*?```", "", q["text"], flags=re.S))).replace("*", "")
            m = re.search(r"(?:contents|values?) of (?:the )?(?:array |variable |list )?`?([A-Za-z_]\w*(?:\[[^\]]*\])*)`?", prose)
            observe = m.group(1) if m else None
        if observe is None and not v.get("compare") and not asks_output(q["text"]):
            out["status"], out["reason"] = "ran", "ran OK, but no observable to check the answers against"
            return out
        if observe:
            if "[" in observe:  # an element expression such as a[2]: print it and compare stdout
                res2 = run_plain(build(blocks, v, "", after_stmt + f" System.out.println({observe});"))
                if "stdout" not in res2 or "run_error" in res2:
                    return {"status": "skip", "reason": f"cannot evaluate {observe!r}"}
                real = norm_val(res2["stdout"])
                shown = f"{observe} = {res2['stdout'].strip()}"
            elif observe not in res["final"]:
                return {"status": "skip", "reason": f"cannot observe {observe!r} at end of main"}
            else:
                real = norm_val(res["final"][observe])
                shown = f"final {observe} = {res['final'][observe]}"
            if str(shown).split("= ", 1)[-1].startswith('"'):  # String value: compare exactly, keeping spaces
                actual = shown.split("= ", 1)[-1][1:-1]
                hits = [a["id"] for a in q["answers"] if ans_exact(a).strip("\"") == actual]
            else:
                hits = [a["id"] for a in q["answers"] if norm_val(ans_code(a)) == real]
            out["kind"] = "state"
        else:
            real = norm(stdout)
            hits = [a["id"] for a in q["answers"] if norm(a["text"]) == real]
            shown = f"real output {real!r}"
    if hits == keyed:
        out["status"] = "pass"
    elif not hits:
        out["status"], out["reason"] = "unmatched", f"{shown} equals no choice (format differs, mid-execution question, or key wrong); key is {keyed}"
    else:
        out["status"], out["reason"] = "FAIL", f"{shown} matches {hits}, key is {keyed}"
    return out


def ex_substitute(q, blocks, v):
    """Put each choice's code where the stem has /* missing code */ (or, with no hole, in place of the stem's
    fragment) and run it on cases=[{before, after, expect_output}]. Passers must be exactly the key
    (or exactly the non-key with expect='fails')."""
    cases = v.get("cases") or [{"expect_output": v["expect_output"]}]
    outs, passers = {}, []
    has_hole = any(HOLE_RE.search(b) for b in blocks)
    for a in q["answers"]:
        code = ans_code(a)
        subbed = [HOLE_RE.sub(lambda m: code, b) for b in blocks] if has_hole else [b for b in blocks if classify(b) != "stmts"] + [code]
        verdicts = []
        for c in cases:
            got, err = run_case(subbed, v, c)
            outs.setdefault(a["id"], []).append(got if err is None else "ERROR: " + err)
            verdicts.append(err is None and got == norm(c["expect_output"]))
        if all(verdicts):
            passers.append(a["id"])
    out = {"kind": "substitute", "per_choice": outs}
    keyed = keyed_ids(q)
    if v.get("expect") == "fails":
        return decide(out, [i for i in outs if i not in passers], keyed, "fail the cases")
    return decide(out, passers, keyed, "satisfy the cases")


def ex_patch(q, blocks, v):
    """variants={choice_id: [[old, new], ...]}: apply each choice's proposed change to the stem code, then require
    expect='compiles' or the cases' expect_output. The unpatched code must NOT already satisfy the check."""
    def passes(bl, vv):
        if vv.get("expect") == "compiles":
            src = build(bl, vv) or classes_only(bl, vv.get("prelude", ""))
            ok, err = compile_ok(src)
            return ok, "compiles" if ok else "error: " + err
        results, ok = [], True
        for c in vv.get("cases") or [{"expect_output": vv["expect_output"]}]:
            got, err = run_case(bl, vv, c)
            results.append(got if err is None else "ERROR: " + err)
            if err is not None or got != norm(c["expect_output"]):
                ok = False
                break  # one failing case is enough (and avoids waiting on later timeouts)
        return ok, " | ".join(results)
    base_ok, base_res = passes(blocks, v)
    if base_ok:
        return {"status": "skip", "reason": "unpatched stem already satisfies the check (harness too weak)"}
    outs, hits = {"(unpatched)": base_res}, []
    for cid, patches in v["variants"].items():
        try:
            bl, vv = patched(blocks, v, patches)
        except ValueError as e:
            return {"status": "skip", "reason": f"choice {cid}: {e}"}
        ok, res = passes(bl, vv)
        outs[cid] = res
        if ok:
            hits.append(cid)
    return decide({"kind": "patch", "per_choice": outs}, hits, keyed_ids(q), "fix the code")


def ex_compile(q, blocks, v, ask):
    """Each choice is a statement (v.template wraps it, e.g. "int x = {};") compiled in a client class next to the stem's
    classes. expect='compiles'|'fails' (else inferred from the question wording)."""
    results = {}
    for a in q["answers"]:
        code = ans_code(a)
        if v.get("template"):
            code = v["template"].replace("{}", code)
        elif not code.endswith(";") and not v.get("kind"):
            return {"status": "skip", "reason": "compile check: choices are not statements"}
        src = classes_only(blocks, v.get("prelude", "")) + f"\nclass Client {{ {v.get('members', '')} void m() throws Exception {{ {v.get('before', '')} {code} }} }}\n"
        ok, err = compile_ok(src)
        results[a["id"]] = "compiles" if ok else "error: " + err
    negative = v["expect"] == "fails" if "expect" in v else bool(re.search(r"\b(?:not|cannot)\b|n't", ask, re.I))
    want = "error" if negative else "compiles"
    hits = [i for i, r in results.items() if r.startswith(want)]
    out = {"kind": "compile", "per_choice": results}
    if (not hits or len(hits) == len(results)) and not v:
        out["status"] = "skip"
        out["reason"] = "needs harness: compile check does not discriminate (" + next(iter(results.values())) + ")"
        return out
    return decide(out, hits, keyed_ids(q), "fail to compile" if negative else "compile")


def ex_witness(q, blocks, v):
    """cases={choice_id: {before, after, expect_output}}: expect_output is the *intended* result. The keyed choice's
    case must be the only one where the stem code misbehaves (wrong output or an exception)."""
    fails, detail = [], {}
    for cid, c in v["cases"].items():
        got, err = run_case(blocks, v, c)
        detail[cid] = f"ERROR: {err}" if err else f"got {got!r}, intended {norm(c['expect_output'])!r}"
        if err or got != norm(c["expect_output"]):
            fails.append(cid)
    return decide({"kind": "witness", "per_choice": detail}, fails, keyed_ids(q), "show wrong behavior")


def ex_oracle(q, blocks, v):
    """One program loops over inputs (v.loops: Java for-header(s)); inside, the stem fragment (or v.fragment) runs and each
    choice is a Java expression compared with v.subject. The key must be the only choice equal on every input."""
    frag = v.get("fragment") or "\n".join(b for b in blocks if classify(b) == "stmts")
    others = [b for b in blocks if classify(b) != "stmts"]
    ids = list(v["choices"])
    checks = " ".join(f'__s += java.util.Objects.equals({v["subject"]}, ({v["choices"][i]})) ? "1" : "0";' for i in ids)
    loop = f'{v["loops"]} {{\n{frag}\nString __s = ""; {checks}\nSystem.out.println(__s + " " + ({v.get("show", chr(34) + chr(34))}));\n}}'
    src = compose(others, loop, v.get("before", ""), v.get("prelude", ""), v.get("members", ""), skip_stmts=True)
    res = run_plain(src)
    if "stdout" not in res or "run_error" in res or "compile_error" in res:
        return {"status": "skip", "reason": "oracle harness failed: " + (res.get("compile_error") or res.get("run_error", "?"))}
    rows = [l.split(" ", 1) for l in res["stdout"].strip().split("\n") if l.strip()]
    equiv, witness = [], {}
    for k, cid in enumerate(ids):
        bad = next((r[1] if len(r) > 1 else "?" for r in rows if r[0][k] == "0"), None)
        if bad is None:
            equiv.append(cid)
        else:
            witness[cid] = f"differs at {bad.strip()}"
    return decide({"kind": "oracle", "per_choice": {**witness, **{i: "equivalent" for i in equiv}}, "inputs": len(rows)},
                  equiv, keyed_ids(q), "match on every input")


def ex_equivalent(q, blocks, v):
    """Each choice (a full fragment) must leave the same observable output as the stem's own fragment."""
    want, err = run_case(blocks, v, {"after": v["after"]})
    if err:
        return {"status": "skip", "reason": "stem itself failed: " + err}
    outs, hits = {}, []
    for a in q["answers"]:
        got, err = run_case([b for b in blocks if classify(b) != "stmts"] + [ans_code(a)], v, {"after": v["after"]})
        outs[a["id"]] = got if err is None else "ERROR: " + err
        if err is None and got == want:
            hits.append(a["id"])
    return decide({"kind": "equivalent", "per_choice": {"(stem)": want, **outs}}, hits, keyed_ids(q), "match the stem's result")


def ex_segments(q, blocks, v):
    """Run each stem code segment separately -> r[i] = its output or 'ERR'; `claims` are Python expressions over r."""
    r = []
    for b in [b for b in blocks if classify(b) == "stmts"]:
        got, err = run_case([b], v, {})
        r.append("ERR" if err else got)
    return decide({"kind": "segments", "per_choice": {"results": str(r)}}, eval_claims(v["claims"], {"r": r}), keyed_ids(q), "are true of the runs")


# ---------------------------------------------------------------- dispatch

def verify(q, v=None):
    v = v or q.get("verification") or {}
    has_block = bool(v)
    if v.get("kind") == "none":
        return {"status": "unverifiable", "reason": v.get("reason", "no executable ground truth"), "kind": "none", "source": "block"}
    blocks = java_blocks(q["text"])
    if v.get("ignore_stem"):
        blocks = []
    elif not blocks:
        return {"status": "skip", "reason": "no java in stem"}
    if v.get("rewrite"):
        blocks = [functools.reduce(lambda t, p: t.replace(p[0], p[1]), v["rewrite"], b) for b in blocks]
    ask = (prose_lines(re.sub(r"```.*?```", "", q["text"], flags=re.S)) or [""])[-1]
    kind = v.get("kind")
    answers_are_code = all(re.search(r"[;(){}=]", ans_code(a)) for a in q["answers"])
    if not kind:
        if any(HOLE_RE.search(b) for b in blocks) and answers_are_code:
            m = re.search(r'prints?\s+(?:the\s+string\s+)?`?"([^"]*)"', q["text"])
            if not m or re.search(r"cannot|not\b", ask):
                return {"status": "skip", "reason": "substitute: expected result is not stated literally (needs a spec)"}
            v, kind = {**v, "expect_output": m.group(1)}, "substitute"
        elif answers_are_code and re.search(r"\bequivalent\b", ask, re.I):
            return {"status": "skip", "reason": "equivalence question (needs a block)"}
        elif answers_are_code and re.search(r"compile.*\band\b", ask, re.I):
            return {"status": "skip", "reason": "compile check: question also requires a run-time result (needs a block)"}
        elif answers_are_code and re.search(r"compile|correctly create|creates? (?:a|an) ", ask, re.I):
            kind = "compile"
        else:
            kind = "run"
    out = {"run": lambda: ex_run(q, blocks, v, ask), "state": lambda: ex_run(q, blocks, v, ask),
           "substitute": lambda: ex_substitute(q, blocks, v), "compile": lambda: ex_compile(q, blocks, v, ask),
           "witness": lambda: ex_witness(q, blocks, v), "oracle": lambda: ex_oracle(q, blocks, v),
           "patch": lambda: ex_patch(q, blocks, v), "equivalent": lambda: ex_equivalent(q, blocks, v),
           "segments": lambda: ex_segments(q, blocks, v)}[kind]()
    out["source"] = "block" if has_block else "auto"
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("banks", nargs="+")
    ap.add_argument("--blocks", help="sidecar JSON {question_id: verification block}")
    ap.add_argument("--trace")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("-j", "--jobs", type=int, default=os.cpu_count(), help="questions verified in parallel (default: all cores)")
    a = ap.parse_args()
    qs = []
    for p in a.banks:
        qs += json.load(open(p))["questions"]
    sidecar = json.load(open(a.blocks)) if a.blocks else {}
    if a.trace:
        q = next(x for x in qs if x["id"] == a.trace)
        r = verify(q, sidecar.get(q["id"]))
        print(f"{q['id']}: {r['status']} {r.get('reason', '')}")
        for k, val in (r.get("per_choice") or {}).items():
            print(f"   {k}: {val}")
        if "_res" in r:
            print(f"stdout: {r['stdout']!r}\n{'line':>4}  {'executed':<45} state after")
            for n, src, st in render_trace(r["_res"], r["_code"]):
                print(f"{n:>4}  {src:<45} {st}")
        return
    def one(q):
        r = verify(q, sidecar.get(q["id"]))
        return {"id": q["id"], **{k: v for k, v in r.items() if not k.startswith("_")}}
    with ThreadPoolExecutor(max_workers=max(1, a.jobs)) as pool:  # work is subprocess-bound (javac/java), so threads suffice
        results = list(pool.map(one, qs))
    if a.json:
        print(json.dumps(results, indent=1)); return
    c = Counter(r["status"] for r in results)
    print("summary:", dict(c))
    print("verified by kind/source:", dict(Counter(f"{r.get('kind')}/{r.get('source')}" for r in results if r["status"] == "pass")))
    for r in results:
        if r["status"] in ("FAIL", "unmatched"):
            print(f"  {r['status']:<9} {r['id']}: {r['reason']}")
    print("\nskip reasons:")
    for k, v in Counter((r["reason"].split(":")[0] + (": " + r["reason"].split(":", 1)[1].strip()[:60] if r["reason"].startswith("needs") else ""))
                        for r in results if r["status"] == "skip").most_common():
        print(f"  {v:>3}  {k}")


if __name__ == "__main__":
    main()
