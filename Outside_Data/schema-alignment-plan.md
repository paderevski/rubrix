# AP CSA Schema Alignment Plan (draft)

Status: draft for review. No schema or bank files have been changed yet.

Reference: AP Computer Science A Course and Exam Description, Fall 2025 (College Board). Text was read from a copy saved during this session; page numbers below refer to the CED's own pages.
https://apcentral.collegeboard.org/pdf/ap-computer-science-a-course-and-exam-description.pdf

---

## 1. Summary

The schema's unit structure and official topic codes match the CED. Four things need work:

1. Inheritance and polymorphism topics are marked as legacy, but the CED explicitly excludes them from the course and exam.
2. The ethics topic `T015` is not an official CED topic. Its content belongs under CED topics 3.2 and 4.1.
3. The skills list (`SK001`–`SK023`) is custom and doesn't use the CED's five Computational Thinking Practices.
4. The schema has no learning-objective or essential-knowledge level, so questions can't be matched to exact CED statements.

---

## 2. What already matches

| Item | Schema | CED | Result |
|---|---|---|---|
| Unit 1 weight | 15–25% | 15–25% | match |
| Unit 2 weight | 25–35% | 25–35% | match |
| Unit 3 weight | 10–18% | 10–18% | match |
| Unit 4 weight | 30–40% | 30–40% | match |
| Topic codes 1.1–1.15, 2.1–2.12, 3.1–3.9, 4.1–4.17 | present | present | match |

Some topic titles could not be confirmed word-for-word because the PDF wraps them across lines. The codes are confirmed. Titles should be spot-checked before the schema is locked.

---

## 3. Findings

### F1. Inheritance and polymorphism are out of scope

The CED has an exclusion statement in its Unit 1 object content (near Topics 1.12–1.13, in the copy read for this plan): "Designing and implementing inheritance relationships are outside the scope of the AP Computer Science A course and exam." The terms polymorphism, abstract class, and interface do not appear in the course content.

Schema entries affected:
- Topics `T016`–`T019` (Inheritance, Polymorphism, Abstract Classes, Interfaces), marked `pre-2025`
- Subtopics `ST057`–`ST061`
- Unit `OLD_U9` and topics `OLD_9.1`–`OLD_9.7`, marked `pre-2025`

The schema's `item_status` uses `legacy` for "tests pre-2025 content; use only when a pre-2025 topic is requested." Because the CED excludes these concepts entirely, `retired` is the closer fit.

No question in either bank uses these tags, so the change is only to the schema.

### F2. `T015` "Ethical and Social Implications" is not an official topic

The CED places ethics and social-impact content in two places:
- Topic 3.2 "Impact of Program Design", Learning Objective 3.2.A:
  - EK 3.2.A.1: system reliability (the program performs as expected under stated conditions; programmers should maximize reliability by testing a variety of conditions).
  - EK 3.2.A.2: impacts of programs on society, economy, and culture.
  - EK 3.2.A.3: legal issues and intellectual property; open-source code vs. code that requires permission or purchase.
- Topic 4.1 "Ethical and Social Issues Around Data Collection": bias in data collection, and related analysis issues.

Schema entries affected:
- `T015` (topic) — no official equivalent
- `ST064` system_reliability (under `T015`) → should be under 3.2
- `ST065` data_bias (under `T015`) → should be under 4.1
- No open-source subtopic exists; one should be added under 3.2

### F3. Skills do not match the CED's practices

The CED defines five Computational Thinking Practices, with approximate multiple-choice weights:

| Practice | MC weight |
|---|---|
| 1. Design Code | 2–10% |
| 2. Develop Code | 22–38% |
| 3. Analyze Code | 37–53% |
| 4. Document Code and Computing Systems | 10–15% |
| 5. Use Computers Responsibly | 2–10% |

The schema's `SK001`–`SK023` (for example "Code Tracing", "Pattern Recognition") are not in the CED. Some map roughly to practices (code tracing is Practice 3, Analyze Code), but the mapping is not one-to-one.

### F4. No learning-objective or essential-knowledge level

The CED's codes (for example `3.2.A.3`, `1.A`, `2.B`) are the finest-grained official unit of content. The schema stops at topic and subtopic, so a question can't say which essential-knowledge statement it tests.

### F5. Duplicate topic systems

The general concept IDs `T001`–`T015` overlap with the official units. Examples:

| General ID | Schema name | Official home in CED |
|---|---|---|
| `T005` | Writing Classes | Unit 3 (3.3–3.9) |
| `T006` | Arrays | Unit 4 (4.3–4.5) |
| `T007` | ArrayList | Unit 4 (4.8–4.10) |
| `T008` | 2D Arrays | Unit 4 (4.11–4.13) |
| `T009` | Recursion | Unit 4 (4.16–4.17) |
| `T010` / `T011` | Searching / Sorting | Unit 4 (4.14–4.15) |
| `T014` | Data Sets and File I/O | Unit 4 (4.2, 4.6) |

