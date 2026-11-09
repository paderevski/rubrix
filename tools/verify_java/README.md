# Question quality layers (AP CS A)

Standalone tools for checking AI-generated multiple-choice questions. Nothing here is wired into the app,
the prompts or the Lambda; every layer is opt-in. Needs `javac`/`java` on PATH and, for LLM layers, the
`claude` CLI (cost is reported per call).

## Recommendation (as of 2026-11-08)

Generate with Haiku 5.5 using the app's unchanged prompt. Run only the free layers on each batch
(`lint,comments,exec`). Use the LLM layers as an occasional sample audit (about 20 questions after a model or
prompt change, around $0.70), not per question.

Evidence: 60 questions, 15 topics, Haiku 5.5, 0 key-level defects (adjudicator + Opus blind solve),
1 arguable prose leak, 16 execution-verified with 0 failures. That bounds the true defect rate at roughly 5%
or less; it does not prove 0.1%. Auditors were Claude models, so only `exec` is independent. gpt-oss logs
(Nov 8 batch) showed about 20% "second answer also correct" flags and 38% leakage.

## Layers (`review.py --layers a,b,c`)

| Layer | Cost | What it does |
|---|---|---|
| `lint` | free | Structure: exactly one correct answer, duplicate or equivalent choice text (line breaks preserved), missing fields |
| `comments` | free | Heuristic for explanatory comments in stem code and choices; ignores `//` in string literals and placeholders like `/* missing for statement */` |
| `exec` | free | Compiles and runs the Java (javac + JDI line tracer). Checks the key against real output, observed variable, line counts, compile errors, patches, etc. Status: pass, FAIL, unmatched, skip, ran, nondeterministic, unverifiable. Covers about 16 of 60 generated questions automatically, 60 of 87 bank questions with the drafted sidecar |
| `leak` | LLM | Judge for stems or choices that reveal what the code does; background comments such as `// ASCII value 65` are exempt |
| `solve` | LLM | Blind solve (no key shown), N times (`--solve-n`). Catches wrong keys; cannot see two-correct choices |
| `adjudicate` | LLM | Sees the key; flags "another choice also defensible" and key problems. Explanation nitpicks are off unless `--adj-explanations` (noisy, about 54% flagged) |
| `draft` | LLM | LLM writes a verification block blind, then `exec` runs it. Proved 1 of 11 in testing, mostly harness failures; not worth it yet |

## Running

```
# free layers over a batch of questions (JSON with a "questions" list)
python3 tools/verify_java/review.py INPUT.json --layers lint,comments,exec -j 8

# add a sample LLM audit
python3 tools/verify_java/review.py INPUT.json --layers adjudicate,solve --solve-n 1 --model sonnet --limit 20

# mine questions out of an app log (repairs the missing-opening-quote JSON slip)
python3 tools/verify_java/mine_logs.py llm_log.txt --since 2026-11-08 --out mined.json

# run the Java for one question and show the trace
python3 tools/verify_java/verify.py INPUT.json --trace ID
```

`verify.py` takes an optional `--blocks blocks/cs-draft.json`: a sidecar of 44 drafted `verification` blocks
(oracle, patch, substitute, compile, run, witness, equivalent, claims, segments, none). The banks themselves
were not modified.

## Distractor quality (`distractors.py`)

