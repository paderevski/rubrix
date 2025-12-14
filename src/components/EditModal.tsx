import { useState } from "react";
import { Question, Answer } from "../types";
import { X, Plus, Trash2 } from "lucide-react";

interface EditModalProps {
  question: Question;
  onSave: (question: Question) => void;
  onClose: () => void;
}

export default function EditModal({ question, onSave, onClose }: EditModalProps) {
  const [text, setText] = useState(question.text);
  const [code, setCode] = useState(question.code || "");
  const [answers, setAnswers] = useState<Answer[]>(question.answers);

  const handleAnswerChange = (index: number, newText: string) => {
    setAnswers((prev) =>
      prev.map((a, i) => (i === index ? { ...a, text: newText } : a))
    );
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
    if (answers.length <= 2) return; // Keep at least 2 answers
    
    const wasCorrect = answers[index].is_correct;
    const newAnswers = answers.filter((_, i) => i !== index);
    
    // If we removed the correct answer, make the first one correct
    if (wasCorrect && newAnswers.length > 0) {
      newAnswers[0].is_correct = true;
    }
    
    setAnswers(newAnswers);
  };

  const handleSave = () => {
    // Ensure at least one answer is correct
    const hasCorrect = answers.some((a) => a.is_correct);
    const finalAnswers = hasCorrect
      ? answers
      : answers.map((a, i) => ({ ...a, is_correct: i === 0 }));

    onSave({
      ...question,
      text,
      code: code.trim() || null,
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
          {/* Question Text */}
          <div>
            <label className="text-sm font-medium text-foreground mb-1.5 block">
              Question Text
            </label>
            <textarea
              value={text}
              onChange={(e) => setText(e.target.value)}
              className="w-full h-24 px-3 py-2 border rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-primary"
              placeholder="Enter your question..."
            />
          </div>

          {/* Code Block */}
          <div>
            <label className="text-sm font-medium text-foreground mb-1.5 block">
              Code (optional)
            </label>
            <textarea
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="w-full h-32 px-3 py-2 border rounded-md resize-none font-mono text-sm focus:outline-none focus:ring-2 focus:ring-primary"
              placeholder="public void example() {&#10;    // Java code here&#10;}"
            />
          </div>

          {/* Answers */}
          <div>
            <label className="text-sm font-medium text-foreground mb-1.5 block">
              Answers
            </label>
            <div className="space-y-2">
              {answers.map((answer, index) => (
                <div key={index} className="flex items-center gap-2">
                  <input
                    type="radio"
                    name="correct"
                    checked={answer.is_correct}
                    onChange={() => handleCorrectChange(index)}
                    className="w-4 h-4 text-primary"
                    title="Mark as correct"
                  />
                  <input
                    type="text"
                    value={answer.text}
                    onChange={(e) => handleAnswerChange(index, e.target.value)}
                    className="flex-1 px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                    placeholder={`Answer ${String.fromCharCode(65 + index)}`}
                  />
                  <button
                    onClick={() => handleRemoveAnswer(index)}
                    disabled={answers.length <= 2}
                    className="p-2 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
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
