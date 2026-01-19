import { useEffect, useRef } from "react";
import { Loader2 } from "lucide-react";

interface StreamingPreviewProps {
  text: string;
  isComplete: boolean;
}

// Show the raw streaming buffer (reasoning tokens) without JSON parsing
export default function StreamingPreview({ text, isComplete }: StreamingPreviewProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom as content streams in
  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [text]);

  const hasContent = text.trim().length > 0;

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center gap-2 px-4 py-2 border-b bg-slate-50">
        {!isComplete && <Loader2 className="w-4 h-4 animate-spin text-primary" />}
        <span className="text-sm font-medium">
          {isComplete ? "✓ Generation complete" : "Generating..."}
        </span>
      </div>

      <div ref={containerRef} className="flex-1 overflow-auto p-4 bg-white">
        {hasContent ? (
          <pre className="code-block--reasoning whitespace-pre-wrap">
            {text}
          </pre>
        ) : (
          <div className="text-sm text-muted-foreground italic">
            Waiting for response...
          </div>
        )}
      </div>
    </div>
  );
}
