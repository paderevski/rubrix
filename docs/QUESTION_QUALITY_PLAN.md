# Question quality plan (AP CS A generation)

Status: planned, nothing in the app changed yet. Evidence and tools: `tools/verify_java/README.md`, `tools/qgen/`.

## Decision
Generate with Claude Haiku 5.5 using the existing prompt. Check each question with free code, regenerate on a flag,
and keep LLM judges out of the per-question path (use them as a ~10% developer-side audit).

Why: 120 Haiku questions with the unchanged prompt had 0 wrong keys and 0 second-correct answers (this bounds the
true rate at roughly 5% or less, it does not prove 0.1%). gpt-oss logs had ~20% "second answer also correct" and
~38% leakage. Free checks plus retry cleared all first-attempt flags (6/60 per batch, about 1.1 attempts/question).

## Life of one question (count=1)
1. Build prompt (`prompts.rs`): prompt.txt, bank examples, topic labels, `{count}`=1.
2. Gateway call to Haiku 5.5. Reasoning chunks tagged `block_type: "reasoning"` and kept out of the parser input.
3. Parse (`parse_llm_response`) answer text only. Unparseable counts as a failed attempt.
4. Free checks (new `quality.rs`), multiple-choice only: 4-5 choices, exactly one correct, required fields,
   no duplicate choices; comments check (Computer Science only) for explanatory comments in stem code.
5. Flagged and attempts < 1-2: rebuild the prompt with `count=1` and call again, with no UI stream events.
6. Deliver. If flags remain, attach them to the question (`flags` field) and show a "needs review" marker.
No second LLM call in the request path. Wrong keys on well-formed questions, logically identical choices and
stem-prose leakage are not caught here; the teacher's review is the backstop.

## Work order
1. Parse hardening and `quality.rs` (offline, free, testable against the llm_log files). Keep parsing and
   validation separate; FRQ (alpha, may break) has empty `answers`. Port `layer_lint` and `layer_comments` from
   `tools/verify_java/review.py`.
2. Model switch. New `lambda_functions/anthropic_gateway/` handler (leave the Bedrock handler untouched). If
   Haiku 5.5 ever appears on Bedrock, `BEDROCK_MODEL_ID` alone may be enough.
   - Anthropic stream has no `totalTokens`: sum input tokens (message_start) and output tokens (final message_delta,
     includes thinking) or balance deduction silently stops.
   - Map 429/529/overloaded to the existing SSE error event.
   - Drop temperature/top_p (Haiku 5.5 rejects non-default).
   - Haiku 5.5's tokenizer counts ~30% more tokens for the same text; budgets are raw token counts (fine for now).
   - Raise the 90 s reqwest timeout (covers the whole streamed body) to ~180 s; short-circuit the `auth_check`
     login prompt so it does not run a full model call.
3. Retries (`generate_questions` makes one call for `count`; retry via `build_generation_prompt` with count=1, not
   `regenerate_question`). Add optional `flags` to `Question` (Rust + types.ts) and show it in `QuestionCard`.
4. Optional prompt change: distractor rules (`tools/qgen/prompts/distractor_notes.txt`) into prompt.txt. Fewer
   repeated/throwaway distractors (24 -> 7 of 60 questions repeat an error type) but near-miss traps for strong
   students did not change.

## Reasoning stream (UI)
Haiku 5.5 omits thinking text by default; request `thinking: {type: "adaptive", display: "summarized"}` to get
summaries. Empty `thinking_delta` chunks still arrive and can drive a "thinking..." indicator. Verify with
`tools/qgen/test_thinking_stream.py` (needs ANTHROPIC_API_KEY in your shell; ~$0.003 per variant). Summaries are not
raw reasoning, so logs are less raw than gpt-oss ones. Thinking is ~70% of output tokens; `{type: "disabled"}` is
accepted at effort high or below.

## Prompt caching: not now
Opt-in via `cache_control`; the prompt's variable parts (topics, difficulty, count, random examples) sit before the
~8K characters of static rules, so only ~1.6K characters are a stable prefix. Reordering prompt.txt would make the
rules cacheable but changes the prompt and needs re-measuring; savings are about 10-15% per question.

## Later ideas
- Teacher "Review and fix" button: free checks, then one LLM call (adjudicator-style checklist) that returns findings
  and a repaired question as a diff to accept; re-check the repair. Validate first on the Nov 8 gpt-oss batch.
- `exec` (run the Java) as an opt-in/developer check (needs a JDK; covers ~25-35% of questions).
- Auto-strip explanatory comments; missing-quote JSON repair only if Haiku output needs it.
- gpt-oss vs Haiku head-to-head on identical specs (needs gateway credentials supplied by the user via env vars).
