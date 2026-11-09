#!/usr/bin/env python3
"""Validate a subject's question-schema.json and question banks.

Usage: tools/validate_knowledge.py "imports/knowledge/Computer Science"

Checks: unique ids/names, subtopic parents, AP coverage (every ap_topics item
referenced by a subtopic), bank vocabulary references, and answer rules.
"""
import glob, json, os, sys
from collections import Counter

def main(folder):
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
    return 1 if errors else 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "imports/knowledge/Computer Science"))
