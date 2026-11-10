#!/usr/bin/env python3
"""Validate a subject's question-schema.json and question banks.

Usage: tools/validate_knowledge.py "imports/knowledge/Computer Science" [--coverage]

--coverage also prints how many questions use each topic and subtopic (all banks
combined), and the AP topics with no questions.

Checks: unique ids/names, subtopic parents, AP coverage (every ap_topics item
referenced by a subtopic), bank vocabulary references, and answer rules.
"""
import glob, json, os, sys
from collections import Counter

def coverage(schema, questions):
    tc = Counter(t for q in questions for t in q.get("topics", []))
    sc = Counter(s for q in questions for s in q.get("subtopics", []))
    kids = {}
    for st in schema["subtopics"]["items"]:
        kids.setdefault(st["parent_topic"], []).append(st)
    print(f"\nCoverage: {len(questions)} questions")
    for t in schema["topics"]["items"]:
        print(f"{t['id']:5}{tc[t['id']]:4}  {t['display']}")
        for st in kids.get(t["id"], []):
            print(f"  {st['id']:7}{sc[st['id']]:4}  {st['display']}")
    zero = [st["id"] for st in schema["subtopics"]["items"] if not sc[st["id"]]]
    print(f"\nTopics with no questions: {[t['id'] for t in schema['topics']['items'] if not tc[t['id']]]}")
    print(f"Subtopics with no questions ({len(zero)} of {len(schema['subtopics']['items'])}): {zero}")
    if "ap_topics" in schema:
        used = {r for st in schema["subtopics"]["items"] if sc[st["id"]] for r in st.get("ap_refs", [])}
        missing = [a["id"] for a in schema["ap_topics"]["items"] if a["id"] not in used]
        print(f"AP topics with no questions ({len(missing)} of {len(schema['ap_topics']['items'])}): {missing}")


def main(folder, show_coverage=False):
    all_questions = []
    errors = []
    schema = json.load(open(os.path.join(folder, "question-schema.json")))
    sections = {k: {i["id"] for i in v["items"] if "id" in i}
                for k, v in schema.items() if isinstance(v, dict) and "items" in v}
    for sec, v in schema.items():
        if isinstance(v, dict) and "items" in v:
            ids = [i["id"] for i in v["items"] if "id" in i]
            errors += [f"duplicate id in {sec}: {d}" for d, c in Counter(ids).items() if c > 1]
            if sec in ("topics", "subtopics"):
                names = [i["name"] for i in v["items"] if "name" in i]
                errors += [f"duplicate name in {sec}: {d}" for d, c in Counter(names).items() if c > 1]
    for st in schema.get("subtopics", {}).get("items", []):
        if st["parent_topic"] not in sections["topics"]:
            errors.append(f"{st['id']}: unknown parent {st['parent_topic']}")
        for r in st.get("ap_refs", []):
            if r not in sections.get("ap_topics", set()):
                errors.append(f"{st['id']}: unknown ap_ref {r}")
    if "ap_topics" in sections:
        covered = {r for st in schema["subtopics"]["items"] for r in st.get("ap_refs", [])}
        errors += [f"AP topic {a} has no subtopic" for a in sorted(sections["ap_topics"] - covered)]
    parent = {s["id"]: s["parent_topic"] for s in schema.get("subtopics", {}).get("items", [])}
    vocab = {"topics": "topics", "subtopics": "subtopics", "skills": "skills", "common_errors": "common_errors"}
    for path in sorted(glob.glob(os.path.join(folder, "banks", "*.json"))):
        bank = json.load(open(path))
        qs = bank.get("questions") if isinstance(bank, dict) else None
        if qs is None:
            continue
        all_questions.extend(qs)
        for q in qs:
            qid = f"{os.path.basename(path)}:{q.get('id')}"
            for f, sec in vocab.items():
                for x in q.get(f, []):
                    if x not in sections.get(sec, set()):
                        errors.append(f"{qid}: unknown {f} {x!r}")
            for f, sec in (("difficulty", "difficulty_levels"), ("cognitive_level", "bloom_taxonomy"), ("status", "item_status")):
                if q.get(f) not in sections.get(sec, set()):
                    errors.append(f"{qid}: unknown {f} {q.get(f)!r}")
            if not q.get("topics"):
                errors.append(f"{qid}: no topics")
            answers = q.get("answers", [])
            if not 4 <= len(answers) <= 5:
                errors.append(f"{qid}: {len(answers)} answers")
            if sum(bool(a.get("is_correct")) for a in answers) != 1:
                errors.append(f"{qid}: needs exactly one correct answer")
            for s in q.get("subtopics", []):
                if s in parent and parent[s] not in q.get("topics", []):
                    errors.append(f"{qid}: subtopic {s} parent {parent[s]} not in topics")
    for e in errors:
        print(e)
    print(f"{len(errors)} problem(s)")
    if show_coverage:
        coverage(schema, all_questions)
    return 1 if errors else 0

if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    sys.exit(main(args[0] if args else "imports/knowledge/Computer Science",
                  "--coverage" in sys.argv))
