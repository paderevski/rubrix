#!/usr/bin/env python3
"""Convert a repaired question markdown file plus a labels JSON into a question bank.

Usage: tools/md_to_bank.py QUESTIONS.md LABELS.json OUT.json ID_PREFIX "SOURCE TEXT" [SKIP]

SKIP is an optional comma-separated list of question numbers to leave out (e.g. duplicates).

Markdown format per question: `## Question N`, stem, `- (A) ...` options, `**Answer:** X`,
`**Explanation:**` overall text followed by one bullet per WRONG option (in option order).
LABELS.json maps the question number to topics/subtopics/skills/difficulty/cognitive_level.
"""
import json, re, sys

def parse(md):
    parts = re.split(r"^## Question (\d+)\s*$", md, flags=re.M)
    for i in range(1, len(parts), 2):
        yield int(parts[i]), parts[i + 1].strip("\n")

def split_question(n, body):
    a_idx = body.index("**Answer:**")
    e_idx = body.index("**Explanation:**")
    head, expl = body[:a_idx], body[e_idx + len("**Explanation:**"):].strip()
    answer = re.search(r"\*\*Answer:\*\*\s*\(?([A-D])", body[a_idx:]).group(1)
    opt_re = re.compile(r"^- \(([A-D])\)[ \t]*", re.M)
    ms = list(opt_re.finditer(head))
    assert [m.group(1) for m in ms] == list("ABCD"), (n, "options")
    stem = head[: ms[0].start()].strip()
    options = []
    for k, m in enumerate(ms):
        end = ms[k + 1].start() if k + 1 < len(ms) else len(head)
        text = head[m.end():end].strip("\n")
        lines = text.split("\n")
        text = "\n".join([lines[0]] + [l[2:] if l.startswith("  ") else l for l in lines[1:]]).strip()
        options.append((m.group(1), text))
    # explanation: overall paragraph(s) then bullets per wrong option
    lines = expl.split("\n")
    first_bullet = next((i for i, l in enumerate(lines) if l.startswith("- ")), len(lines))
    overall = "\n".join(lines[:first_bullet]).strip()
    bullets, cur = [], None
    for l in lines[first_bullet:]:
        if l.startswith("- "):
            cur = [l[2:]]; bullets.append(cur)
        elif cur is not None and l.strip():
            cur.append(l.strip())
    wrong_expl = []
    for b in bullets:
        t = " ".join(b)
        m = re.match(r"^(`+).+?\1:\s*(.*)$", t)
        e = m.group(2) if m else t
        wrong_expl.append(e[:1].upper() + e[1:])
    return stem, options, answer, overall, wrong_expl

def main(md_path, labels_path, out_path, prefix, source, skip=""):
    skipped = {int(x) for x in skip.split(",") if x.strip()}
    labels = json.load(open(labels_path))
    questions = []
    for n, body in parse(open(md_path).read()):
        if n in skipped:
            continue
        stem, options, answer, overall, wrong_expl = split_question(n, body)
        wrong = [o for o in options if o[0] != answer]
        assert len(wrong_expl) == len(wrong), (n, len(wrong_expl))
        wi = iter(wrong_expl)
        answers = []
        for letter, text in options:
            correct = letter == answer
            answers.append({"id": letter.lower(), "text": text, "is_correct": correct,
                            "explanation": ("Correct. " + overall) if correct else next(wi)})
        lab = labels[str(n)]
        questions.append({"id": f"{prefix}_q{n:03d}", "status": "active", "text": stem,
                          "answers": answers, "explanation": overall,
                          "topics": lab["topics"], "subtopics": lab["subtopics"], "skills": lab["skills"],
                          "difficulty": lab["difficulty"], "cognitive_level": lab["cognitive_level"]})
    schema_version = json.load(open("imports/knowledge/Computer Science/question-schema.json"))["schema_version"]
    json.dump({"schema_version": schema_version, "source": source, "questions": questions},
              open(out_path, "w"), indent=2, ensure_ascii=False)
    open(out_path, "a").write("\n")
    print(f"{len(questions)} questions -> {out_path}")

if __name__ == "__main__":
    main(*sys.argv[1:7])
