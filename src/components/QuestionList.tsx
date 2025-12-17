import { Question } from "../types";
import QuestionCard from "./QuestionCard";
import { Plus } from "lucide-react";

interface QuestionListProps {
  questions: Question[];
  onRegenerate: (index: number, instructions?: string) => void;
  onEdit: (index: number) => void;
  onDelete: (index: number) => void;
  onAdd: () => void;
}

export default function QuestionList({
  questions,
  onRegenerate,
  onEdit,
  onDelete,
  onAdd,
}: QuestionListProps) {
  return (
    <div className="space-y-4">
      {questions.map((question, index) => (
        <QuestionCard
          key={question.id}
          question={question}
          index={index}
          onRegenerate={(instructions) => onRegenerate(index, instructions)}
          onEdit={() => onEdit(index)}
          onDelete={() => onDelete(index)}
        />
      ))}

      {/* Add Question Button */}
      <button
        onClick={onAdd}
        className="w-full flex items-center justify-center gap-2 px-4 py-4 border-2 border-dashed rounded-lg text-muted-foreground hover:border-primary hover:text-primary transition-colors"
      >
        <Plus className="w-5 h-5" />
        Add Question
      </button>
    </div>
  );
}
