Kinds (all string fields are Java source or text; choice ids are a, b, c, ... in order):

- `{"kind":"run"}` — runs the stem and compares what it prints with the choices. Add `"observe":"<variable or element, e.g. a[2]>"` to compare a value instead of stdout,
  `"count_lines":true` for "how many times is X printed", or `"claims":{"a":"<python expression over out (stdout string) and lines (int)>"}` for prose choices about the output.
  Optional fields for anything the stem leaves off-screen: `"prelude"` (stub classes), `"members"` (static methods), `"before"` (declarations), `"after"` (statements, e.g. a call), `"rewrite":[["old","new"]]`.
- `{"kind":"substitute","cases":[{"before":"","after":"System.out.println(...);","expect_output":"<INTENDED result>"}]}` — each choice's code replaces `/* missing code */`
  (or replaces the stem's fragment if there is no hole) and is run on every case. Use several cases, including edge cases. `"expect":"fails"` for "which cannot be used".
- `{"kind":"patch","variants":{"a":[["old text","new text"]]},"expect":"compiles"}` or with `"expect_output":"..."` / `"cases"` — each choice proposes a change to the stem code;
  give the exact edit for each choice, applied to the stem text. The unpatched code must fail the check.
- `{"kind":"witness","cases":{"a":{"before":"","after":"System.out.println(f(...));","expect_output":"<INTENDED result>"}}}` — each choice names an input condition; the code should
  misbehave (differ from the intended result or throw) on exactly the keyed choice's input.
- `{"kind":"oracle","loops":"for (...) for (...)","subject":"<java expr: the stem's result>","show":"<expr>","choices":{"a":"<java expr equal to subject exactly when this choice is true>"}}` —
  prose choices ("describes the conditions...") tested over an input grid; may add `"fragment"` and `"prelude"`.
- `{"kind":"equivalent","after":"System.out.println(x);"}` — each choice is a code segment that must leave the same result as the stem's.
- `{"kind":"compile","expect":"compiles"|"fails","prelude":"<stub classes>","before":"<declarations>","template":"int x = {};"}` — each choice is a statement (template wraps it).
- `{"kind":"none","reason":"..."}` — purely conceptual.

Which kind: value_trace -> run; code_selection/algorithm_selection/code_completion -> substitute; equivalence -> equivalent (code) or oracle (conditions);
error_analysis/modification -> patch or witness; conceptual -> compile or none.
