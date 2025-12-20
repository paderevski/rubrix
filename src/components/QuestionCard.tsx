import { Question } from "../types";
import { RefreshCw, Pencil, Trash2, Check, ChevronDown, ChevronUp } from "lucide-react";
import { useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import rehypeRaw from "rehype-raw";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { prism } from "react-syntax-highlighter/dist/esm/styles/prism";
import "katex/dist/katex.min.css";

interface QuestionCardProps {
  question: Question;
  index: number;
  onRegenerate: (instructions?: string) => void;
  onEdit: () => void;
  onDelete: () => void;
}

export default function QuestionCard({
  question,
  index,
  onRegenerate,
  onEdit,
  onDelete,
}: QuestionCardProps) {
  const [showInstructions, setShowInstructions] = useState(false);
  const [instructions, setInstructions] = useState("");

  const handleRegenerate = () => {
    if (showInstructions && instructions.trim()) {
      onRegenerate(instructions.trim());
      setInstructions("");
      setShowInstructions(false);
    } else {
      onRegenerate();
    }
  };

  return (
    <div className="bg-white rounded-lg border shadow-sm overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 bg-secondary/50 border-b">
        <span className="font-medium text-foreground">
          Question {index + 1}
        </span>
        <div className="flex items-center gap-1">
          <button
            onClick={() => setShowInstructions(!showInstructions)}
            className={`p-1.5 rounded hover:bg-secondary transition-colors ${
              showInstructions
                ? "text-primary bg-secondary"
                : "text-muted-foreground hover:text-foreground"
            }`}
            title="Add regeneration instructions"
          >
            {showInstructions ? (
              <ChevronUp className="w-4 h-4" />
            ) : (
              <ChevronDown className="w-4 h-4" />
            )}
          </button>
          <button
            onClick={handleRegenerate}
            className="p-1.5 rounded hover:bg-secondary text-muted-foreground hover:text-foreground transition-colors"
            title="Regenerate"
          >
            <RefreshCw className="w-4 h-4" />
          </button>
          <button
            onClick={onEdit}
            className="p-1.5 rounded hover:bg-secondary text-muted-foreground hover:text-foreground transition-colors"
            title="Edit"
          >
            <Pencil className="w-4 h-4" />
          </button>
          <button
            onClick={onDelete}
            className="p-1.5 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors"
            title="Delete"
          >
            <Trash2 className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Collapsible Instructions Panel */}
      {showInstructions && (
        <div className="px-4 py-3 bg-blue-50 border-b border-blue-200">
          <label className="block text-sm font-medium text-slate-700 mb-2">
            Custom instructions for regeneration:
          </label>
          <textarea
            value={instructions}
            onChange={(e) => setInstructions(e.target.value)}
            placeholder="e.g., Make it easier, focus on arrays instead of loops, add more steps in explanation..."
            className="w-full px-3 py-2 text-sm border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent resize-none"
            rows={3}
          />
          <p className="text-xs text-slate-600 mt-1">
            These instructions will be included when regenerating this question.
          </p>
        </div>
      )}

      {/* Content */}
      <div className="p-4">
        {/* Question Text (with HTML, Markdown, LaTeX, and code blocks) */}
        <div className="mb-4 prose prose-sm max-w-none">
          <ReactMarkdown
            remarkPlugins={[remarkGfm, remarkMath]}
            rehypePlugins={[rehypeRaw, rehypeKatex]}
            components={{
              // Code blocks
              code(props: any) {
                const { node, inline, className, children, ...rest } = props;
                const match = /language-(\w+)/.exec(className || "");
                return !inline && match ? (
                  <SyntaxHighlighter
                    style={prism}
                    language={match[1]}
                    PreTag="div"
                    customStyle={{ margin: 0, borderRadius: "0.5rem" }}
                  >
                    {String(children).replace(/\n$/, "")}
                  </SyntaxHighlighter>
                ) : (
                  <code className="px-1.5 py-0.5 bg-slate-100 text-slate-800 rounded text-sm font-mono" {...rest}>
                    {children}
                  </code>
                );
              },
            }}
          >
            {question.text}
          </ReactMarkdown>
        </div>

        {/* Answers */}
        <div className="space-y-2">
          {question.answers.map((answer, i) => (
            <div
              key={i}
              className={`flex items-start gap-2 px-3 py-2 rounded-md ${
                answer.is_correct
                  ? "bg-green-50 border border-green-200"
                  : "bg-secondary/50"
              }`}
            >
              <span className="flex-shrink-0 w-6 h-6 flex items-center justify-center rounded-full text-xs font-medium bg-white border">
                {String.fromCharCode(65 + i)}
              </span>
              <span className="flex-1 text-sm">
                <ReactMarkdown
                  remarkPlugins={[remarkGfm, remarkMath]}
                  rehypePlugins={[rehypeRaw, rehypeKatex]}
                  components={{
                    p({ children }: any) {
                      return <>{children}</>;
                    },
                    code({ children }: any) {
                      return (
                        <code className="px-1 py-0.5 bg-slate-100 text-slate-800 rounded text-xs font-mono">
                          {children}
                        </code>
                      );
                    },
                  }}
                >
                  {answer.text}
                </ReactMarkdown>
              </span>
              {answer.is_correct && (
                <Check className="w-4 h-4 text-green-600 flex-shrink-0" />
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
