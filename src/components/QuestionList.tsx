import { Question } from "../types";
import QuestionCard from "./QuestionCard";
import { Plus } from "lucide-react";
import StreamingQuestionCard from "./StreamingQuestionCard";

interface QuestionListProps {
  questions: Question[];
  topicMetaById?: Record<string, { label: string; kind: "topic" | "subtopic" }>;
  rawTextByQuestionId?: Record<string, string>;
  showStreamingCard?: boolean;
  streamingText?: string;
  streamingComplete?: boolean;
  showRawStream?: boolean;
  onToggleRawStream?: () => void;
  onRegenerate: (index: number, instructions?: string) => void;
  onEdit: (index: number) => void;
  onDelete: (index: number) => void;
  onAdd: () => void;
}

export default function QuestionList({
  questions,
  topicMetaById = {},
  rawTextByQuestionId = {},
  showStreamingCard = false,
  streamingText = "",
  streamingComplete = false,
  showRawStream = true,
  onToggleRawStream,
  onRegenerate,
  onEdit,
  onDelete,
  onAdd,
}: QuestionListProps) {
  const centeredContainerClass = "w-full max-w-5xl mx-auto";

  return (
    <div className="space-y-4">
      {questions.map((question, index) => (
        <div key={question.id} className={centeredContainerClass}>
          <QuestionCard
            question={question}
            index={index}
            topicMetaById={topicMetaById}
            rawText={rawTextByQuestionId[question.id]}
            onRegenerate={(instructions) => onRegenerate(index, instructions)}
            onEdit={() => onEdit(index)}
            onDelete={() => onDelete(index)}
          />
        </div>
      ))}

      {showStreamingCard && (
        <div className={centeredContainerClass}>
          <StreamingQuestionCard
            text={streamingText}
            isComplete={streamingComplete}
            showRaw={showRawStream}
            onToggleRaw={() => onToggleRawStream?.()}
          />
        </div>
      )}

      {/* Add Question Button */}
      {questions.length > 0 && (
        <div className={centeredContainerClass}>
          <button
            onClick={onAdd}
            className="w-full flex items-center justify-center gap-2 px-4 py-4 border-2 border-dashed rounded-lg text-muted-foreground hover:border-primary hover:text-primary transition-colors"
          >
            <Plus className="w-5 h-5" />
            Add Question
          </button>
        </div>
      )}
    </div>
  );
}
