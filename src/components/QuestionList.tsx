import { Question } from "../types";
import QuestionCard from "./QuestionCard";
import StreamingQuestionCard from "./StreamingQuestionCard";

interface QuestionListProps {
  questions: Question[];
  topicMetaById?: Record<string, { label: string; kind: "topic" | "subtopic" }>;
  rawTextByQuestionId?: Record<string, string>;
  regeneratingQuestionId?: string | null;
  regenerationStreamingText?: string;
  regenerationStreamingComplete?: boolean;
  showStreamingCard?: boolean;
  streamingText?: string;
  streamingComplete?: boolean;
  showRawStream?: boolean;
  onToggleRawStream?: () => void;
  onRegenerate: (index: number, instructions?: string) => void;
  onEdit: (index: number) => void;
  onDelete: (index: number) => void;
}

export default function QuestionList({
  questions,
  topicMetaById = {},
  rawTextByQuestionId = {},
  regeneratingQuestionId = null,
  regenerationStreamingText = "",
  regenerationStreamingComplete = true,
  showStreamingCard = false,
  streamingText = "",
  streamingComplete = false,
  showRawStream = true,
  onToggleRawStream,
  onRegenerate,
  onEdit,
  onDelete,
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
            liveRawText={question.id === regeneratingQuestionId ? regenerationStreamingText : undefined}
            isRegenerating={
              question.id === regeneratingQuestionId && !regenerationStreamingComplete
            }
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
    </div>
  );
}