Not part of `review.py`. Per distractor, a Sonnet judge rates whether the stated misconception is concrete and yields
exactly that choice (`grounded`), whether a partly prepared student would pick it (`plausible`), and whether the
choice text narrates its own flaw. Separately, for runnable "what is printed" stems, an LLM proposes code edits that
implement each misconception, the edited Java is run, and the output is compared with the distractor (independent of
any judge's opinion). About $0.02 per question.

```
python3 tools/verify_java/distractors.py haiku=GEN.json gptoss=LOG_MINED.json --n 25 --out res.json
```

First read (2026-11-08, 25 questions per set, 100 distractors each, not topic-matched):

| | Haiku 5.5 (app prompt) | gpt-oss (Nov 8 log) |
|---|---|---|
| concrete error that yields the choice | 78% | 62% |
| throwaway (no identifiable error) | 4% | 16% |
| partly prepared student would pick it | 49% (rest: only weak students) | 49% |
| choice text narrates its flaw | 0% | 5% |
| questions repeating an error type | 8/25 | 18/25 |
| mutate-and-run reproduced | 22/24 (6 questions) | 7/12 (3 questions) |

Weak spots on both: many distractors rated plausible only for weak students (rare "2" for strong-student traps);
Haiku's repeats are mostly several distractors sharing one error (e.g. skip-after-remove); gpt-oss has bare
default-value choices (`0`, `Data E`) and comments inside choices that state the bug.

### Distractor prompt variant (2026-11-08)

`../qgen/prompts/distractor_notes.txt` is pasted into the app's own "Additional Instructions" slot (no repo prompt
change; generate with `tools/qgen/gen_variant.py NOTES OUT.json`). Same 60 coverage specs, Haiku 5.5:

| | app prompt | + distractor notes |
|---|---|---|
| concrete error that yields the choice | 68% | 79% |
| questions repeating an error type | 19/60 | 7/58 |
| mutate-and-run reproduced | 48/56 | 58/60 |
| choice text narrates flaw | 5 | 1 |
| near-miss traps for strong students | 39% | 38% (unchanged; judge rates 59/60 questions as having one already) |
| key-level or structure defects | 0/60 | 2/60 (one wrong key on a 2-choice question, one duplicate choice) |

Too few questions to tell whether the 2 defects are noise (p about 0.25); rerun on a fresh batch before adopting.

### Regenerate-on-flag (`../qgen/gen_checked.py`)

Generates with the app's prompt (optionally plus `--notes`), runs `lint,comments,exec` on each question and regenerates
flagged ones (default 2 retries). 2026-11-09, Haiku, 60 fresh specs: with the distractor notes 6/60 flagged on first
attempt, plain prompt 6/60; all cleared by retry (avg 1.1 attempts, about $0.004 per question plus retries). Both arms
produced one malformed 1-choice / no-key question, so that failure is not caused by the notes. Held-out audit of the
notes arm after retries (adjudicate + blind solve + lint/comments/exec): 0/60 flagged. Plain arm audit: 0/60 flagged.
Distractors on this fresh batch, plain vs notes: concrete-error 77% vs 81%, throwaway 10 vs 1, questions repeating an
error type 24/60 vs 7/60, mutate-and-run reproduced 20/24 vs 60/64, near-miss 41% vs 44%, plausible-2 51% vs 50%.

## Generation and comparison harness (`tools/qgen`)

Replicates the app's prompt path read-only (`bank.py`: topic labels, 3-tier example selection, prompt
rendering) so arms can be compared on identical specs.

```
# the app's prompt, unchanged (baseline arm), 60-question coverage run, then audit
python3 tools/qgen/compare.py --arms baseline --coverage --model haiku --out-dir OUT -j 8
python3 tools/qgen/compare.py --arms baseline --coverage --resume --out-dir OUT   # redo only failed specs

# the heavier pipeline arm (new author prompt + verification block + check/revise loop)
python3 tools/qgen/generate.py --arm pipeline --topics T003 --difficulty medium --count 4
```

The pipeline arm (`prompts/author.md`, `prompts/block_spec.md`) added almost nothing in quality on Claude
models at 2 to 5 times the cost; keep it only as an experiment. The app offers topics T001 to T015 only
(T016 to T020 have no bank questions).

## Ideas not built

- Auto-strip explanatory comments from stems and choices instead of regenerating.
- Rust parser repair for the missing-opening-quote JSON slip (matters only on gpt-oss).
- Direct-Anthropic Lambda handler as a new file (e.g. `lambda_functions/anthropic_gateway/`); Haiku 5.5
  rejects non-default temperature/top_p.
- A `--backend gateway` option for a real gpt-oss head-to-head (credentials supplied by the user via env vars).
