import { diffWordsWithSpace } from "diff";
import { AlertTriangle, CheckCircle2, Loader2, X } from "lucide-react";
import type { Question, QuestionReviewView, ReviewFinding } from "../types";

interface ReviewPanelProps {
  view: QuestionReviewView;
  /** Current question (to diff the proposed revision against). */
  question: Question;
  /** Live model output while the review runs. */
  liveText?: string;
  onApply: () => void;
  onDismiss: () => void;
}

interface ChangedPart {
  label: string;
  before: string;
  after: string;
}

const letter = (i: number) => String.fromCharCode(65 + i);

function changedParts(before: Question, after: Question): ChangedPart[] {
  const parts: ChangedPart[] = [];
  if (before.text !== after.text) {
    parts.push({ label: "Question text", before: before.text, after: after.text });
  }
  after.answers.forEach((answer, i) => {
    const old = before.answers[i];
    if (!old) return;
    if (old.text !== answer.text) {
      parts.push({ label: `Choice ${letter(i)}`, before: old.text, after: answer.text });
    }
    if ((old.explanation ?? "") !== (answer.explanation ?? "")) {
      parts.push({
        label: `Choice ${letter(i)} explanation`,
        before: old.explanation ?? "",
        after: answer.explanation ?? "",
      });
    }
  });
  if ((before.explanation ?? "") !== (after.explanation ?? "")) {
    parts.push({ label: "Solution", before: before.explanation ?? "", after: after.explanation ?? "" });
  }
  return parts;
}

const isShort = (s: string) => !s.includes("\n") && s.length <= 80;

function WordDiff({ before, after }: { before: string; after: string }) {
  // Short one-line values (typical answer choices) read better as a whole "old → new".
  if (isShort(before) && isShort(after)) {
    return (
      <div className="text-xs font-mono bg-white border rounded-md p-2 flex flex-wrap items-center gap-2">
        <del className="bg-red-100 text-red-800 rounded-sm px-1">{before || "(empty)"}</del>
        <span className="text-muted-foreground">→</span>
        <ins className="bg-green-100 text-green-900 no-underline rounded-sm px-1">{after || "(empty)"}</ins>
      </div>
    );
  }
  const pieces = diffWordsWithSpace(before, after);
  return (
    <pre className="text-xs leading-relaxed whitespace-pre-wrap break-words font-mono bg-white border rounded-md p-2 max-h-72 overflow-auto">
      {pieces.map((piece, i) =>
        piece.added ? (
          <ins key={i} className="bg-green-100 text-green-900 no-underline rounded-sm">
            {piece.value}
          </ins>
        ) : piece.removed ? (
          <del key={i} className="bg-red-100 text-red-800 rounded-sm">
            {piece.value}
          </del>
        ) : (
          <span key={i}>{piece.value}</span>
        )
      )}
    </pre>
  );
}

function FindingRow({ finding }: { finding: ReviewFinding }) {
  const isError = finding.severity === "error";
  return (
    <li className="flex items-start gap-2">
      <span
        className={`flex-shrink-0 mt-0.5 px-1.5 py-0.5 rounded text-[10px] font-semibold uppercase tracking-wide ${
          isError ? "bg-red-100 text-red-700" : "bg-amber-100 text-amber-800"
        }`}
      >
        {isError ? "Error" : "Warning"}
      </span>
      <span className="text-sm text-slate-800">
        {finding.summary}
        {finding.fields.length > 0 && (
          <span className="ml-1.5 text-xs text-muted-foreground">({finding.fields.join(", ")})</span>
        )}
      </span>
    </li>
  );
}

export default function ReviewPanel({ view, question, liveText, onApply, onDismiss }: ReviewPanelProps) {
  if (view.status === "running") {
    const tail = (liveText ?? "").trim().split("\n").slice(-6).join("\n");
    return (
      <div className="px-4 py-3 bg-indigo-50/70 border-b border-indigo-100">
        <div className="w-full max-w-4xl mx-auto space-y-2">
          <div className="flex items-center gap-2 text-sm font-medium text-indigo-900">
            <Loader2 className="w-4 h-4 animate-spin" />
            Reviewing — solving the question independently, then checking it…
          </div>
          {tail && (
            <pre className="text-[11px] leading-snug whitespace-pre-wrap break-words font-mono text-slate-600 max-h-28 overflow-hidden">
              {tail}
            </pre>
          )}
        </div>
      </div>
    );
  }

  const { result, stale } = view;
  const isOk = result.verdict === "ok";
  const parts = result.revised ? changedParts(question, result.revised) : [];
  const errorCount = result.findings.filter((f) => f.severity === "error").length;
  const warningCount = result.findings.length - errorCount;
  const summary = [
    errorCount ? `${errorCount} error${errorCount === 1 ? "" : "s"}` : null,
    warningCount ? `${warningCount} warning${warningCount === 1 ? "" : "s"}` : null,
  ]
    .filter(Boolean)
    .join(", ");

  return (
    <div
      className={`px-4 py-3 border-b ${
        isOk ? "bg-green-50/70 border-green-100" : "bg-amber-50/60 border-amber-100"
      }`}
    >
      <div className="w-full max-w-4xl mx-auto space-y-3">
        <div className="flex items-center gap-2">
          {isOk ? (
            <CheckCircle2 className="w-4 h-4 text-green-600 flex-shrink-0" />
          ) : (
            <AlertTriangle className="w-4 h-4 text-amber-600 flex-shrink-0" />
          )}
          <span className="text-sm font-medium text-slate-900">
            {isOk ? "Review found no problems" : `Review found ${summary || "issues"}`}
          </span>
          {result.solved_answer && (
            <span className="text-xs text-muted-foreground truncate">
              · Reviewer's answer: {result.solved_answer}
            </span>
          )}
          <button
            type="button"
            onClick={onDismiss}
            title="Dismiss review"
            aria-label="Dismiss review"
            className="ml-auto p-1 rounded hover:bg-black/5 text-muted-foreground hover:text-foreground"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {stale && (
          <p className="text-xs text-slate-700 bg-white/70 border rounded px-2 py-1">
            The question has changed since this review, so its changes can't be applied. Run Review again.
          </p>
        )}

        {result.findings.length > 0 && (
          <ul className="space-y-1.5">
            {result.findings.map((finding, i) => (
              <FindingRow key={i} finding={finding} />
            ))}
          </ul>
        )}

        {result.revision_problem && (
          <p className="text-xs text-slate-600">
            The reviewer's proposed fix didn't pass the app's checks ({result.revision_problem.replace(/^The proposed fix /, "")}), so no fix is offered. You can edit the question directly or run Review again.
          </p>
        )}

        {parts.length > 0 && !stale && (
          <div className="space-y-2">
            <div className="text-xs font-semibold uppercase tracking-wide text-slate-500">Proposed changes</div>
            {parts.map((part) => (
              <div key={part.label} className="space-y-1">
                <div className="text-xs font-medium text-slate-700">{part.label}</div>
                <WordDiff before={part.before} after={part.after} />
              </div>
            ))}
            <div className="flex items-center justify-end gap-2 pt-1">
              <span className="mr-auto text-xs text-muted-foreground">You can undo this with ⌘Z.</span>
              <button
                type="button"
                onClick={onDismiss}
                className="px-2.5 py-1 text-xs rounded border bg-white hover:bg-secondary transition-colors"
              >
                Dismiss
              </button>
              <button
                type="button"
                onClick={onApply}
                className="px-2.5 py-1 text-xs rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
              >
                Apply changes
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