Tags can disagree depending on which system the tagger used. For example, a recursion question tagged `T009` and one tagged `4.16` describe the same content with different IDs.

### F6. Scope exclusions are not recorded

The CED lists many explicit exclusions (for example, "outside the scope" notes on `++x`, on NaN handling, and on input from the user in some topics). The schema has no field for scope limits per concept, so a question that tests excluded material would not be flagged.

---

## 4. Proposed schema changes

Each change is numbered so you can approve or reject it individually.

**S1. Retire out-of-scope inheritance content.** Change `status` for `T016`–`T019`, `OLD_U9`, `OLD_9.x`, and `ST057`–`ST061` from `pre-2025` to `retired`. Keep the records for history.

**S2. Replace `T015` with official homes.** Mark `T015` as `deprecated` with a note pointing to 3.2 and 4.1. Move `ST064` under `3.2` and `ST065` under `4.1`.

**S3. Add an open-source subtopic.** Add `ST066` `open_source_and_intellectual_property` ("Open Source and Intellectual Property"), parent `3.2`. Source: EK 3.2.A.3.

**S4. Add a learning-objective and essential-knowledge field.** New optional field `ck_codes` (array of strings, e.g. `["3.2.A.3"]`). Start with the codes for the topics in the banks; fill the rest later.

**S5. Add a practice field.** New field `practices` (array of `P1`–`P5`) replacing or sitting alongside `skills`. Keep the `SK` list until a decision is made (see Decision D2).

**S6. Add a scope field.** Optional `scope_note` on subtopics where the CED excludes something. Fill later.

---

## 5. Affected bank items

Current tags that change under the proposed schema:

**Test 1 bank (`question-bank.json`)**

| Item | Current tag | Proposed tag |
|---|---|---|
| `pt2_q017` | topic `T015`, subtopic `ST064` | topic `3.2`, subtopic `ST064` (moved) |
| `pt2_q031` | topic `T015`, subtopic `ST065` | topic `4.1`, subtopic `ST065` (moved) |

**Test 2 bank (`question-bank-test-2.json`)**

| Item | Current tag | Proposed tag |
|---|---|---|
| `exam2_q013` (open source) | topic `T015`, no subtopic | topic `3.2`, subtopic `ST066`, ck `3.2.A.3` |
| `exam2_q021` (reliability) | topic `T015`, subtopic `ST064` | topic `3.2`, subtopic `ST064`, ck `3.2.A.1` |

Neither bank uses the inheritance topics (`T016`–`T019`), so S1 changes no items.

Skills: test 1 uses 16 distinct `SK` codes and test 2 uses 14. If S5 is adopted, every item needs a practice tag; this is a bulk task.

---

## 6. Decisions for you

- **D1. Retire or keep inheritance content?** Recommendation: retire (S1). Keeping it as `legacy` only makes sense if you want a pre-2025 practice bank.
- **D2. Keep the custom `SK` skills, replace them with the CED practices, or keep both?** Recommendation: keep both for now, add `practices` (S5), and decide on `SK` later.
- **D3. Keep the general topics `T001`–`T015` as an alias layer, or retire them?** Recommendation: keep them as aliases mapped to official codes, so existing tags still resolve. Retire later once every item has an official code.
- **D4. How precise should `ck_codes` be?** Full essential-knowledge codes (`3.2.A.3`) or just learning objectives (`3.2.A`)? Recommendation: essential-knowledge codes, since they're what the CED tests against.
- **D5. Is the rubric topic title list (Section 2) good enough, or should every title be checked against the PDF first?** Recommendation: spot-check the titles before locking the schema.

---

## 7. Steps once approved

1. Back up `question-schema.json`, `question-bank.json`, and `question-bank-test-2.json`.
2. Apply S1–S3 to the schema and bump `schema_version` to `2.4.0`.
3. Update the affected items in both banks (Section 5). Validate every ID against the new schema.
4. Apply S4 and S5 once D2 and D4 are decided. Add `ck_codes` and `practices` to the items.
5. Re-run the validation script (all topic, subtopic, skill, difficulty, cognitive, and status references must resolve) and the answer-key check against both official keys.
6. Record the changes in the schema's change log.

---

## 8. Not covered by this plan

- Full mapping of every subtopic to a CED essential-knowledge code. This needs the full CED text reviewed line by line.
- Whether the Bloom levels (`B1`–`B6`) should be replaced with anything from the CED. The CED does not use Bloom levels, so they're kept as-is.
- The FRQ question types (`QT03`–`QT08`). Not relevant to the MCQ banks.
- Whether the item explanations need updating for the new tags. They don't depend on tags, so they stay.

---

## 9. Source

- [AP Computer Science A Course and Exam Description (College Board PDF, Fall 2025)](https://apcentral.collegeboard.org/pdf/ap-computer-science-a-course-and-exam-description.pdf)
