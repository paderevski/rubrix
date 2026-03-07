import { Loader2, Sparkles, Plus, PanelLeftClose, PanelLeftOpen } from "lucide-react";
import { TopicInfo, SubjectInfo } from "../types";

interface SidebarProps {
  subjects: SubjectInfo[];
  selectedSubject: string;
  onSubjectChange: (subject: string) => void;
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
  collapsed?: boolean;
  onToggleCollapsed?: () => void;
}

export default function Sidebar({
  subjects,
  selectedSubject,
  onSubjectChange,
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
  collapsed = false,
  onToggleCollapsed,
}: SidebarProps) {
  const toggleTopic = (topicId: string) => {
    if (selectedTopics.includes(topicId)) {
      onTopicsChange(selectedTopics.filter((t) => t !== topicId));
    } else {
      onTopicsChange([...selectedTopics, topicId]);
    }
  };

  if (collapsed) {
    return (
      <aside className="w-14 h-full border-r bg-card flex flex-col items-center gap-3 py-3">
        <button
          onClick={onToggleCollapsed}
          className="p-2 rounded-md border hover:bg-secondary"
          title="Expand filters"
        >
          <PanelLeftOpen className="w-4 h-4" />
        </button>
        <button
          onClick={onGenerate}
          disabled={isGenerating || selectedTopics.length === 0}
          className="p-2 rounded-md border bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
          title={isGenerating ? "Generating..." : "Generate questions"}
        >
          {isGenerating ? <Loader2 className="w-4 h-4 animate-spin" /> : <Sparkles className="w-4 h-4" />}
        </button>
      </aside>
    );
  }

  return (
    <aside className="w-80 h-full border-r bg-card flex flex-col overflow-hidden">
      {/* Fixed-height Content */}
      <div className="flex-1 min-h-0 p-4 flex flex-col gap-4 overflow-hidden">
        {/* Subject Selection */}
        <div className="shrink-0">
          <div className="flex items-center justify-between mb-2">
            <label className="text-sm font-medium text-foreground">Subject</label>
            <button
              onClick={onToggleCollapsed}
              className="p-1.5 rounded-md border hover:bg-secondary"
              title="Collapse sidebar"
            >
              <PanelLeftClose className="w-4 h-4" />
            </button>
          </div>
          <select
            value={selectedSubject}
            onChange={(e) => onSubjectChange(e.target.value)}
            className="w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-primary bg-background"
          >
            {subjects.map((subject) => (
              <option key={subject.id} value={subject.id}>
                {subject.name} ({subject.topic_count} topics)
              </option>
            ))}
          </select>
        </div>

        {/* Topics */}
        <div className="flex-1 min-h-0 flex flex-col">
          <h3 className="text-sm font-medium text-foreground mb-3">Topics</h3>
          <div className="relative flex-1 min-h-0">
            <div className="absolute top-0 left-0 right-0 h-4 bg-gradient-to-b from-card to-transparent pointer-events-none z-10" />
            <div className="absolute bottom-0 left-0 right-0 h-4 bg-gradient-to-t from-card to-transparent pointer-events-none z-10" />
            <div className="h-full overflow-y-auto overscroll-contain pr-1 space-y-2">
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
        </div>

        {/* Difficulty */}
        <div className="shrink-0">
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
        <div className="shrink-0">
          <label className="text-sm font-medium text-foreground mb-2 block">
            Questions: {questionCount}
          </label>
          <input
            type="range"
            min="1"
            max="8"
            value={questionCount}
            onChange={(e) => onQuestionCountChange(parseInt(e.target.value))}
            className="w-full accent-primary"
          />
          <div className="flex justify-between text-xs text-muted-foreground">
            <span>1</span>
            <span>8</span>
          </div>
        </div>

        {/* Notes */}
        <div className="shrink-0">
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
          <div className="shrink-0 p-3 bg-secondary/50 rounded-lg">
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
      <div className="p-4 border-t shrink-0">
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
