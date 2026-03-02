//! QTI export functionality - generates IMS QTI XML for LMS import

use crate::Question;
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::io::Write;
use zip::write::FileOptions;
use zip::ZipWriter;

// ============================================================================
// Templates
// ============================================================================

const MANIFEST_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<manifest xmlns="http://www.imsglobal.org/xsd/imsccv1p2/imscp_v1p1" identifier="cctd0001"
    xmlns:lom="http://ltsc.ieee.org/xsd/imsccv1p2/LOM/resource"
    xmlns:lomimscc="http://ltsc.ieee.org/xsd/imsccv1p2/LOM/manifest"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <metadata>
        <schema>IMS Common Cartridge</schema>
        <schemaversion>1.2.0</schemaversion>
        <lomimscc:lom>
            <lomimscc:general>
                <lomimscc:title>
                    <lomimscc:string>{TITLE}</lomimscc:string>
                </lomimscc:title>
            </lomimscc:general>
        </lomimscc:lom>
    </metadata>
    <organizations>
        <organization identifier="org" structure="rooted-hierarchy">
            <item identifier="root">
                <item identifier="iden0000001" identifierref="ccres0000001">
                    <title>{TITLE}</title>
                </item>
            </item>
        </organization>
    </organizations>
    <resources>
        <resource identifier="ccres0000001" type="imsqti_xmlv1p2/imscc_xmlv1p2/question-bank">
            <metadata />
            <file href="{XML_FILE_NAME}" />
        </resource>
    </resources>
</manifest>
"#;

const QTI_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<questestinterop xmlns="http://www.imsglobal.org/xsd/ims_qtiasiv1p2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.imsglobal.org/xsd/ims_qtiasiv1p2 http://www.imsglobal.org/profile/cc/ccv1p2/ccv1p2_qtiasiv1p2p1_v1p0.xsd">
    <objectbank ident="test_bank">"#;

const QTI_FOOTER: &str = r#"    </objectbank>
</questestinterop>"#;

const QTI_ITEM_TEMPLATE: &str = r#"        <item ident="{item_id}">
            <itemmetadata>
                <qtimetadata>
                    <qtimetadatafield>
                        <fieldlabel>cc_profile</fieldlabel>
                        <fieldentry>cc.multiple_choice.v0p1</fieldentry>
                    </qtimetadatafield>
                </qtimetadata>
            </itemmetadata>
            <presentation>
                <material>
                    <mattext texttype="text/html"><![CDATA[{question_html}]]></mattext>
                </material>
                <response_lid ident="{item_id}" rcardinality="Single">
                    <render_choice shuffle="Yes">
{choices}
                    </render_choice>
                </response_lid>
            </presentation>
            <resprocessing>
                <outcomes>
                    <decvar maxvalue="100" minvalue="0" varname="SCORE" vartype="Decimal"/>
                </outcomes>
                <respcondition continue="No">
                    <conditionvar>
                        <varequal respident="{item_id}">{correct_id}</varequal>
                    </conditionvar>
                    <setvar action="Set" varname="SCORE">100</setvar>
                </respcondition>
            </resprocessing>
        </item>"#;

const CHOICE_TEMPLATE: &str = r#"                        <response_label ident="{choice_id}">
                            <material>
                                <mattext texttype="text/html">{choice_text}</mattext>
                            </material>
                        </response_label>"#;

// ============================================================================
// Text Processing Functions
// ============================================================================

/// Clean up encoding artifacts and special characters
fn clean_special_characters(text: &str) -> String {
    text
        // Encoding artifacts
        .replace("Â", "") // Common UTF-8 artifact
        .replace("\u{00a0}", " ") // Non-breaking space
        .replace("\u{feff}", "") // BOM
        // Curly quotes to straight quotes
        .replace("\u{201c}", "\"") // Left double quote
        .replace("\u{201d}", "\"") // Right double quote
        .replace("\u{2018}", "'") // Left single quote
        .replace("\u{2019}", "'") // Right single quote
        .replace("\u{201e}", "\"") // Double low-9 quote
        .replace("\u{201f}", "\"") // Double high-reversed-9 quote
        // Dashes
        .replace("\u{2013}", "-") // En dash
        .replace("\u{2014}", "-") // Em dash
        // Other common issues
        .replace("\u{2026}", "...") // Ellipsis
        .replace("\t", "    ") // Tab to spaces
}

