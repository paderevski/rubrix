import { useEffect, useRef } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { prism } from "react-syntax-highlighter/dist/esm/styles/prism";
import { Loader2 } from "lucide-react";

interface StreamingPreviewProps {
  text: string;
  isComplete: boolean;
}

export default function StreamingPreview({ text, isComplete }: StreamingPreviewProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  
  // Auto-scroll to bottom as content streams in
  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [text]);

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex items-center gap-2 px-4 py-2 border-b bg-slate-50">
        {!isComplete && <Loader2 className="w-4 h-4 animate-spin text-primary" />}
        <span className="text-sm font-medium">
          {isComplete ? "✓ Generation complete" : "Generating..."}
        </span>
        <span className="text-xs text-muted-foreground ml-auto">
          {text.length} characters
        </span>
      </div>
      
      {/* Streaming content */}
      <div 
        ref={containerRef}
        className="flex-1 overflow-auto p-4 bg-white"
      >
        {text ? (
          <div className="prose prose-sm max-w-none">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                code({ node, className, children, ...props }) {
                  const match = /language-(\w+)/.exec(className || "");
                  const isInline = !match && !className;

                  return isInline ? (
                    <code
                      className="px-1.5 py-0.5 bg-slate-100 text-slate-800 rounded text-sm font-mono"
                      {...props}
                    >
                      {children}
                    </code>
                  ) : (
                    <SyntaxHighlighter
                      style={prism}
                      language={match?.[1] || "java"}
                      PreTag="div"
                      customStyle={{ margin: "1rem 0", borderRadius: "0.5rem", fontSize: "0.875rem" }}
                    >
                      {String(children).replace(/\n$/, "")}
                    </SyntaxHighlighter>
                  );
                },
                p({ children }) {
                  return <p className="my-2">{children}</p>;
                },
              }}
            >
              {text}
            </ReactMarkdown>
            
            {/* Cursor indicator when still streaming */}
            {!isComplete && (
              <span className="inline-block w-2 h-5 bg-primary animate-pulse ml-1" />
            )}
          </div>
        ) : (
          <div className="flex items-center justify-center h-full text-muted-foreground">
            <Loader2 className="w-6 h-6 animate-spin mr-2" />
            Waiting for response...
          </div>
        )}
      </div>
    </div>
  );
}
