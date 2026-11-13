# TODO

Larger pieces of work that are planned but not started.

## Generated-question history (SQLite)

Generated questions currently live only in the open document; once deleted or replaced they are gone. Keep a local history so old questions can be reused, promoted to the bank, and used to avoid duplicates.

**Approach:** keep question banks as JSON (curated, hand-edited, bundled), and add a local SQLite database for history.

- One file in the app data directory (e.g. `history.sqlite`), accessed from Rust with `rusqlite` (`bundled` feature), exposed through Tauri commands. Schema versioned with `PRAGMA user_version`.
- One row per generated question: question JSON, subject, topics/subtopics, difficulty, form-model example id, model, effort, cost, timestamp, and a status: `generated` -> `kept` / `deleted` / `exported` -> `promoted`. Exporting a quiz marks its questions as exported.

**Build in this order:**

1. [ ] History table, and record every generated and regenerated question (useful on its own).
2. [ ] "Promote to bank": copy a good question into the bank JSON as a new example. Helps subtopics with few examples.
3. [ ] Duplicate avoidance: add recent same-subtopic questions from history (exported/kept, not deleted) to the "Questions Already in This Set" section, with a setting to turn it off when deliberately regenerating the same quiz.
4. [ ] Optional: near-duplicate check after generation using SQLite full-text search, retrying once on a close match.
5. [ ] Optional: quality tracking, e.g. which form-model examples or subtopics lead to deleted questions, and keep rates by effort setting.

**Notes:** history is per computer. Sharing across machines or teachers would need a server-side store and is out of scope for now.

## Per-subtopic skill list for thin example banks

When a subtopic has one or two bank examples, new questions repeat the same content (e.g. merge sort's two examples share the same `mergeSortHelper` code, so new questions reuse it). Many subtopics have three or fewer examples.

- [ ] Add 6-12 distinct assessable skills per subtopic to `question-schema.json`, generated once with a script (one model call per subtopic) and reviewed by hand.
- [ ] The app assigns the least-used skill per question (like subtopic selection) and the prompt asks for it; examples then only calibrate style and difficulty.
- [ ] Pilot on Sorting and other thin subtopics before covering the whole schema.

Related: promoting good generated questions to the bank (above) also grows thin subtopics over time.
