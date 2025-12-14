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
        // Question number and content
        output.push_str(&format!("{}. {}\n\n", i + 1, q.content));
        
        // Answers - correct one first
        let correct_first: Vec<_> = q.answers.iter()
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
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        
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
fn generate_qti_xml(title: &str, questions: &[Question]) -> String {
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
            
            let answer_html = convert_inline_code(&htmlescape::encode_minimal(&answer.text));
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
    // Convert markdown-style code blocks to HTML
    let code_re = Regex::new(r"(?s)```(\w+)?\n(.*?)```").unwrap();
    let mut html = q.content.clone();
    
    // Replace code blocks with <pre> tags
    html = code_re.replace_all(&html, |caps: &regex::Captures| {
        let code = caps.get(2).map_or("", |m| m.as_str());
        format!("<pre>{}</pre>", htmlescape::encode_minimal(code.trim()))
    }).to_string();
    
    // Escape remaining HTML and convert inline code
    let parts: Vec<&str> = html.split("<pre>").collect();
    let mut result = String::new();
    
    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            // First part - escape and convert inline code
            let escaped = htmlescape::encode_minimal(part);
            result.push_str(&convert_inline_code(&escaped));
        } else {
            // Part after <pre> - find the </pre> and handle accordingly
            if let Some(pre_end) = part.find("</pre>") {
                // Code block content (already escaped above)
                result.push_str("<pre>");
                result.push_str(&part[..pre_end]);
                result.push_str("</pre>");
                // Text after code block
                let after = &part[pre_end + 6..];
                let escaped = htmlescape::encode_minimal(after);
                result.push_str(&convert_inline_code(&escaped));
            } else {
                result.push_str("<pre>");
                result.push_str(part);
            }
        }
    }
    
    // Wrap in paragraph if not already structured
    if !result.contains("<pre>") && !result.contains("<table>") {
        result = format!("<p>{}</p>", result);
    }
    
    result
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
        let questions = vec![
            Question {
                id: "1".to_string(),
                content: "What is 2+2?".to_string(),
                answers: vec![
                    Answer { text: "4".to_string(), is_correct: true },
                    Answer { text: "3".to_string(), is_correct: false },
                ],
            },
        ];
        
        let result = export_txt("Test", &questions).unwrap();
        assert!(result.contains("Title: Test"));
        assert!(result.contains("1. What is 2+2?"));
        assert!(result.contains("a. 4"));
    }
}
