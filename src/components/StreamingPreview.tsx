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

  // Pre-process text to add blank lines between answer choices
  const processedText = text.replace(/\n([a-e])\.\s+/g, '\n\n$1. ');

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
                // Large, spaced question headers
                h1({ children }) {
                  return (
                    <h1 className="text-2xl font-bold text-foreground mt-8 mb-4 pb-2 border-b-2 border-primary/20">
                      {children}
                    </h1>
                  );
                },
                h2({ children }) {
                  // Special styling for "Choices" header
                  const text = String(children);
                  if (text.toLowerCase().includes('choices')) {
                    return (
                      <h2 className="text-base font-semibold text-primary mt-4 mb-3">
                        {children}
                      </h2>
                    );
                  }
                  // Regular h2 (Question header)
                  return (
                    <h2 className="text-xl font-bold text-foreground mt-8 mb-4 pb-2 border-b border-slate-300">
                      {children}
                    </h2>
                  );
                },
                h3({ children }) {
                  return (
                    <h3 className="text-lg font-semibold text-foreground mt-6 mb-3">
                      {children}
                    </h3>
                  );
                },
                // Code blocks with syntax highlighting
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
                // Format answer choice paragraphs
                p({ children }) {
                  // Extract text content for pattern matching
                  const extractText = (node: any): string => {
                    if (typeof node === 'string') return node;
                    if (Array.isArray(node)) return node.map(extractText).join('');
                    if (node?.props?.children) return extractText(node.props.children);
                    return '';
                  };

                  const text = extractText(children);
                  const answerMatch = text.match(/^([a-e])\.\s+/);

                  if (answerMatch) {
                    const letter = answerMatch[1];

                    // Remove the "a. " prefix from children
                    const processChildren = (node: any): any => {
                      if (typeof node === 'string') {
                        return node.replace(/^[a-e]\.\s+/, '');
                      }
                      if (Array.isArray(node)) {
                        return node.map((child, i) => {
                          if (i === 0 && typeof child === 'string') {
                            return child.replace(/^[a-e]\.\s+/, '');
                          }
                          return child;
                        });
                      }
                      return node;
                    };

                    return (
                      <div className="flex items-start gap-2 mb-2 pl-2">
                        <span className="inline-flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary font-semibold text-sm flex-shrink-0 mt-0.5">
                          {letter}
                        </span>
                        <span className="flex-1 pt-0.5">{processChildren(children)}</span>
                      </div>
                    );
                  }

                  return <p className="my-2">{children}</p>;
                },
                // Strong/bold text styling
                strong({ children }) {
                  return <strong className="font-semibold text-foreground">{children}</strong>;
                },
                // List items - special styling for distractor analysis
                li({ children }) {
                  // Extract text content for pattern matching
                  const extractText = (node: any): string => {
                    if (typeof node === 'string') return node;
                    if (Array.isArray(node)) return node.map(extractText).join('');
                    if (node?.props?.children) return extractText(node.props.children);
                    return '';
                  };

                  const text = extractText(children);
                  const distractorMatch = text.match(/^([a-e]):\s+/);

                  if (distractorMatch) {
                    const letter = distractorMatch[1];

                    // Remove the "a: " prefix from children
                    const processChildren = (node: any): any => {
                      if (typeof node === 'string') {
                        return node.replace(/^[a-e]:\s+/, '');
                      }
                      if (Array.isArray(node)) {
                        return node.map((child, i) => {
                          if (i === 0 && typeof child === 'string') {
                            return child.replace(/^[a-e]:\s+/, '');
                          }
                          return child;
                        });
                      }
                      return node;
                    };

                    return (
                      <li className="flex items-start gap-2 my-2" style={{ listStyle: 'none', marginLeft: '-1rem' }}>
                        <span className="inline-flex items-center justify-center w-6 h-6 rounded-full bg-red-100 text-red-700 font-semibold text-sm flex-shrink-0 mt-0.5">
                          {letter}
                        </span>
                        <span className="flex-1">{processChildren(children)}</span>
                      </li>
                    );
                  }

                  return <li className="my-1">{children}</li>;
                },
                ul({ children }) {
                  return <ul className="my-2 ml-4 list-disc">{children}</ul>;
                },
                ol({ children }) {
                  return <ol className="my-2 ml-4 list-decimal">{children}</ol>;
                },
                // Horizontal rules
                hr() {
                  return <hr className="my-6 border-slate-300" />;
                },
              }}
            >
              {processedText}
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