/// Convert markdown table to HTML table
fn convert_markdown_table(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.trim().lines().collect();

    if lines.len() < 2 {
        return markdown.to_string();
    }

    let mut html = String::from("<table border=\"1\" cellpadding=\"5\" cellspacing=\"0\">\n");

    // Parse header row
    let header_cells: Vec<&str> = lines[0]
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if !header_cells.is_empty() {
        html.push_str("<thead><tr>\n");
        for cell in header_cells {
            let cell_html = convert_inline_code(&htmlescape::encode_minimal(cell));
            html.push_str(&format!("<th>{}</th>\n", cell_html));
        }
        html.push_str("</tr></thead>\n");
    }

    // Skip separator line (lines[1])

    // Parse data rows
    if lines.len() > 2 {
        html.push_str("<tbody>\n");
        for row in &lines[2..] {
            let cells: Vec<&str> = row
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if !cells.is_empty() {
                html.push_str("<tr>\n");
                for cell in cells {
                    let cell_html = convert_inline_code(&htmlescape::encode_minimal(cell));
                    html.push_str(&format!("<td>{}</td>\n", cell_html));
                }
                html.push_str("</tr>\n");
            }
        }
        html.push_str("</tbody>\n");
    }

    html.push_str("</table>");
    html
}

/// If a markdown table is wrapped in a fenced code block, unwrap it so pandoc renders it as a table.
fn convert_codeblock_tables_to_markdown(text: &str) -> String {
    let code_block_re = Regex::new(r"```[^\n]*\n(?P<body>[\s\S]*?)\n```\s*").unwrap();

    code_block_re
        .replace_all(text, |caps: &regex::Captures| {
            let body = caps.name("body").map(|m| m.as_str()).unwrap_or("");
            if is_markdown_table(body) {
                body.trim().to_string()
            } else {
                caps.get(0)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default()
            }
        })
        .into_owned()
}

fn is_markdown_table(body: &str) -> bool {
    let mut lines = body.lines().filter(|l| !l.trim().is_empty());
    if let (Some(header), Some(separator)) = (lines.next(), lines.next()) {
        let looks_like_header = header.contains('|');
        let looks_like_separator = separator
            .trim()
            .chars()
            .all(|c| matches!(c, '|' | '-' | ':' | ' '));
        looks_like_header && looks_like_separator
    } else {
        false
    }
}

fn indent_block(text: &str, first_prefix: &str, rest_prefix: &str) -> String {
    let mut lines = text.lines();
    if let Some(first) = lines.next() {
        let mut out = String::new();
        out.push_str(first_prefix);
        out.push_str(first.trim());
        for line in lines {
            out.push('\n');
            out.push_str(rest_prefix);
            out.push_str(line.trim());
        }
        out
    } else {
        String::new()
    }
}

fn normalize_math_delimiters(content: &str) -> String {
    if content.is_empty() {
        return String::new();
    }
    content
        .replace(r"\(", "$")
        .replace(r"\)", "$")
        .replace(r"\[", "$$")
        .replace(r"\]", "$$")
}

/// Convert answer text to HTML with proper escaping and line break preservation
fn convert_answer_to_html(answer: &str) -> String {
    let display_latex_re = Regex::new(r"\$\$([^\$]+)\$\$").unwrap();
    let inline_latex_re = Regex::new(r"\$([^\$]+)\$").unwrap();

    let lines: Vec<&str> = answer.lines().collect();
    let mut html_lines = Vec::new();

    for line in lines {
        let mut processed = String::new();
        let mut last_end = 0;
        let mut latex_blocks: Vec<(usize, usize, String)> = Vec::new();

        // Collect display LaTeX ($$...$$)
        for cap in display_latex_re.captures_iter(line) {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();
            let formula = cap.get(1).unwrap().as_str().trim();
            let encoded = urlencoding::encode(formula);
            let html = format!(
                r#"<img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex={}" alt="{}" formula="{}" class="mathquill-formula" />"#,
                encoded, formula, formula
            );
            latex_blocks.push((start, end, html));
        }

        // Collect inline LaTeX ($...$)
        for cap in inline_latex_re.captures_iter(line) {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();

            // Skip if already part of display LaTeX
            let is_inside_block = latex_blocks
                .iter()
                .any(|(block_start, block_end, _)| start >= *block_start && end <= *block_end);

            if !is_inside_block {
                let formula = cap.get(1).unwrap().as_str().trim();
                let encoded = urlencoding::encode(formula);
                let html = format!(
                    r#"<img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex={}" alt="{}" formula="{}" class="mathquill-formula" />"#,
                    encoded, formula, formula
                );
                latex_blocks.push((start, end, html));
            }
        }

        // Sort by position
        latex_blocks.sort_by_key(|&(start, _, _)| start);

        // Build the line with LaTeX and escaped text
        for (start, end, html) in latex_blocks {
            if last_end < start {
                let text_part = &line[last_end..start];
                let escaped = htmlescape::encode_minimal(text_part);
                let with_code = convert_inline_code(&escaped);
                processed.push_str(&with_code);
            }
            processed.push_str(&html);
            last_end = end;
        }

        // Add remaining text
        if last_end < line.len() {
            let text_part = &line[last_end..];
            let escaped = htmlescape::encode_minimal(text_part);
            let with_code = convert_inline_code(&escaped);
            processed.push_str(&with_code);
        }

        html_lines.push(processed);
    }

    // Join with <br /> for line breaks
    html_lines.join("<br />")
}

