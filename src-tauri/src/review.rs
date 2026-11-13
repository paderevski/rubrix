//! On-demand review of a single multiple-choice question.
//!
//! The model independently solves the question, checks it against a subject checklist, and may
//! propose a minimally revised version. Everything here is pure (prompt building, parsing,
//! checking) so it can be unit-tested; the Tauri command in `main.rs` only orchestrates.
//!
//! Subject-specific rules live in `<subject>/review-prompt.txt`; subjects without one use
//! `DEFAULT_REVIEW_TEMPLATE`.

use crate::prompts;
use crate::Question;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ReviewFinding {
    /// "error" (the question is wrong or unfair) or "warning" (worth fixing, not wrong).
    pub severity: String,
    /// Parts the finding concerns, e.g. "text", "choice C", "explanation".
    pub fields: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewResult {
    /// "ok" or "issues"
    pub verdict: String,
    /// The reviewer's own answer, worked out before reading the key.
    pub solved_answer: Option<String>,
    pub findings: Vec<ReviewFinding>,
    /// Proposed replacement; `None` when nothing needs changing or no usable fix was produced.
    pub revised: Option<Question>,
    /// Set when the model proposed a revision that failed the structural checks.
    pub revision_problem: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawFinding {
    #[serde(default)]
    severity: String,
    #[serde(default)]
    fields: Vec<String>,
    #[serde(default)]
    summary: String,
}

#[derive(Debug, Deserialize)]
struct RawReview {
    #[serde(default)]
    verdict: String,
    #[serde(default)]
    solved_answer: Option<String>,
    #[serde(default)]
    findings: Vec<RawFinding>,
}

// ---- Prompt ------------------------------------------------------------------

/// Subset of a question the reviewer sees (no ids or bookkeeping).
fn question_for_prompt(q: &Question) -> String {
    let value = serde_json::json!({
        "text": q.text,
        "answers": q.answers.iter().map(|a| serde_json::json!({
            "text": a.text,
            "is_correct": a.is_correct,
            "explanation": a.explanation.clone().unwrap_or_default(),
        })).collect::<Vec<_>>(),
        "explanation": q.explanation.clone().unwrap_or_default(),
    });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".to_string())
}

fn same_content(a: &Question, b: &Question) -> bool {
    question_for_prompt(a) == question_for_prompt(b)
}

pub fn build_review_prompt(
    template: Option<&str>,
    current: &Question,
    previous: Option<&Question>,
    user_notes: Option<&str>,
    topics_label: &str,
) -> String {
    let previous_block = match previous {
        Some(prev) if !same_content(prev, current) => format!(
            "## Earlier Version (before the user's edits)\n\n\
             The user edited this question by hand. Below is the version before their edits. \
             Compare the two and check especially whether the edits introduced an error: a key \
             that no longer matches, code that no longer compiles or produces a different result, \
             an explanation that no longer agrees, or choices that now overlap. Judge the CURRENT \
             version. Treat the user's edits as intentional: keep them, and fix the rest of the \
             question (key text, other choices, explanations) so it agrees with them. Revert an \
             edit only if the edit itself is invalid, such as code that no longer compiles, and \
             say so in a finding.\n\n```json\n{}\n```\n",
            question_for_prompt(prev)
        ),
        _ => String::new(),
    };

    let notes_block = match user_notes.map(str::trim) {
        Some(notes) if !notes.is_empty() => format!(
            "## Notes from the User (check these first)\n\n{}\n",
            notes
        ),
        _ => String::new(),
    };

    let subject = if current.subject.trim().is_empty() {
        "(not specified)"
    } else {
        current.subject.trim()
    };
    let difficulty = if current.difficulty.trim().is_empty() {
        "(not specified)"
    } else {
        current.difficulty.trim()
    };

    template
        .unwrap_or(DEFAULT_REVIEW_TEMPLATE)
        .replace("{subject}", subject)
        .replace("{topics}", topics_label)
        .replace("{difficulty}", difficulty)
        .replace("{question_json}", &question_for_prompt(current))
        .replace("{previous_version}", &previous_block)
        .replace("{user_notes}", &notes_block)
}

// ---- Parsing -------------------------------------------------------------------

/// Return the contents of the last complete `<review>...</review>` block.
fn extract_review_block(response: &str) -> Option<(&str, usize)> {
    let end = response.rfind("</review>")?;
    let start = response[..end].rfind("<review>")? + "<review>".len();
    Some((&response[start..end], end + "</review>".len()))
}

fn strip_json_fence(s: &str) -> &str {
    let t = s.trim();
    let t = t
        .strip_prefix("```json")
        .or_else(|| t.strip_prefix("```"))
        .unwrap_or(t);
    t.strip_suffix("```").unwrap_or(t).trim()
}

fn normalize_severity(s: &str) -> String {
    match s.trim().to_ascii_lowercase().as_str() {
        "error" | "critical" | "major" | "high" => "error".to_string(),
        _ => "warning".to_string(),
    }
}

/// Normalize choice text for exact-duplicate detection: ignore whitespace, backticks, and
/// code-fence lines (including the language tag). Case is kept: in code it matters.
pub fn normalize_choice(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim_start().starts_with("```"))
        .collect::<Vec<_>>()
        .join(" ")
        .replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Structural checks a proposed revision must pass.
pub fn check_revision(original: &Question, revised: &Question) -> Result<(), String> {
    if revised.answers.len() != original.answers.len() {
        return Err(format!(
            "changed the number of choices from {} to {}",
            original.answers.len(),
            revised.answers.len()
        ));
    }
    prompts::validate_answer_keys(std::slice::from_ref(revised))
        .map_err(|_| "does not have exactly one correct answer".to_string())?;
    let original_key = original.answers.iter().position(|a| a.is_correct);
    let revised_key = revised.answers.iter().position(|a| a.is_correct);
    if original_key != revised_key {
        return Err("moved the correct answer to a different choice".to_string());
    }
    let normalized: Vec<String> = revised.answers.iter().map(|a| normalize_choice(&a.text)).collect();
    for i in 0..normalized.len() {
        for j in (i + 1)..normalized.len() {
            if normalized[i] == normalized[j] {
                return Err(format!(
                    "has identical choices {} and {}",
                    (b'A' + i as u8) as char,
                    (b'A' + j as u8) as char
                ));
            }
        }
    }
    Ok(())
}

/// Keep the original's identity and metadata; take only content from the revision.
fn adopt_metadata(original: &Question, mut revised: Question) -> Question {
    revised.id = original.id.clone();
    revised.subject = original.subject.clone();
    revised.topics = original.topics.clone();
    revised.subtopics = original.subtopics.clone();
    revised.difficulty = original.difficulty.clone();
    revised.cognitive_level = original.cognitive_level.clone();
    revised.rubric = original.rubric.clone();
    revised
}

/// Turn a raw model response into a `ReviewResult`. Errors only when the `<review>` block is
/// missing or unreadable; a bad revision is reported through `revision_problem` instead.
pub fn interpret_review_response(original: &Question, response: &str) -> Result<ReviewResult, String> {
    let (block, block_end) =
        extract_review_block(response).ok_or_else(|| "No <review> block in model output".to_string())?;
    let raw: RawReview = serde_json::from_str(strip_json_fence(block))
        .map_err(|e| format!("Could not read <review> block: {}", e))?;

    let findings: Vec<ReviewFinding> = raw
        .findings
        .into_iter()
        .filter(|f| !f.summary.trim().is_empty())
        .map(|f| ReviewFinding {
            severity: normalize_severity(&f.severity),
            fields: f.fields,
            summary: f.summary.trim().to_string(),
        })
        .collect();

    let verdict = if raw.verdict.trim().eq_ignore_ascii_case("ok") && findings.is_empty() {
        "ok"
    } else {
        "issues"
    }
    .to_string();

    let mut revised = None;
    let mut revision_problem = None;
    let after = &response[block_end..];
    if verdict == "issues" && after.contains("<question>") {
        match prompts::parse_llm_response(after) {
            Ok(mut questions) if !questions.is_empty() => {
                let candidate = adopt_metadata(original, questions.remove(0));
                if same_content(&candidate, original) {
                    // Model echoed the question unchanged: nothing to apply.
                } else {
                    match check_revision(original, &candidate) {
                        Ok(()) => revised = Some(candidate),
                        Err(problem) => revision_problem = Some(format!("The proposed fix {}", problem)),
                    }
                }
            }
            Ok(_) => revision_problem = Some("The proposed fix was empty".to_string()),
            Err(err) => revision_problem = Some(format!("The proposed fix could not be read: {}", err)),
        }
    }

    Ok(ReviewResult {
        verdict,
        solved_answer: raw.solved_answer.filter(|s| !s.trim().is_empty()),
        findings,
        revised,
        revision_problem,
    })
}

// ---- Built-in fallback template ------------------------------------------------

/// Used for subjects without a `review-prompt.txt`. Subject-neutral on purpose.
pub const DEFAULT_REVIEW_TEMPLATE: &str = r#"You are reviewing ONE multiple-choice question for a teacher. Your job is to find real defects and propose the smallest fix, not to rewrite the question.

**Subject:** {subject}
**Topic(s):** {topics}
**Difficulty:** {difficulty}

## Question Under Review

```json
{question_json}
```

{previous_version}
{user_notes}
## Procedure

1. Solve the question yourself from the stem alone, before looking at which answer is marked correct. Record your answer.
2. Compare your answer with the marked correct answer. Then write one line per choice stating exactly what it evaluates to or claims under the stem's conditions, and compare every pair of lines, including pairs of wrong choices. If two lines say the same thing, those choices are equivalent.
3. Check every item below. Report only concrete defects; do not report style preferences.
   - The marked answer is correct and exactly one choice is correct.
   - Every other choice is actually incorrect for the stem as written.
   - No two choices are the same or equivalent.
   - Nothing gives the answer away (names, labels, comments, wording, or one choice standing out).
   - The stem is complete and unambiguous.
   - The explanation agrees with the marked answer and is accurate.
   - Each choice's explanation matches that choice.
   - No explanation refers to choices by letter or position.

## Fix Rules

- Change as little as possible. Do not restyle or rewrite parts that are fine.
- Never move the correct answer to a different choice: copy every `is_correct` value exactly as it is. If the key is wrong, rewrite the TEXT of the choice marked correct so it becomes correct, even when the right answer already appears as another choice.
- If that makes it the same as another choice, replace that other choice with a new plausible wrong answer and update its explanation.
- Keep the same number of choices. Update explanations to match any change.

## Output Format

Write your reasoning first. Then output a `<review>` block with JSON:

<review>
{"verdict": "ok" | "issues", "solved_answer": "your own answer", "findings": [{"severity": "error" | "warning", "fields": ["text" | "choice A".."choice E" | "explanation"], "summary": "one sentence"}]}
</review>

If the verdict is "issues" and a fix is needed, follow it with the complete revised question as a JSON array with one object (fields: text, answers [text, is_correct, explanation], explanation):

<question>
[ { ... } ]
</question>

If the verdict is "ok", output no `<question>` block.
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Answer;

    fn mcq(choices: &[(&str, bool)]) -> Question {
        Question {
            id: "q7".into(),
            text: "What is printed?".into(),
            answers: choices
                .iter()
                .map(|(t, c)| Answer { text: t.to_string(), is_correct: *c, explanation: Some("e".into()) })
                .collect(),
            explanation: Some("Because.".into()),
            rubric: None,
            subject: "Computer Science".into(),
            topics: vec!["U1".into()],
            difficulty: "D2".into(),
            cognitive_level: "B3".into(),
            subtopics: vec!["ST003".into()],
        }
    }

    fn five() -> Question {
        mcq(&[("`1`", false), ("`2`", true), ("`3`", false), ("`4`", false), ("`5`", false)])
    }

    fn revision_json(choices: &[(&str, bool)]) -> String {
        let answers: Vec<String> = choices
            .iter()
            .map(|(t, c)| format!(r#"{{"text": "{}", "is_correct": {}, "explanation": "e"}}"#, t, c))
            .collect();
        format!(
            r#"<question>[{{"text": "What is printed?", "answers": [{}], "explanation": "Fixed."}}]</question>"#,
            answers.join(",")
        )
    }

    #[test]
    fn ok_verdict_has_no_revision() {
        let r = interpret_review_response(
            &five(),
            r#"thinking... <review>{"verdict": "ok", "solved_answer": "2", "findings": []}</review>"#,
        )
        .unwrap();
        assert_eq!(r.verdict, "ok");
        assert_eq!(r.solved_answer.as_deref(), Some("2"));
        assert!(r.revised.is_none() && r.revision_problem.is_none());
    }

    #[test]
    fn uses_last_review_block_and_accepts_fences() {
        let resp = "I will write <review> later.\n<review>\n```json\n{\"verdict\":\"issues\",\"findings\":[{\"severity\":\"Critical\",\"fields\":[\"choice B\"],\"summary\":\"Key is wrong\"}]}\n```\n</review>";
        let r = interpret_review_response(&five(), resp).unwrap();
        assert_eq!(r.verdict, "issues");
        assert_eq!(r.findings[0].severity, "error");
        assert!(r.revised.is_none());
    }

    #[test]
    fn missing_review_block_is_an_error() {
        assert!(interpret_review_response(&five(), "no block here").is_err());
    }

    #[test]
    fn accepts_valid_revision_and_keeps_metadata() {
        let resp = format!(
            r#"<review>{{"verdict":"issues","findings":[{{"severity":"error","fields":["choice B"],"summary":"Key wrong"}}]}}</review>{}"#,
            revision_json(&[("`1`", false), ("`6`", true), ("`3`", false), ("`4`", false), ("`5`", false)])
        );
        let r = interpret_review_response(&five(), &resp).unwrap();
        let q = r.revised.expect("revision");
        assert_eq!(q.id, "q7");
        assert_eq!(q.subtopics, vec!["ST003".to_string()]);
        assert_eq!(q.answers[1].text, "`6`");
        assert!(r.revision_problem.is_none());
    }

    #[test]
    fn rejects_moved_key() {
        let resp = format!(
            r#"<review>{{"verdict":"issues","findings":[{{"severity":"error","fields":[],"summary":"x"}}]}}</review>{}"#,
            revision_json(&[("`1`", true), ("`2`", false), ("`3`", false), ("`4`", false), ("`5`", false)])
        );
        let r = interpret_review_response(&five(), &resp).unwrap();
        assert!(r.revised.is_none());
        assert!(r.revision_problem.unwrap().contains("moved the correct answer"));
    }

    #[test]
    fn rejects_duplicate_choices_ignoring_backticks_and_fences() {
        let resp = format!(
            r#"<review>{{"verdict":"issues","findings":[{{"severity":"error","fields":[],"summary":"x"}}]}}</review>{}"#,
            revision_json(&[("`1`", false), ("`6`", true), ("6", false), ("`4`", false), ("`5`", false)])
        );
        let r = interpret_review_response(&five(), &resp).unwrap();
        assert!(r.revision_problem.unwrap().contains("identical choices B and C"));
        assert_eq!(
            normalize_choice("```java\nint x = 1;\n```"),
            normalize_choice("`int x = 1;`")
        );
    }

    #[test]
    fn rejects_changed_choice_count() {
        let resp = format!(
            r#"<review>{{"verdict":"issues","findings":[{{"severity":"warning","fields":[],"summary":"x"}}]}}</review>{}"#,
            revision_json(&[("`1`", false), ("`2`", true), ("`3`", false), ("`4`", false)])
        );
        let r = interpret_review_response(&five(), &resp).unwrap();
        assert!(r.revision_problem.unwrap().contains("number of choices"));
    }

    #[test]
    fn unchanged_echo_is_not_a_revision() {
        let resp = format!(
            r#"<review>{{"verdict":"issues","findings":[{{"severity":"warning","fields":[],"summary":"x"}}]}}</review>{}"#,
            revision_json(&[("`1`", false), ("`2`", true), ("`3`", false), ("`4`", false), ("`5`", false)])
        )
        .replace("Fixed.", "Because.");
        let r = interpret_review_response(&five(), &resp).unwrap();
        assert!(r.revised.is_none() && r.revision_problem.is_none());
    }

    #[test]
    fn ok_verdict_with_findings_counts_as_issues() {
        let r = interpret_review_response(
            &five(),
            r#"<review>{"verdict":"ok","findings":[{"severity":"warning","fields":[],"summary":"Minor"}]}</review>"#,
        )
        .unwrap();
        assert_eq!(r.verdict, "issues");
    }

    #[test]
    fn prompt_includes_previous_version_only_when_different() {
        let current = five();
        let mut previous = five();
        let same = build_review_prompt(None, &current, Some(&previous), None, "Loops");
        assert!(!same.contains("Earlier Version"));
        previous.text = "Old stem".into();
        let diff = build_review_prompt(None, &current, Some(&previous), Some("B and D look alike"), "Loops");
        assert!(diff.contains("Earlier Version") && diff.contains("Old stem"));
        assert!(diff.contains("B and D look alike"));
        assert!(!diff.contains("{question_json}") && !diff.contains("{user_notes}"));
    }
}

/// Live evaluation against the gateway, using the sample questions in
/// `tools/review-eval/<subject>.json`. Ignored by default because it spends model budget:
///
///   cargo test --manifest-path src-tauri/Cargo.toml review_live_eval -- --ignored --nocapture
///
/// Env: REVIEW_EVAL_SUBJECT (default "Computer Science"), REVIEW_EVAL_ONLY (comma-separated names),
/// REVIEW_EVAL_OUT (path for full JSON results). Credentials come from the app's saved login and
/// are never printed.
#[cfg(test)]
mod live_eval {
    use super::*;
    use crate::llm;
    use std::path::PathBuf;

    #[derive(Deserialize)]
    struct Fixture {
        name: String,
        expect: String,
        question: Question,
        #[serde(default)]
        previous: Option<Question>,
    }

    #[derive(Deserialize)]
    struct SavedLogin {
        username: String,
        password: String,
    }

    fn changed_parts(a: &Question, b: &Question) -> Vec<String> {
        let mut parts = Vec::new();
        if a.text != b.text {
            parts.push("text".to_string());
        }
        for (i, (x, y)) in a.answers.iter().zip(&b.answers).enumerate() {
            let letter = (b'A' + i as u8) as char;
            if x.text != y.text {
                parts.push(format!("choice {} `{}` -> `{}`", letter, normalize_choice(&x.text), normalize_choice(&y.text)));
            } else if x.explanation != y.explanation {
                parts.push(format!("choice {} explanation", letter));
            }
        }
        if a.explanation != b.explanation {
            parts.push("explanation".to_string());
        }
        parts
    }

    #[tokio::test]
    #[ignore]
    async fn review_live_eval() {
        let subject = std::env::var("REVIEW_EVAL_SUBJECT").unwrap_or_else(|_| "Computer Science".into());
        let only = std::env::var("REVIEW_EVAL_ONLY").ok();
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let slug = subject.to_lowercase().replace(' ', "-");
        let fixtures: Vec<Fixture> = serde_json::from_str(
            &std::fs::read_to_string(root.join(format!("tools/review-eval/{}.json", slug))).expect("fixtures"),
        )
        .expect("fixture JSON");
        let template =
            std::fs::read_to_string(root.join(format!("imports/knowledge/{}/review-prompt.txt", subject))).ok();
        println!("template: {}", if template.is_some() { "subject file" } else { "built-in default" });

        let home = std::env::var("HOME").expect("HOME");
        let login: SavedLogin = serde_json::from_str(
            &std::fs::read_to_string(format!(
                "{}/Library/Application Support/com.rubrix.app/auth/credentials.json",
                home
            ))
            .expect("saved login (log in once in the app)"),
        )
        .expect("login JSON");
        let auth = llm::GatewayAuth { user: login.username, password: login.password };

        let mut results = Vec::new();
        for f in fixtures
            .iter()
            .filter(|f| only.as_deref().map_or(true, |list| list.split(',').any(|n| n.trim() == f.name)))
        {
            let prompt = build_review_prompt(template.as_deref(), &f.question, f.previous.as_ref(), None, "(eval)");
            println!("\n=== {} ===\nexpect: {}", f.name, f.expect);
            let response = match llm::generate(&prompt, None, Some(auth.clone()), Some("review")).await {
                Ok(r) => r,
                Err(e) => {
                    println!("GATEWAY ERROR: {}", e);
                    continue;
                }
            };
            match interpret_review_response(&f.question, &response) {
                Ok(r) => {
                    println!("verdict: {}   solved: {:?}", r.verdict, r.solved_answer);
                    for finding in &r.findings {
                        println!("  [{}] {:?} {}", finding.severity, finding.fields, finding.summary);
                    }
                    if let Some(rev) = &r.revised {
                        println!("revision changes: {:?}", changed_parts(&f.question, rev));
                    }
                    if let Some(p) = &r.revision_problem {
                        println!("REVISION PROBLEM: {}", p);
                    }
                    results.push(serde_json::json!({"name": f.name, "result": r, "raw": response}));
                }
                Err(e) => {
                    println!("PARSE ERROR: {}", e);
                    results.push(serde_json::json!({"name": f.name, "error": e, "raw": response}));
                }
            }
        }
        if let Ok(out) = std::env::var("REVIEW_EVAL_OUT") {
            std::fs::write(&out, serde_json::to_string_pretty(&results).unwrap()).expect("write results");
            println!("\nfull results: {}", out);
        }
    }
}
