#!/usr/bin/env python3
"""Convert Calculus schema from unit/topic shape into parser-friendly topics/subtopics.

Current parser logic expects:
- topics.items: top-level selectable nodes (codes used for matching)
- subtopics.items: child nodes with parent_topic

This script can:
1) Build a transformed schema where units become topics and AP topics become subtopics.
2) Optionally update question-bank pedagogy.topics to include unit IDs so current
   matching logic works for both parent and child selections.
"""

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Tuple


@dataclass
class ConversionResult:
    schema_out: dict
    topic_to_unit: Dict[str, str]
    missing_unit_topics: List[str]


def load_json(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def save_json(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        json.dump(payload, f, indent=2)
        f.write("\n")


def build_hierarchical_schema(schema: dict) -> ConversionResult:
    units = schema.get("ap_calc_units", {}).get("items", [])
    topics = schema.get("topics", {}).get("items", [])

    unit_topics = []
    for unit in units:
        unit_id = unit.get("id", "").strip()
        if not unit_id:
            continue
        unit_topics.append(
            {
                "id": unit_id,
                "name": unit.get("name", unit_id),
                "display": unit.get("display", unit_id),
            }
        )

    subtopics = []
    topic_to_unit: Dict[str, str] = {}
    missing_unit_topics: List[str] = []

    for topic in topics:
        topic_id = str(topic.get("id", "")).strip()
        if not topic_id:
            continue

        parent_unit = str(topic.get("unit", "")).strip()
        if not parent_unit:
            # Keep track of entries that cannot be mapped into hierarchy.
            missing_unit_topics.append(topic_id)
            continue

        topic_to_unit[topic_id] = parent_unit

        subtopics.append(
            {
                "id": topic_id,
                "name": topic.get("name", topic_id),
                "display": topic.get("display", topic_id),
                "parent_topic": parent_unit,
            }
        )

    out = {
        "schema_version": schema.get("schema_version", "1.0.0"),
        "description": (
            "Controlled vocabularies for AP Calculus AB/BC pedagogy metadata "
            "(hierarchical for Rubrix parser)"
        ),
        "last_updated": schema.get("last_updated", ""),
        "source": schema.get("source", ""),
        "topics": {
            "description": "Units mapped as top-level topics",
            "items": unit_topics,
        },
        "subtopics": {
            "description": "Original AP Calculus topics mapped under their unit",
            "items": subtopics,
        },
    }

    return ConversionResult(
        schema_out=out,
        topic_to_unit=topic_to_unit,
        missing_unit_topics=missing_unit_topics,
    )


def append_units_to_question_bank_topics(
    question_bank: dict, topic_to_unit: Dict[str, str]
) -> Tuple[int, int]:
    updated_questions = 0
    appended_tags = 0

    for q in question_bank.get("questions", []):
        pedagogy = q.get("pedagogy", {})
        topics = pedagogy.get("topics", [])
        if not isinstance(topics, list):
            continue

        ordered = [str(t) for t in topics if isinstance(t, str)]
        existing = set(ordered)
        to_append: List[str] = []

        for topic_id in ordered:
            unit_id = topic_to_unit.get(topic_id)
            if unit_id and unit_id not in existing and unit_id not in to_append:
                to_append.append(unit_id)

        if to_append:
            pedagogy["topics"] = ordered + to_append
            q["pedagogy"] = pedagogy
            updated_questions += 1
            appended_tags += len(to_append)

    return updated_questions, appended_tags


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Convert Calculus schema to parser-friendly hierarchy"
    )
    parser.add_argument(
        "--schema",
        default="imports/knowledge/Calculus/question-schema.json",
        help="Path to source Calculus question-schema.json",
    )
    parser.add_argument(
        "--out-schema",
        default="imports/knowledge/Calculus/question-schema.hierarchical.json",
        help="Path for converted schema output",
    )
    parser.add_argument(
        "--question-bank",
        default="imports/knowledge/Calculus/question-bank.json",
        help="Path to Calculus question-bank.json",
    )
    parser.add_argument(
        "--out-question-bank",
        default="imports/knowledge/Calculus/question-bank.with-units.json",
        help="Path for converted question bank output when --update-question-bank is set",
    )
    parser.add_argument(
        "--update-question-bank",
        action="store_true",
        help="Append unit IDs (U1...) to pedagogy.topics for parser compatibility",
    )
    args = parser.parse_args()

    schema_path = Path(args.schema)
    out_schema_path = Path(args.out_schema)

    schema = load_json(schema_path)
    converted = build_hierarchical_schema(schema)
    save_json(out_schema_path, converted.schema_out)

    print(f"Wrote hierarchical schema: {out_schema_path}")
    print(
        f"Mapped topics -> subtopics: {len(converted.topic_to_unit)}; "
        f"unmapped entries: {len(converted.missing_unit_topics)}"
    )
    if converted.missing_unit_topics:
        preview = ", ".join(converted.missing_unit_topics[:10])
        print(f"Unmapped topic IDs (first 10): {preview}")

    if args.update_question_bank:
        bank_path = Path(args.question_bank)
        out_bank_path = Path(args.out_question_bank)

        question_bank = load_json(bank_path)
        updated_questions, appended_tags = append_units_to_question_bank_topics(
            question_bank, converted.topic_to_unit
        )
        save_json(out_bank_path, question_bank)

        print(f"Wrote question bank with unit tags: {out_bank_path}")
        print(
            f"Updated questions: {updated_questions}; "
            f"total unit tags appended: {appended_tags}"
        )


if __name__ == "__main__":
    main()