// ============================================================================
// Export Functions
// ============================================================================

/// Export questions to our intermediate .txt format
pub fn export_md(title: &str, questions: &[Question]) -> Result<String, String> {
    let mut output = format!("# {}\n\n", title);
    let mut rng = thread_rng();
    let mut answer_key: Vec<(usize, char)> = Vec::new();

    for (i, q) in questions.iter().enumerate() {
        let question_text =
            convert_codeblock_tables_to_markdown(&normalize_math_delimiters(q.text.trim()));
        output.push_str(&format!("**Question {}.** {}\n\n", i + 1, question_text));

        let mut answers = q.answers.clone();
        answers.shuffle(&mut rng);

        for (idx, answer) in answers.iter().enumerate() {
            let label = (b'A' + idx as u8) as char;
            let body = convert_codeblock_tables_to_markdown(&normalize_math_delimiters(
                answer.text.trim(),
            ));
            let formatted = format!("{}) {}", label, body);
            output.push_str(&formatted);
            output.push('\n');
        }

        if let Some((idx, _)) = answers.iter().enumerate().find(|(_, a)| a.is_correct) {
            let label = (b'A' + idx as u8) as char;
            answer_key.push((i + 1, label));
        }

        output.push('\n');
    }

    if !answer_key.is_empty() {
        output.push_str("## Answers\n\n");
        for (number, label) in answer_key {
            output.push_str(&format!("{}. {}\n", number, label));
        }
    }

    Ok(output)
}

/// Backward compatibility: keep TXT export entry point but emit markdown content.
pub fn export_txt(title: &str, questions: &[Question]) -> Result<String, String> {
    export_md(title, questions)
}

/// Export questions to QTI ZIP format (ready for Schoology)
pub fn export_qti_zip(title: &str, questions: &[Question]) -> Result<Vec<u8>, String> {
    let xml_filename = format!("{}.xml", sanitize_filename(title));

    // Generate QTI XML
    let qti_xml = generate_qti_xml(title, questions);

    // Generate manifest
    let manifest = MANIFEST_TEMPLATE
        .replace("{TITLE}", title)
        .replace("{XML_FILE_NAME}", &xml_filename);

    // Create ZIP in memory
    let mut buffer = Vec::new();
    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buffer));
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Add QTI XML
        zip.start_file(&xml_filename, options)
            .map_err(|e| format!("Failed to add XML: {}", e))?;
        zip.write_all(qti_xml.as_bytes())
            .map_err(|e| format!("Failed to write XML: {}", e))?;

        // Add manifest
        zip.start_file("imsmanifest.xml", options)
            .map_err(|e| format!("Failed to add manifest: {}", e))?;
        zip.write_all(manifest.as_bytes())
            .map_err(|e| format!("Failed to write manifest: {}", e))?;

        zip.finish()
            .map_err(|e| format!("Failed to finalize ZIP: {}", e))?;
    }

    Ok(buffer)
}

