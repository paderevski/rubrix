import { Question, QuestionReviewView } from "../types";
import QuestionCard from "./QuestionCard";
import StreamingQuestionCard from "./StreamingQuestionCard";

interface QuestionListProps {
  questions: Question[];
  topicMetaById?: Record<string, { label: string; kind: "topic" | "subtopic" }>;
  rawTextByQuestionId?: Record<string, string>;
  regeneratingQuestionId?: string | null;
  isRegeneratingAll?: boolean;
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
  onUpdate?: (index: number, question: Question, label: string) => void;
  reviewingQuestionId?: string | null;
  reviewStreamingText?: string;
  reviewById?: Record<string, QuestionReviewView>;
  isReviewDisabled?: boolean;
  onReview?: (index: number, notes?: string) => void;
  onApplyReview?: (index: number) => void;
  onDismissReview?: (index: number) => void;
}

export default function QuestionList({
  questions,
  topicMetaById = {},
  rawTextByQuestionId = {},
  regeneratingQuestionId = null,
  isRegeneratingAll = false,
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
  onUpdate,
  reviewingQuestionId = null,
  reviewStreamingText = "",
  reviewById = {},
  isReviewDisabled = false,
  onReview,
  onApplyReview,
  onDismissReview,
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
            liveRawText={
              question.id === regeneratingQuestionId
                ? regenerationStreamingText
                : question.id === reviewingQuestionId
                ? reviewStreamingText
                : undefined
            }
            isRegenerating={
              question.id === regeneratingQuestionId && !regenerationStreamingComplete
            }
            isRegenerationDisabled={isRegeneratingAll || reviewingQuestionId !== null}
            onRegenerate={(instructions) => onRegenerate(index, instructions)}
            onEdit={() => onEdit(index)}
            onDelete={() => onDelete(index)}
            onUpdate={onUpdate ? (updated, label) => onUpdate(index, updated, label) : undefined}
            review={reviewById[question.id]}
            onReview={onReview ? (notes) => onReview(index, notes) : undefined}
            isReviewDisabled={isReviewDisabled}
            onApplyReview={() => onApplyReview?.(index)}
            onDismissReview={() => onDismissReview?.(index)}
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
