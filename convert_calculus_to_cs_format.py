#!/usr/bin/env python3
"""
Convert Calculus question bank from Django model format to AP CS format.
Pedagogical fields are populated with null/empty values.
"""

import json
from typing import Any, Dict, List


def convert_choices_to_options(
    choices: Dict[str, str], correct_answer: str
) -> List[Dict[str, Any]]:
    """Convert choice dict to options array format."""
    options = []
    for letter, text in sorted(choices.items()):
        options.append(
            {"id": letter.lower(), "text": text, "is_correct": letter == correct_answer}
        )
    return options


def convert_question(calc_question: Dict[str, Any]) -> Dict[str, Any]:
    """Convert a single Calculus question to AP CS format."""
    fields = calc_question["fields"]

    # Generate ID from pk
    question_id = f"calc_q{str(calc_question['pk'])[4:]}"

    # Convert to AP CS format
    cs_question = {
        "id": question_id,
        "type": "QT02",  # Default to multiple choice
        "structure_type": None,
        "difficulty": None,
        "cognitive_level": None,
        "estimated_time_seconds": None,
        "content": {
            "code_language": None,
            "options": convert_choices_to_options(
                fields["choices"], fields["correct_answer"]
            ),
            "explanation": fields.get("explanation", ""),
            "text": fields["text"],
        },
        "pedagogy": {
            "ap_cs_unit": None,
            "ap_cs_topics": [],
            "topics": fields.get("topics", []),
            "subtopics": [],
            "skills": [],
        },
        "distractors": {"common_mistakes": [], "common_errors": []},
    }

    # Add metadata from 'other' field if available
    if "other" in fields and fields["other"]:
        cs_question["metadata"] = fields["other"]

    return cs_question


def convert_question_bank(input_file: str, output_file: str):
    """Convert entire question bank from Calculus to AP CS format."""

    # Read Calculus format
    with open(input_file, "r", encoding="utf-8") as f:
        calc_questions = json.load(f)

    # Extract source info from first question if available
    first_question = calc_questions[0] if calc_questions else {}
    source_info = first_question.get("fields", {}).get("other", {})

    # Create AP CS format structure
    cs_format = {
        "schema_version": "2.1.0",
        "source": f"AP Calculus Practice Exam {source_info.get('year', 'Unknown')}",
        "exam_format": f"Section {source_info.get('part', 'Unknown')} - Multiple Choice",
        "total_questions": len(calc_questions),
        "total_time_minutes": None,
        "structure_types": [],
        "questions": [],
    }

    # Convert each question
    for calc_q in calc_questions:
        cs_q = convert_question(calc_q)
        cs_format["questions"].append(cs_q)

    # Write output
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(cs_format, f, indent=2, ensure_ascii=False)

    print(f"Converted {len(calc_questions)} questions")
    print(f"Output written to: {output_file}")


if __name__ == "__main__":
    input_path = "src-tauri/knowledge/Calculus/question-bank.json"
    output_path = "src-tauri/knowledge/Calculus/question-bank-cs-format.json"

    convert_question_bank(input_path, output_path)
