#!/usr/bin/env python

import re
import sys
import os

import argparse
import random
import html

import xml.etree.ElementTree as ET
import pypandoc


# Define the class structures
class Choice:
    """Represents a choice for a multiple-choice question."""

    def __init__(self, text, correct=False):
        """
        Initialize a Choice object.

        Args:
            text (str): The text of the choice.
            correct (bool, optional): Whether the choice is correct. Defaults to False.
        """
        self.text = text
        self.correct = correct


class Question:
    """Represents a multiple-choice question."""

    def __init__(self, text, choices=[]):
        """
        Initialize a Question object.

        Args:
            text (str): The text of the question.
            choices (list, optional): A list of Choice objects representing the choices. Defaults to an empty list.
        """
        self.text = text
        self.choices = choices

    def add_choice(self, choice):
        """
        Add a choice to the question.

        Args:
            choice (Choice): The Choice object to add to the question.
        """
        self.choices.append(choice)


def format_truth_tables(text):
    """
    Finds markdown tables in code blocks and converts them to LaTeX tabular environments.
    """
    # Regex to find a markdown table within a ``` block
    table_regex = re.compile(r"```\s*\n(.*?)\n```", re.DOTALL)

    def replace_with_latex_table(match):
        table_content = match.group(1).strip()
        lines = table_content.split("\n")

        # Skip the separator line '---|---|...'
        header = lines[0]
        rows = [line for line in lines[1:] if not re.match(r"^[\s\-:|]+$", line)]

        # Process header
        header = header.strip()
        if header[0] == "|":
            header = header[1:]
        if header[-1] == "|":
            header = header[:-1]
        header_cells = [cell.strip() for cell in header.split("|")]
        num_cols = len(header_cells)
        latex_header = " & ".join(header_cells) + r" \\ \hline"

        # Process rows
        latex_rows = []
        for row in rows:
            row = row.strip()
            if row[0] == "|":
                row = row[1:]
            if row[-1] == "|":
                row = row[:-1]
            cells = [cell.strip() for cell in row.split("|")]
            latex_rows.append(" & ".join(cells) + r" \\ \hline")

        # Assemble the LaTeX table
        table_format = "|".join(["c"] * num_cols)
        latex_table = (
            r"\begin{center}"
            rf"\begin{{tabular}}{{|{table_format}|}}\hline "
            f"{latex_header}\n" + "\n".join(latex_rows) + r"\end{tabular}"
            r"\end{center}"
        )
        return latex_table

    return table_regex.sub(replace_with_latex_table, text)


def convert_question_to_markup(question, args):
    """
    Convert a question block into markup format.

    Args:
        question (str): The question block to convert.

    Returns:
        str: The question block converted into markup format.
    """
    if args.type == 1:
        parts = re.split(r"\n\s*[a-z]\.\s+", question)
    elif args.type == 2:
        parts = re.split(r"\n\s*\([A-Z]\)\s+", question)
    question_text = parts[0].strip()
    choices = parts[1:]

    # Format truth tables before any other conversion
    question_text = format_truth_tables(question_text)

    markup_tex = html.escape(pypandoc.convert_text(question_text, "latex", format="md"))

    markup = f"<Q>\n<t>{markup_tex}</t>\n"
    for index, choice in enumerate(choices):
        escaped_choice = html.escape(
            pypandoc.convert_text(choice, "latex", format="md")
        )
        if index == 0:  # Mark the first choice as the correct one
            markup += f"<CC> {escaped_choice} </CC>\n"
        else:
            markup += f"<c> {escaped_choice} </c>\n"
    markup += "</Q>\n"
    return markup


def parse_markup(markup_text):
    """
    Parse markup text into Question objects.

    Args:
        markup_text (str): The markup text to parse.

    Returns:
        list: A list of Question objects parsed from the markup text.
    """
    root = ET.fromstring("<root>" + markup_text + "</root>")

    questions = []

    for q_elem in root.findall("Q"):
        question_text = q_elem.find("t").text
        choices = []

        for c_elem in q_elem.findall("c"):
            choices.append(Choice(text=c_elem.text, correct=False))
        for cc_elem in q_elem.findall("CC"):
            choices.append(Choice(text=cc_elem.text, correct=True))

        question = Question(text=question_text, choices=choices)
        questions.append(question)

    return questions


def decode_xml_entities(xml_string):
    """Decodes common XML entities in a string to their character equivalents."""
    entity_replacements = {"&lt;": "<", "&gt;": ">", "&amp;": "&"}
    for entity, char in entity_replacements.items():
        xml_string = xml_string.replace(entity, char)
    return xml_string


def xml2markdown(questions):
    """
    Convert parsed questions to Markdown format.

    Args:
        questions (list): A list of Question objects.

    Returns:
        None
    """
    outfile = open("/tmp/test-new.md", "w")
    for question in questions:
        question.text = html.unescape(question.text)
        outfile.write("1.  " + question.text + "\n")
        for choice in question.choices:
            choice.text = html.unescape(choice.text)
            if choice.correct:
                decoration = "\u2713"
            else:
                decoration = ""
            outfile.write(f"    a.  {choice.text} {decoration}" + "\n")
    outfile.close()


def make_key(questions, args):
    """
    Generate a key for the questions.

    Args:
        questions (list): A list of Question objects.

    Returns:
        str: The generated key.
    """
    output = ""
    if args.make_key:
        output = "\\clearpage\n"
        output += "\\textbf{KEY}\n"
        output += "\\begin{enumerate}\n"

        letters = ["A", "B", "C", "D", "E"]
        for n, q in enumerate(questions):
            for i, c in enumerate(q.choices):
                if c.correct:
                    output += "\t\\item " + letters[i] + "\n"
        output += "\\end{enumerate}\n"
    return output


