"""Read-only access to the repo's knowledge base, mirroring what the Rust app does (src-tauri/src/knowledge.rs, prompts.rs).

Nothing here writes to imports/knowledge or touches the app's prompts: the baseline arm renders the repo's own
prompt.txt exactly as the app would, and the new pipeline reuses the same banks, schema and example-selection tiers.
"""
import json
import random
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
KB = ROOT / "imports" / "knowledge"
SIDECAR = ROOT / "tools" / "verify_java" / "blocks" / "cs-draft.json"

DIFFICULTY_DESC = {
    "easy": "D1 (Easy) - Basic recall or simple application, 1-2 steps",
    "medium": "D2 (Medium) - Requires analysis or multi-step reasoning, 3-5 steps",
    "hard": "D3 (Hard) - Complex analysis, synthesis of multiple concepts, 5+ steps",
}
DIFFICULTY_CODE = {"easy": "D1", "medium": "D2", "hard": "D3"}


class Subject:
    def __init__(self, name="Computer Science", with_blocks=False):
        base = KB / name
        self.name = name
        self.schema = json.load(open(base / "question-schema.json"))
        self.prompt = (base / "prompt.txt").read_text()
        self.entries = []
        for f in sorted((base / "banks").glob("*.json")):
            self.entries += json.load(open(f))["questions"]
        self.blocks = json.load(open(SIDECAR)) if with_blocks and SIDECAR.exists() else {}
        self.topics = {t["id"]: t["display"] for t in self.schema["topics"]["items"] if t.get("id")}
        self.subtopics = {s["id"]: s for s in self.schema["subtopics"]["items"] if s.get("id")}
        self.common_errors = [e["display"] for e in self.schema["common_errors"]["items"] if e.get("display")]
        self.question_types = [q["name"] for q in self.schema["question_types"]["items"] if q.get("name")]

    # -- topic labels, as topic_labels_for_prompt in main.rs builds them
    def with_questions(self):
        have = {t for e in self.entries for t in e["topics"]}
        return {t: n for t, n in self.topics.items() if t in have}

    def label(self, tid):
        if tid in self.subtopics:
            s = self.subtopics[tid]
            parent = self.topics.get(s["parent_topic"], "")
            return f"{tid}: {parent} > {s['display']}" if parent else f"{tid}: {s['display']}"
        return f"{tid}: {self.topics[tid]}" if tid in self.topics else tid

    def labels(self, topic_ids):
        seen, out = set(), []
        for tid in topic_ids:
            lab = self.label(tid)
            if lab and lab not in seen:
                seen.add(lab)
                out.append(lab)
        return ", ".join(out) or ", ".join(topic_ids)

    def parent_codes(self, topic_ids):
        return {self.subtopics[t]["parent_topic"] if t in self.subtopics else t for t in topic_ids}

    # -- example selection: the three tiers of KnowledgeBase::get_bank_examples
    def select_examples(self, topic_ids, difficulty=None, max_total=3, rng=None):
        rng = rng or random.Random()
        code = DIFFICULTY_CODE.get(difficulty)
        parents = self.parent_codes(topic_ids)
        seen, out = set(), []

        def id_match(e):
            return any(t in e["topics"] or t in (e.get("subtopics") or []) for t in topic_ids)

        def add(pred):
            if len(out) >= max_total:
                return
            cand = [e for e in self.entries if e.get("status", "active") == "active" and e["id"] not in seen and pred(e)]
            rng.shuffle(cand)
            for e in cand[: max_total - len(out)]:
                seen.add(e["id"])
                out.append(e)

        add(lambda e: id_match(e) and (code is None or e["difficulty"] == code))
        if code:
            add(id_match)
        add(lambda e: any(t in parents for t in e["topics"]))
        return out

    def example_json(self, e, with_block=False):
        """An example as the app serializes it (all bank metadata), optionally with its verification block."""
        d = {"id": e["id"], "status": e.get("status", "active"), "_migration_note": e.get("_migration_note"), "text": e["text"],
             "answers": [{"id": a["id"], "text": a["text"], "is_correct": a["is_correct"], "explanation": a.get("explanation", "")} for a in e["answers"]],
             "explanation": e["explanation"], "difficulty": e["difficulty"], "cognitive_level": e["cognitive_level"],
             "topics": e["topics"], "subtopics": e.get("subtopics")}
        if with_block and e["id"] in self.blocks and self.blocks[e["id"]].get("kind") != "none":
            d["verification"] = self.blocks[e["id"]]
        return json.dumps(d, indent=2, ensure_ascii=False)

    # -- the app's own prompt, rendered the way format_custom_prompt does
    def baseline_prompt(self, topic_ids, difficulty, count, examples, notes=None):
        ex = "\n\n".join(f"### Example {i + 1}\n```json\n{self.example_json(e)}\n```" for i, e in enumerate(examples)) or "(No examples available)"
        instr = f"\n\n**Additional Instructions from User:**\n{notes}" if notes and notes.strip() else ""
        return (self.prompt.replace("{topics}", self.labels(topic_ids))
                .replace("{topic_ids}", json.dumps(topic_ids))
                .replace("{difficulty}", DIFFICULTY_DESC.get(difficulty, DIFFICULTY_DESC["medium"]))
                .replace("{difficulty_code}", DIFFICULTY_CODE.get(difficulty, "D2"))
                .replace("{count}", str(count))
                .replace("{examples}", ex)
                .replace("{user_instructions}", instr)
                .replace("{regenerate}", ""))