/// Generate the QTI XML content
fn generate_qti_xml(_title: &str, questions: &[Question]) -> String {
    let mut items = Vec::new();

    for (i, q) in questions.iter().enumerate() {
        let item_id = format!("{}", i + 1);
        let question_html = convert_to_html(q);

        // Generate choices
        let mut choices = Vec::new();
        let mut correct_id = String::from("1");

        // Put correct answer first, track its ID
        let mut choice_num = 1;
        for answer in &q.answers {
            if answer.is_correct {
                correct_id = choice_num.to_string();
            }

            // Clean special characters, convert LaTeX, then convert to HTML with proper escaping
            let cleaned = clean_special_characters(&answer.text);
            let processed = convert_latex(&cleaned);
            let answer_html = convert_answer_to_html(&processed);
            let choice = CHOICE_TEMPLATE
                .replace("{choice_id}", &choice_num.to_string())
                .replace("{choice_text}", &answer_html);
            choices.push(choice);
            choice_num += 1;
        }

        let item = QTI_ITEM_TEMPLATE
            .replace("{item_id}", &item_id)
            .replace("{question_html}", &question_html)
            .replace("{choices}", &choices.join("\n"))
            .replace("{correct_id}", &correct_id);

        items.push(item);
    }

    format!("{}\n{}\n{}", QTI_HEADER, items.join("\n"), QTI_FOOTER)
}

/// Convert a question to HTML format
fn convert_to_html(q: &Question) -> String {
    // Clean special characters first
    let text = clean_special_characters(&q.text);

    // Parse markdown: extract code blocks, tables, LaTeX, and convert rest
    let code_block_re = Regex::new(r"```(?:java)?\n([^`]+)\n```").unwrap();
    let table_re = Regex::new(r"(\|[^\n]+\|\n)(\|[-:\s|]+\|\n)((?:\|[^\n]+\|\n?)+)").unwrap();
    let display_latex_re = Regex::new(r"\$\$([^\$]+)\$\$").unwrap();
    let inline_latex_re = Regex::new(r"\$([^\$]+)\$").unwrap();

    #[derive(Debug)]
    enum Block {
        BlockLevel(usize, usize, String), // start, end, html
        Inline(usize, usize, String),     // start, end, html
    }

    let mut blocks: Vec<Block> = Vec::new();

    // Collect code blocks (block-level)
    for cap in code_block_re.captures_iter(&text) {
        let start: usize = cap.get(0).unwrap().start();
        let end: usize = cap.get(0).unwrap().end();
        let code: &str = cap.get(1).unwrap().as_str();
        let html: String = format!("<pre>{}</pre>", htmlescape::encode_minimal(code.trim()));
        blocks.push(Block::BlockLevel(start, end, html));
    }

    // Collect tables (block-level)
    for cap in table_re.captures_iter(&text) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();
        let table_md = cap.get(0).unwrap().as_str();
        let html = convert_markdown_table(table_md);
        blocks.push(Block::BlockLevel(start, end, html));
    }

    // Collect display LaTeX (block-level) - do this before inline to avoid conflicts
    for cap in display_latex_re.captures_iter(&text) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();
        let formula = cap.get(1).unwrap().as_str().trim();
        let encoded = urlencoding::encode(formula);
        let html = format!(
            r#"<img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex={}" alt="{}" formula="{}" class="mathquill-formula" />"#,
            encoded, formula, formula
        );
        blocks.push(Block::BlockLevel(start, end, html));
    }

    // Collect inline LaTeX (inline)
    for cap in inline_latex_re.captures_iter(&text) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();

        // Skip if this LaTeX is already part of another block
        let is_inside_block = blocks.iter().any(|block| {
            let (block_start, block_end) = match block {
                Block::BlockLevel(s, e, _) => (s, e),
                Block::Inline(s, e, _) => (s, e),
            };
            start >= *block_start && end <= *block_end
        });

        if !is_inside_block {
            let formula = cap.get(1).unwrap().as_str().trim();
            let encoded = urlencoding::encode(formula);
            let html = format!(
                r#"<img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex={}" alt="{}" formula="{}" class="mathquill-formula" />"#,
                encoded, formula, formula
            );
            blocks.push(Block::Inline(start, end, html));
        }
    }

    // Sort by position
    blocks.sort_by_key(|block| match block {
        Block::BlockLevel(start, _, _) => *start,
        Block::Inline(start, _, _) => *start,
    });

    // Build HTML, grouping inline content into paragraphs
    let mut result = Vec::new();
    let mut last_end = 0;
    let mut paragraph_parts: Vec<String> = Vec::new();

    for block in blocks {
        let (start, end, html, is_block_level) = match block {
            Block::BlockLevel(s, e, h) => (s, e, h, true),
            Block::Inline(s, e, h) => (s, e, h, false),
        };

        // Add text before this block
        if last_end < start {
            let text_part = &text[last_end..start];
            if !text_part.trim().is_empty() {
                let escaped = htmlescape::encode_minimal(text_part.trim());
                let processed = convert_inline_code(&escaped);
                paragraph_parts.push(processed);
            }
        }

        if is_block_level {
            // Flush any accumulated paragraph content
            if !paragraph_parts.is_empty() {
                result.push(format!("<p>{}</p>", paragraph_parts.join(" ")));
                paragraph_parts.clear();
            }
            // Add the block-level element directly
            result.push(html);
        } else {
            // Add inline element to current paragraph
            paragraph_parts.push(html);
        }

        last_end = end;
    }

    // Add remaining text after last block
    if last_end < text.len() {
        let text_part = &text[last_end..];
        if !text_part.trim().is_empty() {
            let escaped = htmlescape::encode_minimal(text_part.trim());
            let processed = convert_inline_code(&escaped);
            paragraph_parts.push(processed);
        }
    }

    // Flush any remaining paragraph content
    if !paragraph_parts.is_empty() {
        result.push(format!("<p>{}</p>", paragraph_parts.join(" ")));
    }

    result.join("\n")
}

