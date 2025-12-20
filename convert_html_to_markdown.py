#!/usr/bin/env python3
"""
Convert HTML tags in Calculus question bank to markdown.
"""

import json
import re


def html_to_markdown(text):
    """Convert common HTML tags to markdown."""
    if not text:
        return text

    # Remove <p> tags - markdown doesn't need them
    text = re.sub(r"<p>(.*?)</p>", r"\1", text, flags=re.DOTALL)

    # Convert <em> to *italic*
    text = re.sub(r"<em>(.*?)</em>", r"*\1*", text, flags=re.DOTALL)

    # Convert <strong> to **bold**
    text = re.sub(r"<strong>(.*?)</strong>", r"**\1**", text, flags=re.DOTALL)

    # Convert <i> to *italic*
    text = re.sub(r"<i>(.*?)</i>", r"*\1*", text, flags=re.DOTALL)

    # Convert <b> to **bold**
    text = re.sub(r"<b>(.*?)</b>", r"**\1**", text, flags=re.DOTALL)

    # Handle tables - convert to markdown table format
    # This is more complex, but for now just note them
    if "<table>" in text:
        print(f"Warning: Found table in text, may need manual conversion: {text[:100]}")

    return text.strip()


def convert_question_bank(input_file, output_file):
    """Convert HTML in question bank to markdown."""

    # Read the file
    with open(input_file, "r", encoding="utf-8") as f:
        data = json.load(f)

    converted_count = 0

    # Process each question
    for question in data.get("questions", []):
        content = question.get("content", {})

        # Convert text field
        if "text" in content and content["text"]:
            original = content["text"]
            content["text"] = html_to_markdown(content["text"])
            if original != content["text"]:
                converted_count += 1

        # Convert explanation field
        if "explanation" in content and content["explanation"]:
            original = content["explanation"]
            content["explanation"] = html_to_markdown(content["explanation"])
            if original != content["explanation"]:
                converted_count += 1

    # Write output
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

    print(f"Converted {converted_count} fields")
    print(f"Output written to: {output_file}")


if __name__ == "__main__":
    input_path = "src-tauri/knowledge/Calculus/question-bank.json"
    output_path = "src-tauri/knowledge/Calculus/question-bank.json"

    convert_question_bank(input_path, output_path)
