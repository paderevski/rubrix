import { Question } from "../types";
import { RefreshCw, Pencil, Trash2, Check } from "lucide-react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";
import { prism } from "react-syntax-highlighter/dist/esm/styles/prism";

interface QuestionCardProps {
  question: Question;
  index: number;
  onRegenerate: () => void;
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
  return (
    <div className="bg-white rounded-lg border shadow-sm overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 bg-secondary/50 border-b">
        <span className="font-medium text-foreground">
          Question {index + 1}
        </span>
        <div className="flex items-center gap-1">
          <button
            onClick={onRegenerate}
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

      {/* Content */}
      <div className="p-4">
        {/* Question Content (Markdown) */}
        <div className="prose prose-sm max-w-none mb-4">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            components={{
              // Syntax highlighted code blocks
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
                    customStyle={{ margin: "1rem 0", borderRadius: "0.5rem" }}
                  >
                    {String(children).replace(/\n$/, "")}
                  </SyntaxHighlighter>
                );
              },
              // Table styling
              table({ children }) {
                return (
                  <table className="border-collapse border border-slate-300 my-3">
                    {children}
                  </table>
                );
              },
              thead({ children }) {
                return <thead className="bg-slate-100">{children}</thead>;
              },
              th({ children }) {
                return (
                  <th className="border border-slate-300 px-3 py-1.5 text-left font-medium">
                    {children}
                  </th>
                );
              },
              td({ children }) {
                return (
                  <td className="border border-slate-300 px-3 py-1.5">
                    {children}
                  </td>
                );
              },
              // Remove extra margins from paragraphs
              p({ children }) {
                return <p className="my-2">{children}</p>;
              },
            }}
          >
            {question.content}
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
                  remarkPlugins={[remarkGfm]}
                  components={{
                    p({ children }) {
                      return <>{children}</>;
                    },
                    code({ children }) {
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
