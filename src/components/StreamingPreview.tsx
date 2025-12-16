import { useEffect, useRef, useMemo } from "react";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { prism } from "react-syntax-highlighter/dist/esm/styles/prism";
import { Loader2 } from "lucide-react";

interface StreamingPreviewProps {
  text: string;
  isComplete: boolean;
}

interface PartialQuestion {
  stem?: string;
  code?: string;
  answers?: Array<{ text?: string; is_correct?: boolean; explanation?: string }>;
  explanation?: string;
  distractors?: string;
  isStreaming?: boolean; // Currently being streamed
}

// Helper to unescape JSON string values
function unescapeJson(s: string): string {
  return s
    .replace(/\\n/g, '\n')
    .replace(/\\"/g, '"')
    .replace(/\\t/g, '\t')
    .replace(/\\\\/g, '\\');
}

// Extract a partial or complete string value from a field
function extractFieldValue(text: string, fieldName: string): string | undefined {
  const pattern = new RegExp(`"${fieldName}"\\s*:\\s*"`, 'g');
  const match = pattern.exec(text);
  if (!match) return undefined;

  // Find the value starting after the opening quote
  const valueStart = match.index + match[0].length;
  let i = valueStart;
  let value = '';

  while (i < text.length) {
    const char = text[i];
    if (char === '\\' && i + 1 < text.length) {
      // Escape sequence - include both chars
      value += char + text[i + 1];
      i += 2;
    } else if (char === '"') {
      // End of string
      break;
    } else {
      value += char;
      i++;
    }
  }

  return value.length > 0 ? unescapeJson(value) : undefined;
}

// Extract answers from partial text
function extractAnswers(text: string): Array<{ text?: string; is_correct?: boolean; explanation?: string }> {
  const answers: Array<{ text?: string; is_correct?: boolean; explanation?: string }> = [];

  const answersStart = text.indexOf('"answers"');
  if (answersStart === -1) return answers;

  const arrayStart = text.indexOf('[', answersStart);
  if (arrayStart === -1) return answers;

  // Extract content after the [
  const afterArray = text.substring(arrayStart + 1);

  // Find individual answer objects
  let depth = 0;
  let objStart = -1;

  for (let i = 0; i < afterArray.length; i++) {
    const char = afterArray[i];
    if (char === '{') {
      if (depth === 0) objStart = i;
      depth++;
    } else if (char === '}') {
      depth--;
      if (depth === 0 && objStart !== -1) {
        const answerText = afterArray.substring(objStart, i + 1);
        const answer: { text?: string; is_correct?: boolean; explanation?: string } = {};

        answer.text = extractFieldValue(answerText, 'text');
        const correctMatch = answerText.match(/"is_correct"\s*:\s*(true|false)/);
        if (correctMatch) {
          answer.is_correct = correctMatch[1] === 'true';
        }
        answer.explanation = extractFieldValue(answerText, 'explanation');

        if (answer.text) {
          answers.push(answer);
        }
        objStart = -1;
      }
    } else if (char === ']' && depth === 0) {
      // End of answers array
      break;
    }
  }

  // Also try to capture an in-progress answer
  if (depth > 0 && objStart !== -1) {
    const partialAnswer = afterArray.substring(objStart);
    const answer: { text?: string; is_correct?: boolean; explanation?: string } = {};
    answer.text = extractFieldValue(partialAnswer, 'text');
    const correctMatch = partialAnswer.match(/"is_correct"\s*:\s*(true|false)/);
    if (correctMatch) {
      answer.is_correct = correctMatch[1] === 'true';
    }
    answer.explanation = extractFieldValue(partialAnswer, 'explanation');
    if (answer.text) {
      answers.push(answer);
    }
  }

  return answers;
}

export default function StreamingPreview({ text, isComplete }: StreamingPreviewProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom as content streams in
  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [text]);

  // Parse questions - memoized for efficiency
  const parsedQuestions = useMemo(() => {
    if (!text.trim()) return [];

    const questions: PartialQuestion[] = [];

    // Find the start of the JSON array
    const arrayStart = text.indexOf('[');
    if (arrayStart === -1) return [];

    const content = text.substring(arrayStart + 1);

    // Find question object boundaries
    let depth = 0;
    let questionStart = -1;
    const questionStrings: string[] = [];
    let hasIncomplete = false;

    for (let i = 0; i < content.length; i++) {
      const char = content[i];
      // Handle string contents - skip to end of string
      if (char === '"') {
        let j = i + 1;
        while (j < content.length) {
          if (content[j] === '\\' && j + 1 < content.length) {
            j += 2; // Skip escape sequence
          } else if (content[j] === '"') {
            i = j; // Move past the string
            break;
          } else {
            j++;
          }
        }
        if (j >= content.length) {
          i = j; // Incomplete string
        }
        continue;
      }

      if (char === '{') {
        if (depth === 0) {
          questionStart = i;
        }
        depth++;
      } else if (char === '}') {
        depth--;
        if (depth === 0 && questionStart !== -1) {
          questionStrings.push(content.substring(questionStart, i + 1));
          questionStart = -1;
        }
      }
    }

    // If we have an unclosed question object, include it as partial
    if (depth > 0 && questionStart !== -1) {
      questionStrings.push(content.substring(questionStart));
      hasIncomplete = true;
    }

    // Parse each question string
    for (let qIdx = 0; qIdx < questionStrings.length; qIdx++) {
      const questionText = questionStrings[qIdx];
      const isLast = qIdx === questionStrings.length - 1;
      const question: PartialQuestion = {};

      question.stem = extractFieldValue(questionText, 'stem');
      question.code = extractFieldValue(questionText, 'code');
      question.explanation = extractFieldValue(questionText, 'explanation');
      question.distractors = extractFieldValue(questionText, 'distractors');
      question.answers = extractAnswers(questionText);
      question.isStreaming = isLast && hasIncomplete && !isComplete;

      // Add if we have at least a stem (even partial)
      if (question.stem) {
        questions.push(question);
      }
    }

    return questions;
  }, [text, isComplete]);

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex items-center gap-2 px-4 py-2 border-b bg-slate-50">
        {!isComplete && <Loader2 className="w-4 h-4 animate-spin text-primary" />}
        <span className="text-sm font-medium">
          {isComplete ? "✓ Generation complete" : "Generating..."}
        </span>
        <span className="text-xs text-muted-foreground ml-auto">
          {parsedQuestions.length} question{parsedQuestions.length !== 1 ? 's' : ''}
        </span>
      </div>

      {/* Streaming content */}
      <div
        ref={containerRef}
        className="flex-1 overflow-auto p-4 bg-white"
      >
        {parsedQuestions.length > 0 ? (
          <div className="space-y-6">
            {parsedQuestions.map((q, idx) => (
              <div key={idx} className={`border rounded-lg p-4 ${q.isStreaming ? 'bg-blue-50 border-blue-200' : 'bg-slate-50'}`}>
                <div className="font-semibold text-primary mb-3">
                  Question {idx + 1}
                  {q.isStreaming && <span className="ml-2 text-xs text-blue-600 animate-pulse">● streaming</span>}
                </div>

                {/* Stem */}
                {q.stem && (
                  <div className="text-sm mb-3 whitespace-pre-wrap">{q.stem}</div>
                )}

                {/* Code */}
                {q.code && (
                  <div className="mb-3">
                    <SyntaxHighlighter
                      style={prism}
                      language="java"
                      PreTag="div"
                      customStyle={{ fontSize: "0.85rem", margin: 0, borderRadius: "0.375rem" }}
                    >
                      {q.code}
                    </SyntaxHighlighter>
                  </div>
                )}

                {/* Answers */}
                {q.answers && q.answers.length > 0 && (
                  <div className="space-y-2 mb-4">
                    <div className="text-xs font-semibold text-slate-600 uppercase">Answers</div>
                    {q.answers.map((ans, aidx) => (
                      <div key={aidx} className="text-xs">
                        <div
                          className={`px-2 py-1 rounded ${
                            ans.is_correct
                              ? "bg-green-100 text-green-800 font-medium"
                              : "bg-white text-slate-700"
                          }`}
                        >
                          {String.fromCharCode(65 + aidx)}. {ans.text}
                        </div>
                        {ans.explanation && (
                          <div className="text-slate-600 italic ml-6 mt-1">
                            {ans.explanation}
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                )}

                {/* Explanation */}
                {q.explanation && (
                  <div className="mt-4 pt-3 border-t">
                    <div className="text-xs font-semibold text-slate-600 uppercase mb-1">
                      Explanation
                    </div>
                    <div className="text-xs text-slate-700 whitespace-pre-wrap">
                      {q.explanation}
                    </div>
                  </div>
                )}

                {/* Distractors */}
                {q.distractors && (
                  <div className="mt-3 pt-3 border-t">
                    <div className="text-xs font-semibold text-slate-600 uppercase mb-1">
                      Distractors Analysis
                    </div>
                    <div className="text-xs text-slate-700 whitespace-pre-wrap">
                      {q.distractors}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        ) : (
          <div className="text-sm text-muted-foreground italic">
            {text.length > 0 ? "Parsing response..." : "Waiting for response..."}
          </div>
        )}
      </div>
    </div>
  );
}
