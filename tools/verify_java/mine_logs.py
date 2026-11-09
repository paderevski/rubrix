#!/usr/bin/env python3
"""Extract generated CS questions (and classify failed generations) from Rubrix llm_log*.txt files.

usage: mine_logs.py LOG.txt [LOG2.txt ...] [--since 'YYYY-MM-DD HH:MM:SS'] [--out mined.json] [--subject CS|Calc]
Writes a list of {"log","ts","topic","q"} entries that review.py can read.
"""
import argparse, collections, json, re

DEC = json.JSONDecoder()


# gpt-oss sometimes drops the OPENING quote of a string value: `"explanation": Text here."` -> repairable
MISSING_OPEN_QUOTE = re.compile(r'("(?:explanation|text)"\s*:\s*)(?=[A-Za-z0-9`$(\\])')


def final_array(resp, repair=True):
    """Last JSON array in the response that looks like questions (reasoning text may precede/contain other brackets)."""
    for m in reversed(list(re.finditer(r"\[\s*\{", resp))):
        text = resp[m.start():]
        arr = None
        for candidate in ([text, MISSING_OPEN_QUOTE.sub(r'\1"', text)] if repair else [text]):
            try:
                arr, _ = DEC.raw_decode(candidate)
                break
            except Exception:
                continue
        if arr is None:
            continue
        if isinstance(arr, list) and arr and isinstance(arr[0], dict) and "answers" in arr[0] and "text" in arr[0]:
            return arr
    return None


def classify_failure(resp):
    """Why no question array was recovered."""
    opened = len(re.findall(r"<question>", resp, re.I))
    closed = len(re.findall(r"</question>", resp, re.I))
    if not resp.strip():
        return "empty response"
    if opened and not closed:
        return "truncated (<question> never closed)"
    if opened and closed:
        return "tagged block present but JSON invalid"
    if "[" in resp and '"answers"' in resp:
        return "JSON-like text but unparseable (likely truncated/bad escapes)"
    return "no question JSON at all (reasoning only / cut off)"


def mine(paths, since="", subject="CS"):
    out, fails, calls = [], collections.Counter(), 0
    key = {"CS": "Computer Science", "Calc": "Calculus"}[subject]
    for f in paths:
        for e in re.split(r"={80}\nTIMESTAMP: ", open(f).read())[1:]:
            ts = e[:19]
            m = re.search(r"--- PROMPT ---(.*?)--- RESPONSE ---(.*)", e, re.S)
            if not m or ts < since:
                continue
            prompt, resp = m.groups()
            if key not in prompt[:800]:
                continue
            calls += 1
            arr = final_array(resp)
            if not arr:
                fails[classify_failure(resp)] += 1
                continue
            topic = re.search(r"Target Topic\(s\):\*\*\s*(.*)", prompt)
            for q in arr:
                if len(q.get("answers", [])) >= 2:
                    out.append({"log": f, "ts": ts, "topic": topic.group(1).strip() if topic else "", "q": q})
    return out, calls, fails


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("logs", nargs="+")
    ap.add_argument("--since", default="")
    ap.add_argument("--subject", default="CS")
    ap.add_argument("--out")
    a = ap.parse_args()
    out, calls, fails = mine(a.logs, a.since, a.subject)
    print(f"{calls} {a.subject} calls -> {len(out)} questions recovered; {sum(fails.values())} calls with none")
    for k, v in fails.most_common():
        print(f"  {v:>3}  {k}")
    by_day = collections.Counter(o["ts"][:10] for o in out)
    print("questions by day:", dict(sorted(by_day.items())))
    if a.out:
        json.dump(out, open(a.out, "w"))