def convert_xml_to_latex(questions, args, template_path="template.tex"):
    """Converts a list of Question objects to LaTeX format using a template file.

    Args:
        questions (list of Question): The questions to convert.
        args: The command line arguments object.
        template_path (str): Path to the LaTeX template file.
    """
    # Read the template file
    with open(template_path, "r") as file:
        latex_template = file.read()

    # Generate LaTeX code for questions
    questions_latex = "\\begin{enumerate}\n\t\\itemsep0.2em\n"
    for question in questions:
        question.text = html.unescape(question.text)
        questions_latex += "\t\\item\n\t\\begin{minipage}[t]{\\linewidth}\n"
        questions_latex += "\t\t" + question.text + "\n\n"
        questions_latex += "\t\t" + "\\vspace{1em}\n\n"
        questions_latex += "\t\t\\begin{enumerate}\n\t\t\\setlength\\itemsep{0.25em}\n"
        for choice in question.choices:
            choice.text = html.unescape(choice.text)
            # choice.text = choice.text.replace("\n", "\\\\ ")
            if not choice.correct:
                questions_latex += "\t\t\t\\item " + choice.text + "\n"
            elif choice.correct and args.mark_correct:
                questions_latex += "\t\t\t\\item (*) " + choice.text + "\n"
            elif choice.correct and not args.mark_correct:
                questions_latex += "\t\t\t\\item " + choice.text + "\n"
        if args.none_above:
            questions_latex += "\t\t\t\\item  None of the above \n"
        questions_latex += "\t\t\\end{enumerate}\n"
        questions_latex += "\t\\end{minipage}\n"
    questions_latex += "\\end{enumerate}\n"

    # Replace placeholders in the ßtemplate
    final_latex = latex_template.replace("%%QUESTIONS%%", questions_latex)
    final_latex = final_latex.replace("%%KEY%%", make_key(questions, args))
    final_latex = final_latex.replace("%%TESTID%%", str(args.rand_seed))

    # Save the final LaTeX code to a new file
    output_path = "test-temp.tex"
    with open(output_path, "w") as file:
        file.write(final_latex)

    return output_path


def parse_arguments():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Convert questions from markdown to LaTeX."
    )
    parser.add_argument("--input", "-i", type=str, help="Input filename", required=True)
    parser.add_argument(
        "--rand_seed", "-s", type=int, help="Random seed for shuffling", default=2048
    )
    parser.add_argument(
        "--output",
        "-o",
        type=str,
        help="Filename for the output PDF file",
        default="Test-Output",
    )
    parser.add_argument(
        "--shuffle_questions",
        "-sq",
        action="store_true",
        help="Shuffle questions globally",
    )
    parser.add_argument(
        "--shuffle_choices",
        "-sc",
        action="store_true",
        help="Shuffle multiple choice options",
    )
    parser.add_argument(
        "--mark_correct", "-m", action="store_true", help="Mark correct answers"
    )
    parser.add_argument(
        "--make_key", "-k", action="store_true", help="Include LaTeX key in the output"
    )
    parser.add_argument(
        "--none_above",
        "-na",
        action="store_true",
        help="Add none of the above as a choice",
    )
    parser.add_argument(
        "--type",
        type=int,
        default=1,
        help="1: choices use a.b.c 2: choices use (A)(B)(C)",
    )
    return parser.parse_args()


def process_markdown_file(input_filename):
    """Process the input markdown file."""
    with open(input_filename, "r") as file:
        content = file.readlines()
    content = [line for line in content if not line.startswith(("#", ";"))]
    return "".join(content)


def generate_questions(markdown_text, args):
    """Generate question objects from markdown text."""
    body = re.split(r"\n+\-\-\-", markdown_text)  # find text before "---"
    questions_text = re.split(r"\n+\d+\.\s+", body[0])  # split at question numbers
    questions_text = [question for question in questions_text if len(question) > 3]

    markup_text = "\n".join(
        [
            convert_question_to_markup(question, args)
            for question in questions_text
            if question
        ]
    )

    with open("/tmp/question_bank.txt", "w") as f:
        f.write(markup_text)

    return parse_markup(markup_text)


def shuffle_questions_if_needed(questions, args):
    """Shuffle questions and choices if specified."""
    if args.shuffle_questions:
        random.shuffle(questions)
    if args.shuffle_choices:
        for question in questions:
            random.shuffle(question.choices)
    return questions


def compile_latex_to_pdf(latex_code, args):
    os.system("pdflatex test-temp.tex")
    os.system(
        f"pdfunite CoverPage.pdf test-temp.pdf {args.output}-{args.rand_seed}.pdf"
    )


def cleanup_temp_files():
    os.system("rm test-temp.*")


def main():
    args = parse_arguments()
    random.seed(args.rand_seed)

    # Process the markdown file and generate question objects
    markdown_text = process_markdown_file(args.input)
    questions = generate_questions(markdown_text, args)
    questions = shuffle_questions_if_needed(questions, args)

    # Convert questions to LaTeX and compile the PDF
    latex_code = convert_xml_to_latex(questions, args)
    compile_latex_to_pdf(latex_code, args)

    # Clean up temporary files
    # cleanup_temp_files()


if __name__ == "__main__":
    main()
