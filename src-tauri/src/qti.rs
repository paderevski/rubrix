//! QTI export functionality - generates IMS QTI XML for LMS import

use crate::Question;
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
// Export Functions
// ============================================================================

/// Export questions to our intermediate .txt format
pub fn export_txt(title: &str, questions: &[Question]) -> Result<String, String> {
    let mut output = format!("Title: {}\n\n", title);

    for (i, q) in questions.iter().enumerate() {
        // Question number and text (markdown format)
        output.push_str(&format!("{}. {}\n\n", i + 1, q.text));

        // Answers - correct one first
        let correct_first: Vec<_> = q
            .answers
            .iter()
            .filter(|a| a.is_correct)
            .chain(q.answers.iter().filter(|a| !a.is_correct))
            .collect();

        for answer in correct_first {
            output.push_str(&format!("a. {}\n", answer.text));
        }

        output.push_str("\n\n");
    }

    Ok(output)
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

            // Convert LaTeX first, then inline code for answer text
            let processed = convert_latex(&answer.text);
            let answer_html = convert_inline_code(&processed);
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
    let mut html = String::new();

    // Parse markdown: extract code blocks and convert rest
    let code_block_re = Regex::new(r"```(?:java)?\n([^`]+)\n```").unwrap();

    // Split text into parts with and without code blocks
    let mut last_end = 0;
    let parts: Vec<(&str, Option<&str>)> = {
        let mut result = Vec::new();
        for cap in code_block_re.captures_iter(&q.text) {
            let match_start = cap.get(0).unwrap().start();
            let match_end = cap.get(0).unwrap().end();

            // Add text before code block
            if last_end < match_start {
                result.push((&q.text[last_end..match_start], None));
            }

            // Add code block
            result.push(("", Some(cap.get(1).unwrap().as_str())));
            last_end = match_end;
        }

        // Add remaining text after last code block
        if last_end < q.text.len() {
            result.push((&q.text[last_end..], None));
        }

        result
    };

    // Convert each part
    for (text_part, code_part) in parts {
        if let Some(code) = code_part {
            // Code block
            html.push_str(&format!(
                "<pre>{}</pre>",
                htmlescape::encode_minimal(code.trim())
            ));
        } else if !text_part.is_empty() {
            // Regular text with LaTeX and inline code
            let processed = convert_latex(text_part);
            let processed = convert_inline_code(&processed);
            html.push_str(&processed);
        }
    }

    html
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
        assert!(result.contains("Title: Test"));
        assert!(result.contains("1. What is 2+2?"));
        assert!(result.contains("a. 4"));
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
}
