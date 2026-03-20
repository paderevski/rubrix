import { Question } from "../types";
import { RefreshCw, Pencil, Trash2, Check, ChevronDown, ChevronUp, Eye, EyeOff } from "lucide-react";
import { useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import rehypeRaw from "rehype-raw";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { ghcolors } from "react-syntax-highlighter/dist/esm/styles/prism";
import "katex/dist/katex.min.css";

interface QuestionCardProps {
  question: Question;
  index: number;
  topicMetaById?: Record<string, { label: string; kind: "topic" | "subtopic" }>;
  rawText?: string;
  liveRawText?: string;
  isRegenerating?: boolean;
  isRegenerationDisabled?: boolean;
  onRegenerate: (instructions?: string) => void;
  onEdit: () => void;
  onDelete: () => void;
}

const questionMarkdownComponents = {
  p({ children }: any) {
    return <div className="prose-p:my-2">{children}</div>;
  },
  code(props: any) {
    const { inline, className, children, ...rest } = props;
    const textContent = Array.isArray(children)
      ? children.join("")
      : typeof children === "string"
      ? children
      : String(children ?? "");
    const match = /language-(\w+)/.exec(className || "");
    return !inline && match ? (
      <SyntaxHighlighter
        style={ghcolors}
        language={match[1]}
        PreTag="div"
        className="code-block--question"
        customStyle={{ margin: 0, borderRadius: "0.5rem" }}
      >
        {textContent.replace(/\n$/, "")}
      </SyntaxHighlighter>
    ) : (
      <code className="px-1.5 py-0.5 bg-slate-100 text-slate-800 rounded text-sm font-mono" {...rest}>
        {textContent}
      </code>
    );
  },
};

function normalizeMathDelimiters(content: string) {
  if (!content) return "";
  const normalizedEscapes = content.replace(/\\\\(?=[A-Za-z])/g, "\\");
  const envPattern =
    /(^|\n)([ \t]*)(\\{1,2}begin\{(align\*?|aligned|alignat\*?|gather\*?|multline\*?|split|eqnarray\*?|array|cases)\}[\s\S]*?\\{1,2}end\{\4\})(?=\n|$)/g;

  const wrapBareMathEnvironments = (input: string) =>
    input.replace(envPattern, (fullMatch, lineStart, indent, block, _envName, offset, source) => {
      const before = source.slice(0, offset).trimEnd();
      const after = source.slice(offset + fullMatch.length).trimStart();

      // If this block is already inside a $$...$$ region, keep it unchanged.
      if (before.endsWith("$$") && after.startsWith("$$")) {
        return fullMatch;
      }

      return `${lineStart}${indent}$$\n${block}\n$$`;
    });

  // Only turn escaped newlines into real newlines when they are not the start of a LaTeX command
  // (e.g., `\ne` should stay as not-equal, not become a newline).
  return wrapBareMathEnvironments(normalizedEscapes)
    .replace(/\\\(/g, "$")
    .replace(/\\\)/g, "$")
    .replace(/(^|[^\\])\\\[/g, "$1$$")
    .replace(/(^|[^\\])\\\]/g, "$1$$")
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

function inferTopicKind(id: string): "topic" | "subtopic" | null {
  if (/^U\d+$/i.test(id) || /^TOPIC_?/i.test(id)) return "topic";
  if (/^T\d+$/i.test(id) || /^SUBTOPIC_?/i.test(id)) return "subtopic";
  return null;
}

function normalizeDifficultyLabel(value?: string): string | null {
  if (!value) return null;
  const normalized = value.trim().toLowerCase();
  if (!normalized) return null;
  if (normalized === "easy" || normalized === "medium" || normalized === "hard") {
    return normalized[0].toUpperCase() + normalized.slice(1);
  }
  return value;
}

export default function QuestionCard({
  question,
  index,
  topicMetaById = {},
  rawText,
  liveRawText,
  isRegenerating = false,
  isRegenerationDisabled = false,
  onRegenerate,
  onEdit,
  onDelete,
}: QuestionCardProps) {
  const [showInstructions, setShowInstructions] = useState(false);
  const [instructions, setInstructions] = useState("");
  const [showExplanation, setShowExplanation] = useState(false);
  const [showRubric, setShowRubric] = useState(false);
  const [showRaw, setShowRaw] = useState(false);

  const iconButtonBase = "p-1.5 rounded transition-colors";
  const iconButtonNeutral =
    `${iconButtonBase} hover:bg-secondary text-muted-foreground hover:text-foreground`;

  const explanationContent = question.explanation?.trim() ?? "";
  const formattedExplanation = formatExplanation(explanationContent);
  const hasSolution = Boolean(formattedExplanation);
  const rubricContent = question.rubric?.trim() ?? "";
  const hasRubric = Boolean(rubricContent);
  const hasAnswers = Array.isArray(question.answers) && question.answers.length > 0;
  const liveRaw = liveRawText?.trim() ?? "";
  const storedRaw = rawText?.trim() ?? "";
  const activeRaw = liveRaw || storedRaw;
  const hasRawText = Boolean(activeRaw);

  const topicIds = question.topics ?? [];
  const topicCandidates = topicIds.map((id) => {
    const meta = topicMetaById[id];
    const kind = meta?.kind ?? inferTopicKind(id);
    return {
      id,
      kind,
      label: meta?.label ?? id,
    };
  });
  const topicChip = topicCandidates.find((item) => item.kind === "topic") ?? topicCandidates[0] ?? null;
  const subtopicChip = topicCandidates.find(
    (item) => item.kind === "subtopic" && item.id !== topicChip?.id
  ) ?? null;
  const difficultyChip = normalizeDifficultyLabel(question.difficulty);

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
      <div className="px-4 py-3 bg-sky-50/70 border-b border-sky-100">
        <div className="w-full max-w-4xl mx-auto flex items-center justify-between">
          <span className="font-medium text-foreground">
            Question {index + 1}
          </span>
          <div className="flex items-center gap-1.5 mr-auto ml-3">
            {topicChip && (
              <span className="px-2 py-0.5 rounded-full text-[11px] font-medium bg-slate-100 text-slate-700">
                Topic: {topicChip.label}
              </span>
            )}
            {subtopicChip && (
              <span className="px-2 py-0.5 rounded-full text-[11px] font-medium bg-blue-50 text-blue-700">
                Subtopic: {subtopicChip.label}
              </span>
            )}
            {difficultyChip && (
              <span className="px-2 py-0.5 rounded-full text-[11px] font-medium bg-amber-50 text-amber-700">
                Difficulty: {difficultyChip}
              </span>
            )}
          </div>
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
              disabled={isRegenerationDisabled}
              className={iconButtonNeutral}
              title="Regenerate"
            >
              <RefreshCw className={`w-4 h-4 ${isRegenerating ? "animate-spin" : ""}`} />
            </button>
            {hasRawText && (
              <button
                onClick={() => setShowRaw((prev) => !prev)}
                className={`p-1.5 rounded hover:bg-secondary transition-colors ${
                  showRaw
                    ? "text-primary bg-secondary"
                    : "text-muted-foreground hover:text-foreground"
                }`}
                title={showRaw ? "Show formatted view" : "Show raw view"}
              >
                {showRaw ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
              </button>
            )}
            <button
              onClick={onEdit}
              className={iconButtonNeutral}
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
      </div>

      {/* Collapsible Instructions Panel */}
      {showInstructions && (
        <div className="px-4 py-3 bg-blue-50 border-b border-blue-200">
          <div className="w-full max-w-4xl mx-auto">
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
        </div>
      )}

      {/* Content */}
      <div className="p-4">
        <div className="w-full max-w-4xl mx-auto">
          {showRaw && hasRawText ? (
            <pre className="text-xs leading-relaxed max-h-80 overflow-auto bg-slate-950 text-slate-100 rounded-lg p-3 border border-slate-800 whitespace-pre-wrap">
              {activeRaw}
            </pre>
          ) : (
            <>
          {/* Question Text (with HTML, Markdown, LaTeX, and code blocks) */}
          <div className="prose max-w-none mb-4">
            <RichMarkdown content={question.text} />
          </div>

        {/* Answers (MCQ) */}
        {hasAnswers && (
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
        )}

          {hasSolution && (
            <div className="mt-4 border border-slate-200 rounded-lg">
              <button
                type="button"
                onClick={() => setShowExplanation((prev) => !prev)}
                className="w-full flex items-center justify-between px-4 py-2 text-sm font-medium text-foreground bg-secondary/50 hover:bg-secondary transition-colors"
              >
                <span>Solution</span>
                {showExplanation ? (
                  <ChevronUp className="w-4 h-4" />
                ) : (
                  <ChevronDown className="w-4 h-4" />
                )}
              </button>
              {showExplanation && (
                <div className="prose px-4 py-3 border-t border-slate-200 bg-white">
                  <RichMarkdown content={formattedExplanation} />
                </div>
              )}
            </div>
          )}

          {hasRubric && (
            <div className="mt-4 border border-slate-200 rounded-lg">
              <button
                type="button"
                onClick={() => setShowRubric((prev) => !prev)}
                className="w-full flex items-center justify-between px-4 py-2 text-sm font-medium text-foreground bg-secondary/50 hover:bg-secondary transition-colors"
              >
                <span>Rubric</span>
                {showRubric ? (
                  <ChevronUp className="w-4 h-4" />
                ) : (
                  <ChevronDown className="w-4 h-4" />
                )}
              </button>
              {showRubric && (
                <div className="prose px-4 py-3 border-t border-slate-200 bg-white">
                  <RichMarkdown content={rubricContent} />
                </div>
              )}
            </div>
          )}
            </>
          )}
        </div>
      </div>
    </div>
  );
}
