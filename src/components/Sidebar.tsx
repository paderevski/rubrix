import { TopicInfo } from "../types";
import { Loader2, Sparkles } from "lucide-react";

interface SidebarProps {
  topics: TopicInfo[];
  selectedTopics: string[];
  onTopicsChange: (topics: string[]) => void;
  difficulty: string;
  onDifficultyChange: (difficulty: string) => void;
  questionCount: number;
  onQuestionCountChange: (count: number) => void;
  notes: string;
  onNotesChange: (notes: string) => void;
  onGenerate: () => void;
  isGenerating: boolean;
}

export default function Sidebar({
  topics,
  selectedTopics,
  onTopicsChange,
  difficulty,
  onDifficultyChange,
  questionCount,
  onQuestionCountChange,
  notes,
  onNotesChange,
  onGenerate,
  isGenerating,
}: SidebarProps) {
  const toggleTopic = (topicId: string) => {
    if (selectedTopics.includes(topicId)) {
      onTopicsChange(selectedTopics.filter((t) => t !== topicId));
    } else {
      onTopicsChange([...selectedTopics, topicId]);
    }
  };

  return (
    <aside className="w-72 bg-white border-r flex flex-col">
      <div className="p-4 border-b">
        <h2 className="font-semibold text-foreground">Settings</h2>
      </div>

      <div className="flex-1 overflow-auto p-4 space-y-6">
        {/* Topics */}
        <div>
          <label className="text-sm font-medium text-foreground mb-2 block">
            Topics
          </label>
          <div className="space-y-2">
            {topics.map((topic) => (
              <label
                key={topic.id}
                className="flex items-center gap-2 cursor-pointer"
              >
                <input
                  type="checkbox"
                  checked={selectedTopics.includes(topic.id)}
                  onChange={() => toggleTopic(topic.id)}
                  className="rounded border-gray-300 text-primary focus:ring-primary"
                />
                <span className="text-sm">{topic.name}</span>
                {topic.example_count > 0 && (
                  <span className="text-xs text-muted-foreground">
                    ({topic.example_count})
                  </span>
                )}
              </label>
            ))}
          </div>
        </div>

        {/* Difficulty */}
        <div>
          <label className="text-sm font-medium text-foreground mb-2 block">
            Difficulty
          </label>
          <div className="flex gap-1">
            {["easy", "medium", "hard"].map((level) => (
              <button
                key={level}
                onClick={() => onDifficultyChange(level)}
                className={`flex-1 px-3 py-1.5 text-sm rounded-md capitalize transition-colors ${
                  difficulty === level
                    ? "bg-primary text-primary-foreground"
                    : "bg-secondary hover:bg-secondary/80"
                }`}
              >
                {level}
              </button>
            ))}
          </div>
        </div>

        {/* Question Count */}
        <div>
          <label className="text-sm font-medium text-foreground mb-2 block">
            Questions: {questionCount}
          </label>
          <input
            type="range"
            min="1"
            max="20"
            value={questionCount}
            onChange={(e) => onQuestionCountChange(parseInt(e.target.value))}
            className="w-full accent-primary"
          />
          <div className="flex justify-between text-xs text-muted-foreground">
            <span>1</span>
            <span>20</span>
          </div>
        </div>

        {/* Notes */}
        <div>
          <label className="text-sm font-medium text-foreground mb-2 block">
            Additional Notes
          </label>
          <textarea
            value={notes}
            onChange={(e) => onNotesChange(e.target.value)}
            placeholder="Focus on base cases, avoid trick questions..."
            className="w-full h-24 px-3 py-2 text-sm border rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-primary"
          />
        </div>
      </div>

      {/* Generate Button */}
      <div className="p-4 border-t">
        <button
          onClick={onGenerate}
          disabled={isGenerating || selectedTopics.length === 0}
          className="w-full flex items-center justify-center gap-2 px-4 py-2.5 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {isGenerating ? (
            <>
              <Loader2 className="w-4 h-4 animate-spin" />
              Generating...
            </>
          ) : (
            <>
              <Sparkles className="w-4 h-4" />
              Generate Questions
            </>
          )}
        </button>
      </div>
    </aside>
  );
}
