import { useEffect, useState } from "react";
import {
  ChevronDown,
  ChevronRight,
  Loader2,
  Sparkles,
  Plus,
  PanelLeftClose,
  PanelLeftOpen,
} from "lucide-react";
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
  collapsed?: boolean;
  onToggleCollapsed?: () => void;
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
  collapsed = false,
  onToggleCollapsed,
}: SidebarProps) {
  const [expandedTopics, setExpandedTopics] = useState<Record<string, boolean>>({});

  useEffect(() => {
    setExpandedTopics((prev) => {
      const next: Record<string, boolean> = {};
      for (const topic of topics) {
        const hasSelectedChild =
          topic.children?.some((child) => selectedTopics.includes(child.id)) ?? false;
        if (prev[topic.id] || hasSelectedChild) {
          next[topic.id] = true;
        }
      }
      return next;
    });
  }, [topics, selectedTopics]);

  const toggleTopic = (topicId: string) => {
    if (selectedTopics.includes(topicId)) {
      onTopicsChange(selectedTopics.filter((t) => t !== topicId));
    } else {
      onTopicsChange([...selectedTopics, topicId]);
    }
  };

  const toggleExpanded = (topicId: string) => {
    setExpandedTopics((prev) => ({
      ...prev,
      [topicId]: !prev[topicId],
    }));
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
        {/* Topics */}
        <div className="flex-1 min-h-0 flex flex-col">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-sm font-medium text-foreground">Topics</h3>
            <button
              onClick={onToggleCollapsed}
              className="p-1.5 rounded-md border hover:bg-secondary"
              title="Collapse sidebar"
            >
              <PanelLeftClose className="w-4 h-4" />
            </button>
          </div>
          <div className="relative flex-1 min-h-0">
            <div className="absolute top-0 left-0 right-0 h-4 bg-gradient-to-b from-card to-transparent pointer-events-none z-10" />
            <div className="absolute bottom-0 left-0 right-0 h-4 bg-gradient-to-t from-card to-transparent pointer-events-none z-10" />
            <div className="h-full overflow-y-auto overscroll-contain pr-1 space-y-2">
              {topics.map((topic) => (
                <div key={topic.id} className="space-y-1">
                  <div className="flex items-center gap-2 rounded-md px-1 py-1 hover:bg-secondary/60">
                    {topic.children && topic.children.length > 0 ? (
                      <button
                        type="button"
                        onClick={() => toggleExpanded(topic.id)}
                        className="p-0.5 rounded hover:bg-secondary"
                        title={expandedTopics[topic.id] ? "Collapse" : "Expand"}
                      >
                        {expandedTopics[topic.id] ? (
                          <ChevronDown className="w-4 h-4 text-muted-foreground" />
                        ) : (
                          <ChevronRight className="w-4 h-4 text-muted-foreground" />
                        )}
                      </button>
                    ) : (
                      <span className="w-5" />
                    )}

                    <label className="flex items-center gap-2 cursor-pointer flex-1 min-w-0">
                      <input
                        type="checkbox"
                        checked={selectedTopics.includes(topic.id)}
                        onChange={() => toggleTopic(topic.id)}
                        className="rounded border-gray-300 text-primary focus:ring-primary"
                      />
                      <span
                        className="text-sm leading-5 line-clamp-2 break-words"
                        title={topic.name}
                      >
                        {topic.name}
                      </span>
                      <span className="text-xs text-muted-foreground ml-auto whitespace-nowrap">
                        {topic.example_count}
                      </span>
                    </label>
                  </div>

                  {topic.children && topic.children.length > 0 && expandedTopics[topic.id] && (
                    <div className="ml-6 pl-3 border-l border-slate-200 space-y-1">
                      {topic.children.map((child) => (
                        <label
                          key={child.id}
                          className="flex items-center gap-2 cursor-pointer rounded-md px-1.5 py-1 hover:bg-secondary/50"
                        >
                          <input
                            type="checkbox"
                            checked={selectedTopics.includes(child.id)}
                            onChange={() => toggleTopic(child.id)}
                            className="rounded border-gray-300 text-primary focus:ring-primary"
                          />
                          <span
                            className="text-sm leading-5 line-clamp-2 break-words"
                            title={child.name}
                          >
                            {child.name}
                          </span>
                          <span className="text-xs text-muted-foreground ml-auto whitespace-nowrap">
                            {child.example_count}
                          </span>
                        </label>
                      ))}
                    </div>
                  )}
                </div>
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
