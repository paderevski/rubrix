import { useEffect, useRef } from "react";
import { Eye, EyeOff, Loader2 } from "lucide-react";

interface StreamingQuestionCardProps {
  text: string;
  isComplete: boolean;
  showRaw: boolean;
  onToggleRaw: () => void;
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

  return (
    <div className="bg-white rounded-lg border shadow-sm overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 bg-secondary/50 border-b">
        <span className="font-medium text-foreground">Generating Question(s)</span>
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
          <p>
            Formatted question cards will appear here once response parsing completes.
          </p>
        </div>
      )}
    </div>
  );
}
