import { Question } from "../types";
import { RefreshCw, Pencil, Trash2, Check, ChevronDown, ChevronUp } from "lucide-react";
import { useEffect, useState } from "react";
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

const questionMarkdownComponents = {
  code(props: any) {
    const { inline, className, children, ...rest } = props;
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
};

function normalizeMathDelimiters(content: string) {
  if (!content) return "";
  // Only turn escaped newlines into real newlines when they are not the start of a LaTeX command
  // (e.g., `\ne` should stay as not-equal, not become a newline).
  return content
    .replace(/\\\(/g, "$")
    .replace(/\\\)/g, "$")
    .replace(/\\\[/g, "$$")
    .replace(/\\\]/g, "$$")
}

function RichMarkdown({
  content,
  className,
  components,
}: {
  content: string;
  className?: string;
  components?: any;
}) {
  const normalized = normalizeMathDelimiters(content);
  return (
    <ReactMarkdown
      className={className}
      remarkPlugins={[remarkGfm, remarkMath]}
      rehypePlugins={[rehypeRaw, rehypeKatex]}
      components={{ ...questionMarkdownComponents, ...components }}
    >
      {normalized}
    </ReactMarkdown>
  );
}

function formatExplanation(content: string) {
  if (!content) return "";

  const stepPattern = /(^|\n)(Step\s+\d+(?:\s*[—-]\s*[^\n:]+)?):?\s*/g;

  return content.replace(stepPattern, (_match, prefix, stepLabel) => {
    const leadingNewline = prefix === "\n" ? "\n" : "";
    const heading = `### ${stepLabel}`;
    return `${leadingNewline}${heading}\n\n`;
  });
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
  const [showExplanation, setShowExplanation] = useState(false);

  const explanationContent = question.explanation?.trim() ?? "";
  const formattedExplanation = formatExplanation(explanationContent);
  const hasExplanation = Boolean(formattedExplanation);

  useEffect(() => {
    if (!question) return;
    console.log("Debug:", question.explanation);
    console.log("Formatted Explanation:", formattedExplanation);
  }, [question, formattedExplanation]);

  const handleRegenerate = () => {
    const trimmed = instructions.trim();

    if (trimmed) {
      onRegenerate(trimmed);
      setInstructions("");
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
          <RichMarkdown content={question.text} />
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
              <div className="flex-1 text-sm">
                <RichMarkdown
                  content={answer.text}
                  className="prose prose-sm max-w-none prose-p:my-1 prose-ul:my-1 prose-ol:my-1 prose-pre:my-2"
                  components={{
                    p({ children }: any) {
                      return <>{children}</>;
                    },
                  }}
                />
              </div>
              {answer.is_correct && (
                <Check className="w-4 h-4 text-green-600 flex-shrink-0" />
              )}
            </div>
          ))}
        </div>

        {hasExplanation && (
          <div className="mt-4 border border-slate-200 rounded-lg">
            <button
              type="button"
              onClick={() => setShowExplanation((prev) => !prev)}
              className="w-full flex items-center justify-between px-4 py-2 text-sm font-medium text-foreground bg-secondary/50 hover:bg-secondary transition-colors"
            >
              <span>Explanation</span>
              {showExplanation ? (
                <ChevronUp className="w-4 h-4" />
              ) : (
                <ChevronDown className="w-4 h-4" />
              )}
            </button>
            {showExplanation && (
              <div className="px-4 py-3 border-t border-slate-200 bg-white prose prose-sm max-w-none">
                <RichMarkdown content={formattedExplanation} />
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
