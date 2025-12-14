import { Loader2, Sparkles, Plus } from "lucide-react";
import { TopicInfo } from "../types";

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
  appendMode: boolean;
  onAppendModeChange: (append: boolean) => void;
  existingCount: number;
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
  appendMode,
  onAppendModeChange,
  existingCount,
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
    <aside className="w-80 border-r bg-card flex flex-col">
      {/* Scrollable Content */}
      <div className="flex-1 overflow-y-auto p-4 space-y-6">
        {/* Topics */}
        <div>
          <h3 className="text-sm font-medium text-foreground mb-3">Topics</h3>
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
                <span className="text-xs text-muted-foreground ml-auto">
                  {topic.example_count} examples
                </span>
              </label>
            ))}
          </div>
        </div>

        {/* Difficulty */}
        <div>
          <label className="text-sm font-medium text-foreground mb-2 block">
            Difficulty
          </label>
          <div className="flex gap-2">
            {["easy", "medium", "hard"].map((level) => (
              <button
                key={level}
                onClick={() => onDifficultyChange(level)}
                className={`flex-1 py-1.5 px-3 text-sm rounded-md capitalize transition-colors ${
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

        {/* Append Mode Toggle */}
        {existingCount > 0 && (
          <div className="p-3 bg-secondary/50 rounded-lg">
            <label className="flex items-start gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={appendMode}
                onChange={(e) => onAppendModeChange(e.target.checked)}
                className="mt-0.5 rounded border-gray-300 text-primary focus:ring-primary"
              />
              <div>
                <span className="text-sm font-medium">Add to existing</span>
                <p className="text-xs text-muted-foreground mt-0.5">
                  {appendMode 
                    ? `Will add ${questionCount} to your ${existingCount} questions`
                    : `Will replace your ${existingCount} questions`
                  }
                </p>
              </div>
            </label>
          </div>
        )}
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
          ) : appendMode && existingCount > 0 ? (
            <>
              <Plus className="w-4 h-4" />
              Add {questionCount} More
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