/// Convert LaTeX blocks to img tags with learn.lcps.org API
fn convert_latex(text: &str) -> String {
    // First handle display math ($$...$$)
    let display_re = Regex::new(r"\$\$([^\$]+)\$\$").unwrap();
    let text = display_re.replace_all(text, |caps: &regex::Captures| {
        let formula = caps.get(1).unwrap().as_str().trim();
        let encoded = urlencoding::encode(formula);
        format!(
            r#"<img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex={}" alt="{}" formula="{}" class="mathquill-formula" />"#,
            encoded, formula, formula
        )
    });

    // Then handle inline math ($...$)
    let inline_re = Regex::new(r"\$([^\$]+)\$").unwrap();
    inline_re.replace_all(&text, |caps: &regex::Captures| {
        let formula = caps.get(1).unwrap().as_str().trim();
        let encoded = urlencoding::encode(formula);
        format!(
            r#"<img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex={}" alt="{}" formula="{}" class="mathquill-formula" />"#,
            encoded, formula, formula
        )
    }).to_string()
}

/// Convert backticks to <code> tags
fn convert_inline_code(text: &str) -> String {
    let re = Regex::new(r"`([^`]+)`").unwrap();
    re.replace_all(text, "<code>$1</code>").to_string()
}

/// Sanitize a string for use as a filename
fn sanitize_filename(name: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
    re.replace_all(name, "_").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Answer;

    #[test]
    fn test_export_txt() {
        let questions = vec![Question {
            id: "1".to_string(),
            text: "What is 2+2?".to_string(),
            explanation: None,
            distractors: None,
            subject: "Test".to_string(),
            topics: vec!["Math".to_string()],
            answers: vec![
                Answer {
                    text: "4".to_string(),
                    is_correct: true,
                    explanation: None,
                },
                Answer {
                    text: "3".to_string(),
                    is_correct: false,
                    explanation: None,
                },
            ],
        }];

        let result = export_txt("Test", &questions).unwrap();
        assert!(result.contains("# Test"));
        assert!(result.contains("**Question 1.**"));
        assert!(result.contains("What is 2+2?"));
        // Both answers must appear (shuffled order); match the ") <answer>" format
        assert!(result.contains(") 4"));
        assert!(result.contains(") 3"));
    }

    #[test]
    fn test_convert_latex_inline() {
        let input = "What is $3x-sin\\left(x\\right)+\\sqrt{x}$ equal to?";
        let result = convert_latex(input);
        assert!(result.contains("https://learn.lcps.org/svc/latex/latex-to-svg?latex="));
        assert!(result.contains(r#"alt="3x-sin\left(x\right)+\sqrt{x}""#));
        assert!(result.contains(r#"formula="3x-sin\left(x\right)+\sqrt{x}""#));
        assert!(result.contains("class=\"mathquill-formula\""));
    }

    #[test]
    fn test_convert_latex_display() {
        let input = "An equation: $$\\large 3x-sin\\left(x\\right)+\\sqrt{x}$$";
        let result = convert_latex(input);
        assert!(result.contains("https://learn.lcps.org/svc/latex/latex-to-svg?latex="));
        assert!(result.contains(r#"alt="\large 3x-sin\left(x\right)+\sqrt{x}""#));
        assert!(result.contains("class=\"mathquill-formula\""));
    }

    #[test]
    fn test_convert_latex_url_encoding() {
        let input = "Test $\\large 3x-sin\\left(x\\right)+\\sqrt{x}$ formula";
        let result = convert_latex(input);
        // Check that special characters are URL encoded
        assert!(result.contains("%5C")); // backslash
        assert!(result.contains("%2B")); // plus sign
        assert!(result.contains("%7B")); // {
        assert!(result.contains("%7D")); // }
    }

    #[test]
    fn test_latex_with_primes_not_html_encoded() {
        // Test that primes (apostrophes) in LaTeX are NOT HTML encoded
        let q = Question {
            id: "1".to_string(),
            text: "Find $F'(x)$ and $g''(t)$".to_string(),
            explanation: None,
            distractors: None,
            subject: "Calculus".to_string(),
            topics: vec!["Derivatives".to_string()],
            answers: vec![],
        };

        let result = convert_to_html(&q);

        // The LaTeX should contain F'(x) in the URL, not F&apos;(x) or F&#39;(x)
        // %27 is URL-encoded apostrophe, %28 is (, %29 is )
        assert!(result.contains("F%27%28x%29"));
        assert!(result.contains("g%27%27%28t%29")); // double prime

        // Should NOT contain HTML entities
        assert!(!result.contains("&apos;"));
        assert!(!result.contains("&#39;"));

        // The alt and formula attributes should contain the raw LaTeX (not HTML encoded)
        assert!(result.contains(r#"alt="F'(x)""#));
        assert!(result.contains(r#"formula="F'(x)""#));
    }

    #[test]
    fn test_answer_latex_with_primes() {
        // Test that answer LaTeX also preserves primes correctly
        let cleaned_answer = "$f'(x) = 2x$";
        let result = convert_answer_to_html(cleaned_answer);

        // Should contain URL-encoded prime (and other chars)
        assert!(result.contains("f%27%28x%29")); // %27 is URL-encoded apostrophe

        // Should NOT contain HTML entities
        assert!(!result.contains("&apos;"));
        assert!(!result.contains("&#39;"));

        // The alt and formula attributes should contain the raw LaTeX
        assert!(result.contains(r#"alt="f'(x) = 2x""#));
    }

    #[test]
    fn test_inline_latex_grouped_in_paragraph() {
        // Test that inline LaTeX stays within the same paragraph as surrounding text
        let q = Question {
            id: "1".to_string(),
            text: "Find $F'(x)$ here".to_string(),
            explanation: None,
            distractors: None,
            subject: "Calculus".to_string(),
            topics: vec!["Derivatives".to_string()],
            answers: vec![],
        };

        let result = convert_to_html(&q);

        // The entire content should be in ONE paragraph
        let paragraph_count = result.matches("<p>").count();
        assert_eq!(
            paragraph_count, 1,
            "Should have exactly one paragraph, got: {}",
            result
        );

        // Should NOT have separate paragraphs for text before/after LaTeX
        assert!(
            !result.contains("<p>Find</p>"),
            "Text before LaTeX should not be in separate paragraph"
        );
        assert!(
            !result.contains("<p>here</p>"),
            "Text after LaTeX should not be in separate paragraph"
        );

        // Should have all content in one paragraph
        assert!(
            result.contains("<p>Find"),
            "Should start paragraph with 'Find'"
        );
        assert!(
            result.contains("here</p>"),
            "Should end paragraph with 'here'"
        );
    }

    #[test]
    fn test_block_latex_creates_new_paragraph() {
        // Test that display LaTeX ($$...$$) breaks paragraphs
        let q = Question {
            id: "1".to_string(),
            text: "Here is an equation: $$F'(x) = 2x$$ and more text".to_string(),
            explanation: None,
            distractors: None,
            subject: "Calculus".to_string(),
            topics: vec!["Derivatives".to_string()],
            answers: vec![],
        };

        let result = convert_to_html(&q);

        // Should have separate paragraphs before and after the display math
        assert!(
            result.contains("<p>Here is an equation:</p>"),
            "Should have paragraph before display math"
        );
        assert!(
            result.contains("<p>and more text</p>"),
            "Should have paragraph after display math"
        );

        // Display math should be its own element
        assert!(
            result.contains(r#"<img src="https://learn.lcps.org/svc/latex/latex-to-svg?latex="#)
        );
    }

    // -------------------------------------------------------------------------
    // clean_special_characters
    // -------------------------------------------------------------------------

    #[test]
    fn test_clean_special_characters_encoding_artifacts() {
        assert_eq!(clean_special_characters("Â test"), " test");
        assert_eq!(clean_special_characters("no\u{feff}bom"), "nobom");
        assert_eq!(clean_special_characters("non\u{00a0}breaking"), "non breaking");
    }

    #[test]
    fn test_clean_special_characters_curly_quotes() {
        assert_eq!(clean_special_characters("\u{201c}hello\u{201d}"), "\"hello\"");
        assert_eq!(clean_special_characters("\u{2018}it\u{2019}s"), "'it's");
    }

    #[test]
    fn test_clean_special_characters_dashes_and_ellipsis() {
        assert_eq!(clean_special_characters("A\u{2013}B"), "A-B");
        assert_eq!(clean_special_characters("A\u{2014}B"), "A-B");
        assert_eq!(clean_special_characters("wait\u{2026}"), "wait...");
    }

    #[test]
    fn test_clean_special_characters_tabs() {
        assert_eq!(clean_special_characters("a\tb"), "a    b");
    }

    #[test]
    fn test_clean_special_characters_no_change() {
        let plain = "Hello World 123";
        assert_eq!(clean_special_characters(plain), plain);
    }

    // -------------------------------------------------------------------------
    // is_markdown_table
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_markdown_table_valid() {
        let table = "| A | B |\n|---|---|\n| 1 | 2 |";
        assert!(is_markdown_table(table));
    }

    #[test]
    fn test_is_markdown_table_invalid_no_pipe() {
        let not_a_table = "Header\n------\nRow";
        assert!(!is_markdown_table(not_a_table));
    }

    #[test]
    fn test_is_markdown_table_too_short() {
        let one_line = "| A |";
        assert!(!is_markdown_table(one_line));
    }

    #[test]
    fn test_is_markdown_table_with_alignment() {
        let table = "| Left | Center | Right |\n|:-----|:------:|------:|\n| a | b | c |";
        assert!(is_markdown_table(table));
    }

    // -------------------------------------------------------------------------
    // normalize_math_delimiters
    // -------------------------------------------------------------------------

    #[test]
    fn test_normalize_math_delimiters_inline() {
        assert_eq!(normalize_math_delimiters(r"\(x+1\)"), "$x+1$");
    }

    #[test]
    fn test_normalize_math_delimiters_display() {
        assert_eq!(normalize_math_delimiters(r"\[x^2\]"), "$$x^2$$");
    }

    #[test]
    fn test_normalize_math_delimiters_mixed() {
        let input = r"Inline \(a+b\) and display \[c=d\]";
        let output = normalize_math_delimiters(input);
        assert!(output.contains("$a+b$"));
        assert!(output.contains("$$c=d$$"));
    }

    #[test]
    fn test_normalize_math_delimiters_passthrough_dollars() {
        let input = "Already $x$ and $$y$$";
        assert_eq!(normalize_math_delimiters(input), input);
    }

    #[test]
    fn test_normalize_math_delimiters_empty() {
        assert_eq!(normalize_math_delimiters(""), "");
    }

    // -------------------------------------------------------------------------
    // sanitize_filename
    // -------------------------------------------------------------------------

    #[test]
    fn test_sanitize_filename_basic() {
        assert_eq!(sanitize_filename("My Quiz"), "My_Quiz");
    }

    #[test]
    fn test_sanitize_filename_special_chars() {
        assert_eq!(sanitize_filename("Test/Quiz:2024"), "Test_Quiz_2024");
    }

    #[test]
    fn test_sanitize_filename_allowed_chars() {
        assert_eq!(sanitize_filename("valid-name_123"), "valid-name_123");
    }

    #[test]
    fn test_sanitize_filename_empty() {
        assert_eq!(sanitize_filename(""), "");
    }

    // -------------------------------------------------------------------------
    // convert_inline_code
    // -------------------------------------------------------------------------

    #[test]
    fn test_convert_inline_code_basic() {
        assert_eq!(convert_inline_code("`foo`"), "<code>foo</code>");
    }

    #[test]
    fn test_convert_inline_code_multiple() {
        let input = "Use `a` and `b`";
        let result = convert_inline_code(input);
        assert_eq!(result, "Use <code>a</code> and <code>b</code>");
    }

    #[test]
    fn test_convert_inline_code_no_backticks() {
        let input = "plain text";
        assert_eq!(convert_inline_code(input), "plain text");
    }

    // -------------------------------------------------------------------------
    // convert_markdown_table
    // -------------------------------------------------------------------------

    #[test]
    fn test_convert_markdown_table_basic() {
        let md = "| Name | Score |\n|------|-------|\n| Alice | 95 |\n| Bob | 87 |";
        let html = convert_markdown_table(md);
        assert!(html.contains("<table"));
        assert!(html.contains("<th>Name</th>"));
        assert!(html.contains("<th>Score</th>"));
        assert!(html.contains("<td>Alice</td>"));
        assert!(html.contains("<td>87</td>"));
        assert!(html.contains("</table>"));
    }

    #[test]
    fn test_convert_markdown_table_header_only() {
        // Only a header row (< 2 lines) should return input unchanged
        let md = "| A |";
        let result = convert_markdown_table(md);
        assert_eq!(result.trim(), md.trim());
    }

    // -------------------------------------------------------------------------
    // export_md
    // -------------------------------------------------------------------------

    #[test]
    fn test_export_md_structure() {
        let questions = vec![Question {
            id: "q1".to_string(),
            text: "What is 1+1?".to_string(),
            explanation: None,
            distractors: None,
            subject: "Math".to_string(),
            topics: vec![],
            answers: vec![
                Answer {
                    text: "2".to_string(),
                    is_correct: true,
                    explanation: None,
                },
                Answer {
                    text: "3".to_string(),
                    is_correct: false,
                    explanation: None,
                },
            ],
        }];
        let result = export_md("Algebra", &questions).unwrap();
        assert!(result.starts_with("# Algebra\n"));
        assert!(result.contains("**Question 1.**"));
        assert!(result.contains("What is 1+1?"));
        // Both answer choices must appear in the shuffled output
        assert!(result.contains(") 2"));
        assert!(result.contains(") 3"));
        assert!(result.contains("## Answers"));
        // Answer key must contain the entry for question 1 (format: "1. <letter>")
        assert!(result.contains("1. "));
    }

    #[test]
    fn test_export_md_answer_key_present() {
        let questions = vec![Question {
            id: "q1".to_string(),
            text: "Pick the right one".to_string(),
            explanation: None,
            distractors: None,
            subject: "Test".to_string(),
            topics: vec![],
            answers: vec![
                Answer {
                    text: "Wrong".to_string(),
                    is_correct: false,
                    explanation: None,
                },
                Answer {
                    text: "Correct".to_string(),
                    is_correct: true,
                    explanation: None,
                },
            ],
        }];
        let result = export_md("Quiz", &questions).unwrap();
        // Answer key section must appear and contain question 1
        assert!(result.contains("## Answers"));
        assert!(result.contains("1."));
    }

    #[test]
    fn test_export_md_empty_questions() {
        let result = export_md("Empty", &[]).unwrap();
        assert!(result.starts_with("# Empty\n"));
        // No answer key section for an empty question list
        assert!(!result.contains("## Answers"));
    }

    // -------------------------------------------------------------------------
    // export_qti_zip
    // -------------------------------------------------------------------------

    #[test]
    fn test_export_qti_zip_returns_bytes() {
        let questions = vec![Question {
            id: "q1".to_string(),
            text: "Question text".to_string(),
            explanation: None,
            distractors: None,
            subject: "Test".to_string(),
            topics: vec![],
            answers: vec![
                Answer {
                    text: "A".to_string(),
                    is_correct: true,
                    explanation: None,
                },
                Answer {
                    text: "B".to_string(),
                    is_correct: false,
                    explanation: None,
                },
            ],
        }];
        let result = export_qti_zip("My Quiz", &questions);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        // ZIP magic bytes: PK\x03\x04
        assert!(bytes.starts_with(b"PK\x03\x04"));
    }
}
