import { useLayoutEffect, useRef, useState, type FormEvent } from "react";
import { Question, Answer } from "../types";
import { X, Plus, Trash2 } from "lucide-react";

interface EditModalProps {
  question: Question;
  onSave: (question: Question) => void;
  onClose: () => void;
}

export default function EditModal({ question, onSave, onClose }: EditModalProps) {
  const MAX_ANSWER_HEIGHT_PX = 192;
  const [content, setContent] = useState(question.text);
  const [answers, setAnswers] = useState<Answer[]>(
    question.answers.map((answer) => ({
      ...answer,
      // Normalize escaped newlines so they edit as real line breaks.
      text: answer.text.replace(/\\n/g, "\n"),
    }))
  );
  const answerRefs = useRef<Array<HTMLTextAreaElement | null>>([]);

  const resizeAnswerTextarea = (el: HTMLTextAreaElement | null) => {
    if (!el) return;
    el.style.height = "auto";
    const nextHeight = Math.min(el.scrollHeight, MAX_ANSWER_HEIGHT_PX);
    el.style.height = `${nextHeight}px`;
    el.style.overflowY = el.scrollHeight > MAX_ANSWER_HEIGHT_PX ? "auto" : "hidden";
  };

  useLayoutEffect(() => {
    answerRefs.current.forEach((el) => resizeAnswerTextarea(el));
  }, [answers]);

  const handleAnswerChange = (index: number, newText: string) => {
    setAnswers((prev) =>
      prev.map((a, i) => (i === index ? { ...a, text: newText } : a))
    );
  };

  const handleAnswerInput = (e: FormEvent<HTMLTextAreaElement>) => {
    resizeAnswerTextarea(e.currentTarget);
  };

  const handleCorrectChange = (index: number) => {
    setAnswers((prev) =>
      prev.map((a, i) => ({ ...a, is_correct: i === index }))
    );
  };

  const handleAddAnswer = () => {
    setAnswers((prev) => [...prev, { text: "", is_correct: false }]);
  };

  const handleRemoveAnswer = (index: number) => {
    if (answers.length <= 2) return;

    const wasCorrect = answers[index].is_correct;
    const newAnswers = answers.filter((_, i) => i !== index);

    if (wasCorrect && newAnswers.length > 0) {
      newAnswers[0].is_correct = true;
    }

    setAnswers(newAnswers);
  };

  const handleSave = () => {
    const hasCorrect = answers.some((a) => a.is_correct);
    const finalAnswers = hasCorrect
      ? answers
      : answers.map((a, i) => ({ ...a, is_correct: i === 0 }));

    onSave({
      ...question,
      text: content,
      answers: finalAnswers,
    });
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b">
          <h2 className="text-lg font-semibold">Edit Question</h2>
          <button
            onClick={onClose}
            className="p-1 rounded hover:bg-secondary transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-auto p-6 space-y-4">
          {/* Question Content (Markdown) */}
          <div>
            <label className="text-sm font-medium text-foreground mb-1.5 block">
              Question Content (Markdown supported)
            </label>
            <textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              className="w-full h-48 px-3 py-2 border rounded-md resize-none font-mono text-sm focus:outline-none focus:ring-2 focus:ring-primary"
              placeholder={`What is printed by this code?

\`\`\`java
System.out.println("Hello");
\`\`\`

You can also use tables, \`inline code\`, **bold**, etc.`}
            />
            <p className="text-xs text-muted-foreground mt-1">
              Supports Markdown: code blocks, tables, inline `code`, **bold**, *italic*
            </p>
          </div>

          {/* Answers */}
          <div>
            <label className="text-sm font-medium text-foreground mb-1.5 block">
              Answers
            </label>
            <div className="space-y-2">
              {answers.map((answer, index) => (
                <div key={index} className="flex items-start gap-2">
                  <input
                    type="radio"
                    name="correct"
                    checked={answer.is_correct}
                    onChange={() => handleCorrectChange(index)}
                    className="w-4 h-4 text-primary mt-2"
                    title="Mark as correct"
                  />
                  <textarea
                    ref={(el) => {
                      answerRefs.current[index] = el;
                    }}
                    value={answer.text}
                    onChange={(e) => handleAnswerChange(index, e.target.value)}
                    onInput={handleAnswerInput}
                    rows={3}
                    className="flex-1 min-h-[76px] max-h-48 px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-y"
                    placeholder={`Answer ${String.fromCharCode(65 + index)}`}
                  />
                  <button
                    onClick={() => handleRemoveAnswer(index)}
                    disabled={answers.length <= 2}
                    className="p-2 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive disabled:opacity-30 disabled:cursor-not-allowed transition-colors mt-1"
                    title="Remove answer"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              ))}
            </div>
            <button
              onClick={handleAddAnswer}
              className="mt-2 flex items-center gap-1 text-sm text-primary hover:underline"
            >
              <Plus className="w-4 h-4" />
              Add Answer
            </button>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-2 px-6 py-4 border-t">
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm border rounded-md hover:bg-secondary transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
          >
            Save Changes
          </button>
        </div>
      </div>
    </div>
  );
}
