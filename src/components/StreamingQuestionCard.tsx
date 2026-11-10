import { useEffect, useRef } from "react";
import { Eye, EyeOff, Loader2 } from "lucide-react";

interface StreamingQuestionCardProps {
  text: string;
  isComplete: boolean;
  showRaw: boolean;
  onToggleRaw: () => void;
}

const STATUS_MAX_CHARS = 90;

/** One-line status derived from the raw stream: the latest reasoning paragraph while the
 *  model is thinking, then a progress line once it starts writing the questions. */
function streamStatus(text: string, isComplete: boolean): string {
  if (isComplete) return "Generation complete";
  const trimmed = text.trim();
  if (!trimmed) return "Waiting for response...";

  const responseIdx = text.lastIndexOf("[RESPONSE]");
  if (responseIdx >= 0) {
    // Each finished question object ends with its "cognitive_level" field.
    const done = (text.slice(responseIdx).match(/"cognitive_level"/g) ?? []).length;
    return done === 0 ? "Writing the questions..." : `Writing questions... (${done} finished)`;
  }

  const reasoningIdx = text.lastIndexOf("[REASONING]");
  const reasoning = reasoningIdx >= 0 ? text.slice(reasoningIdx + "[REASONING]".length) : text;
  if (reasoning.includes("[REASONING DONE]")) return "Writing the questions...";
  const paragraphs = reasoning
    .split(/\n\s*\n/)
    .map((p) => p.replace(/\s+/g, " ").trim())
    .filter(Boolean);
  const latest = paragraphs[paragraphs.length - 1];
  if (!latest) return "Thinking...";
  return latest.length > STATUS_MAX_CHARS
    ? `${latest.slice(0, STATUS_MAX_CHARS).trimEnd()}...`
    : latest;
}

export default function StreamingQuestionCard({
  text,
  isComplete,
  showRaw,
  onToggleRaw,
}: StreamingQuestionCardProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!showRaw) return;
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [text, showRaw]);

  const hasContent = text.trim().length > 0;
  const status = streamStatus(text, isComplete);

  return (
    <div className="bg-white rounded-lg border shadow-sm overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 bg-secondary/50 border-b">
        <div>
          <span className="font-medium text-foreground">Generating Question(s)</span>
        </div>
        <div className="flex items-center gap-2">
          {!isComplete && <Loader2 className="w-4 h-4 animate-spin text-primary" />}
          <button
            type="button"
            onClick={onToggleRaw}
            className="inline-flex items-center gap-1 px-2 py-1 text-xs rounded border hover:bg-secondary"
            title={showRaw ? "Show formatted placeholder" : "Show raw stream"}
          >
            {showRaw ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5" />}
            {showRaw ? "Formatted" : "Raw"}
          </button>
        </div>
      </div>

      {showRaw ? (
        <div ref={containerRef} className="max-h-72 overflow-auto overscroll-contain p-4 bg-white">
          {hasContent ? (
            <pre className="code-block--reasoning whitespace-pre-wrap">{text}</pre>
          ) : (
            <div className="text-sm text-muted-foreground italic">Waiting for response...</div>
          )}
        </div>
      ) : (
        <div className="p-4 text-sm text-muted-foreground bg-white">
          <p className="font-medium text-foreground mb-1">
            {isComplete ? "Generation complete" : "Generating..."}
          </p>
          <p className="truncate" title={status} aria-live="polite">
            {status}
          </p>
        </div>
      )}
    </div>
  );
}
