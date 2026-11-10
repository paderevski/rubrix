import json
import subprocess
import base64
from pathlib import Path

# Only import weasyprint if we need PDF
weasyprint = None


def get_weasyprint():
    global weasyprint
    if weasyprint is None:
        from weasyprint import HTML

        weasyprint = HTML
    return weasyprint


def handler(event, context):
    """
    Convert Markdown to DOCX or PDF.

    Event body (JSON):
    {
        "markdown": "# Hello World\n\nThis is content.",
        "format": "docx" | "pdf",
        "filename": "output"  (optional, default: "output")
    }

    Returns base64-encoded file content.
    """
    try:
        # Parse input
        body = (
            json.loads(event.get("body", "{}"))
            if isinstance(event.get("body"), str)
            else event
        )

        markdown_content = body.get("markdown", "")
        output_format = body.get("format", "docx").lower()
        filename = body.get("filename", "output")

        if not markdown_content:
            return error_response(400, "Missing 'markdown' field")

        if output_format not in ("docx", "pdf"):
            return error_response(400, "Format must be 'docx' or 'pdf'")

        # Set up paths in /tmp
        input_path = Path("/tmp/input.md")
        input_path.write_text(markdown_content)

        reference_docx_path = None
        reference_docx_b64 = body.get("reference_docx_base64")
        use_reference_docx = body.get("use_reference_docx", True)

        print(
            "[convert] request",
            {
                "format": output_format,
                "use_reference_docx": bool(use_reference_docx),
                "has_reference_docx_base64": bool(reference_docx_b64),
                "markdown_chars": len(markdown_content),
            },
        )

        if output_format == "docx" and use_reference_docx and reference_docx_b64:
            try:
                reference_docx_bytes = base64.b64decode(reference_docx_b64)
            except Exception as e:
                return error_response(
                    400, f"Invalid reference/template docx base64: {e}"
                )

            reference_docx_path = Path("/tmp/reference.docx")
            reference_docx_path.write_bytes(reference_docx_bytes)
            print(
                "[convert] reference_docx_loaded",
                {
                    "bytes": len(reference_docx_bytes),
                    "path": str(reference_docx_path),
                },
            )

        if output_format == "docx":
            output_path = Path(f"/tmp/{filename}.docx")
            convert_to_docx(input_path, output_path, reference_docx_path)
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        else:
            output_path = Path(f"/tmp/{filename}.pdf")
            convert_to_pdf(input_path, output_path)
            content_type = "application/pdf"

        # Read and encode output
        output_bytes = output_path.read_bytes()
        output_b64 = base64.b64encode(output_bytes).decode("utf-8")

        return {
            "statusCode": 200,
            "headers": {
                "Content-Type": content_type,
                "Content-Disposition": f'attachment; filename="{output_path.name}"',
            },
            "body": output_b64,
            "isBase64Encoded": True,
        }

    except subprocess.CalledProcessError as e:
        return error_response(500, f"Conversion failed: {e.stderr}")
    except Exception as e:
        return error_response(500, str(e))


def convert_to_docx(
    input_path: Path, output_path: Path, reference_docx_path: Path | None = None
):
    """Convert markdown to DOCX using Pandoc."""
    cmd = ["pandoc", str(input_path), "-o", str(output_path)]
    if reference_docx_path is not None:
        cmd.extend(["--reference-doc", str(reference_docx_path)])

    print(
        "[convert] pandoc_cmd",
        {
            "uses_reference_docx": reference_docx_path is not None,
            "cmd": cmd,
        },
    )

    subprocess.run(cmd, check=True, capture_output=True, text=True)


def convert_to_pdf(input_path: Path, output_path: Path):
    """Convert markdown to PDF via HTML intermediate."""
    html_path = Path("/tmp/intermediate.html")

    # Step 1: Markdown -> HTML with Pandoc (standalone for proper styling)
    subprocess.run(
        ["pandoc", str(input_path), "-s", "-o", str(html_path)],
        check=True,
        capture_output=True,
        text=True,
    )

    # Step 2: HTML -> PDF with WeasyPrint
    HTML = get_weasyprint()
    HTML(filename=str(html_path)).write_pdf(str(output_path))


def error_response(status_code: int, message: str):
    return {
        "statusCode": status_code,
        "headers": {"Content-Type": "application/json"},
        "body": json.dumps({"error": message}),
    }
