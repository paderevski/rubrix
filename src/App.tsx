import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { save } from "@tauri-apps/api/dialog";
import { writeBinaryFile, writeTextFile } from "@tauri-apps/api/fs";
import Sidebar from "./components/Sidebar";
import QuestionList from "./components/QuestionList";
import EditModal from "./components/EditModal";
import { Question, TopicInfo, GenerationRequest } from "./types";
import { FileDown, FileText, Loader2 } from "lucide-react";

function App() {
  // State
  const [topics, setTopics] = useState<TopicInfo[]>([]);
  const [selectedTopics, setSelectedTopics] = useState<string[]>([]);
  const [difficulty, setDifficulty] = useState("medium");
  const [questionCount, setQuestionCount] = useState(5);
  const [notes, setNotes] = useState("");
  const [questions, setQuestions] = useState<Question[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  const [editingIndex, setEditingIndex] = useState<number | null>(null);
  const [status, setStatus] = useState("Ready");

  // Load topics on mount
  useEffect(() => {
    loadTopics();
  }, []);

  const loadTopics = async () => {
    try {
      const topicList = await invoke<TopicInfo[]>("get_topics");
      setTopics(topicList);
    } catch (err) {
      console.error("Failed to load topics:", err);
    }
  };

  const handleGenerate = async () => {
    if (selectedTopics.length === 0) {
      setStatus("Please select at least one topic");
      return;
    }

    setIsGenerating(true);
    setStatus("Generating questions...");

    try {
      const request: GenerationRequest = {
        topics: selectedTopics,
        difficulty,
        count: questionCount,
        notes: notes || null,
      };

      const generated = await invoke<Question[]>("generate_questions", {
        request,
      });
      setQuestions(generated);
      setStatus(`Generated ${generated.length} questions`);
    } catch (err) {
      console.error("Generation failed:", err);
      setStatus(`Error: ${err}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleRegenerate = async (index: number) => {
    setStatus(`Regenerating question ${index + 1}...`);

    try {
      const newQuestion = await invoke<Question>("regenerate_question", {
        index,
      });
      setQuestions((prev) => {
        const updated = [...prev];
        updated[index] = newQuestion;
        return updated;
      });
      setStatus("Question regenerated");
    } catch (err) {
      console.error("Regeneration failed:", err);
      setStatus(`Error: ${err}`);
    }
  };

  const handleEdit = (index: number) => {
    setEditingIndex(index);
  };

  const handleSaveEdit = async (question: Question) => {
    if (editingIndex === null) return;

    try {
      await invoke("update_question", { index: editingIndex, question });
      setQuestions((prev) => {
        const updated = [...prev];
        updated[editingIndex] = question;
        return updated;
      });
      setEditingIndex(null);
      setStatus("Question updated");
    } catch (err) {
      console.error("Update failed:", err);
      setStatus(`Error: ${err}`);
    }
  };

  const handleDelete = async (index: number) => {
    try {
      await invoke("delete_question", { index });
      setQuestions((prev) => prev.filter((_, i) => i !== index));
      setStatus("Question deleted");
    } catch (err) {
      console.error("Delete failed:", err);
      setStatus(`Error: ${err}`);
    }
  };

  const handleAddQuestion = async () => {
    try {
      const newQuestion = await invoke<Question>("add_question");
      setQuestions((prev) => [...prev, newQuestion]);
      setEditingIndex(questions.length); // Open edit modal for new question
    } catch (err) {
      console.error("Add failed:", err);
    }
  };

  const handleExportTxt = async () => {
    const filePath = await save({
      defaultPath: "questions.txt",
      filters: [{ name: "Text", extensions: ["txt"] }],
    });

    if (!filePath) return;

    try {
      const content = await invoke<string>("export_to_txt", {
        title: "AP CS Questions",
      });
      await writeTextFile(filePath, content);
      setStatus(`Exported to ${filePath}`);
    } catch (err) {
      console.error("Export failed:", err);
      setStatus(`Export error: ${err}`);
    }
  };

  const handleExportQti = async () => {
    const filePath = await save({
      defaultPath: "questions.imscc",
      filters: [{ name: "IMS Common Cartridge", extensions: ["imscc", "zip"] }],
    });

    if (!filePath) return;

    try {
      const data = await invoke<number[]>("export_to_qti", {
        title: "AP CS Questions",
      });
      await writeBinaryFile(filePath, new Uint8Array(data));
      setStatus(`Exported to ${filePath}`);
    } catch (err) {
      console.error("Export failed:", err);
      setStatus(`Export error: ${err}`);
    }
  };

  return (
    <div className="flex h-screen bg-background">
      {/* Sidebar */}
      <Sidebar
        topics={topics}
        selectedTopics={selectedTopics}
        onTopicsChange={setSelectedTopics}
        difficulty={difficulty}
        onDifficultyChange={setDifficulty}
        questionCount={questionCount}
        onQuestionCountChange={setQuestionCount}
        notes={notes}
        onNotesChange={setNotes}
        onGenerate={handleGenerate}
        isGenerating={isGenerating}
      />

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Header */}
        <header className="flex items-center justify-between px-6 py-4 border-b bg-white">
          <h1 className="text-xl font-semibold text-foreground">
            📝 Rubrix
            <span className="text-sm font-normal text-muted-foreground ml-2">
              AP CS Test Generator
            </span>
          </h1>

          {/* Export Buttons */}
          <div className="flex gap-2">
            <button
              onClick={handleExportTxt}
              disabled={questions.length === 0}
              className="flex items-center gap-2 px-3 py-2 text-sm border rounded-md hover:bg-secondary disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <FileText className="w-4 h-4" />
              Export .txt
            </button>
            <button
              onClick={handleExportQti}
              disabled={questions.length === 0}
              className="flex items-center gap-2 px-3 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <FileDown className="w-4 h-4" />
              Export QTI
            </button>
          </div>
        </header>

        {/* Questions List */}
        <main className="flex-1 overflow-auto p-6">
          {questions.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
              <div className="text-6xl mb-4">📚</div>
              <p className="text-lg">No questions yet</p>
              <p className="text-sm">
                Select topics and click "Generate Questions" to get started
              </p>
            </div>
          ) : (
            <QuestionList
              questions={questions}
              onRegenerate={handleRegenerate}
              onEdit={handleEdit}
              onDelete={handleDelete}
              onAdd={handleAddQuestion}
            />
          )}
        </main>

        {/* Status Bar */}
        <footer className="flex items-center justify-between px-6 py-2 border-t bg-white text-sm text-muted-foreground">
          <span className="flex items-center gap-2">
            {isGenerating && <Loader2 className="w-4 h-4 animate-spin" />}
            {status}
          </span>
          <span>{questions.length} questions</span>
        </footer>
      </div>

      {/* Edit Modal */}
      {editingIndex !== null && (
        <EditModal
          question={questions[editingIndex]}
          onSave={handleSaveEdit}
          onClose={() => setEditingIndex(null)}
        />
      )}
    </div>
  );
}

export default App;
