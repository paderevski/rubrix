import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/api/dialog";
import { writeBinaryFile, writeTextFile, readTextFile } from "@tauri-apps/api/fs";
import Sidebar from "./components/Sidebar";
import QuestionList from "./components/QuestionList";
import EditModal from "./components/EditModal";
import StreamingPreview from "./components/StreamingPreview";
import BankEditor from "./components/BankEditor";
import LoginModal from "./components/LoginModal";
import { Question, TopicInfo, SubjectInfo, GenerationRequest, Answer } from "./types";
import {
  FileDown,
  FileText,
  Loader2,
  Eye,
  EyeOff,
  Save as SaveIcon,
  FolderOpen,
} from "lucide-react";
import AlertModal from "./components/AlertModal";

// Event payload from Rust backend
interface StreamEvent {
  text: string;
  done: boolean;
}

const sessionFileFilter = { name: "Rubrix Session", extensions: ["json"] };

function parseSessionQuestions(raw: unknown): Question[] {
  const payload: unknown[] | null = Array.isArray(raw)
    ? raw
    : raw && typeof raw === "object" && Array.isArray((raw as any).questions)
    ? (raw as any).questions
    : null;

  if (!payload) {
    throw new Error("Invalid session file format");
  }

  const warn = (message: string, meta?: unknown) => {
    if (meta !== undefined) {
      console.warn(`[Rubrix Session] ${message}`, meta);
    } else {
      console.warn(`[Rubrix Session] ${message}`);
    }
  };

  const stringifyValue = (value: unknown): string | undefined => {
    if (value == null) return undefined;
    if (typeof value === "string") return value;
    try {
      return JSON.stringify(value);
    } catch {
      return String(value);
    }
  };

  const coerceRichText = (value: unknown): string | undefined => {
    const str = stringifyValue(value)?.trim();
    return str && str.length > 0 ? str : undefined;
  };

  const coerceAnswers = (entry: Record<string, unknown>, questionIndex: number): Answer[] => {
    const rawAnswers = Array.isArray((entry as any).answers)
      ? ((entry as any).answers as unknown[])
      : Array.isArray((entry as any).options)
      ? ((entry as any).options as unknown[])
      : [];

    if (!Array.isArray((entry as any).answers) && Array.isArray((entry as any).options)) {
      warn(`Question ${questionIndex + 1} used legacy "options" field; converting to "answers".`);
    }

    if (rawAnswers.length === 0) {
      warn(`Question ${questionIndex + 1} is missing answers.`);
      return [];
    }

    const answers: Answer[] = [];

    rawAnswers.forEach((answerRaw, answerIndex) => {
      if (!answerRaw || typeof answerRaw !== "object") {
        warn(`Question ${questionIndex + 1} answer ${answerIndex + 1} is not an object; skipping.`);
        return;
      }

      const answer = answerRaw as Record<string, unknown>;
      const text =
        typeof answer.text === "string" && answer.text.trim().length > 0
          ? answer.text
          : `Answer ${String.fromCharCode(65 + answerIndex)}`;

      const isCorrect =
        typeof answer.is_correct === "boolean"
          ? answer.is_correct
          : typeof (answer as any).correct === "boolean"
          ? Boolean((answer as any).correct)
          : false;

      answers.push({
        text,
        is_correct: isCorrect,
        explanation: coerceRichText(answer.explanation),
      });
    });

    return answers;
  };

  const normalized = payload
    .map((entry: unknown, questionIndex: number) => {
      if (!entry || typeof entry !== "object") {
        warn(`Question ${questionIndex + 1} is not an object; skipping.`);
        return null;
      }

      const q = entry as Record<string, unknown>;
      const text =
        typeof q.text === "string" && q.text.trim().length > 0
          ? q.text
          : stringifyValue(q.text) ?? "Untitled question";

      const answers = coerceAnswers(q, questionIndex);
      if (answers.length === 0) {
        warn(`Question ${questionIndex + 1} has no valid answers; it may fail to render.`);
      }

      const topics = Array.isArray(q.topics)
        ? (q.topics as unknown[]).filter(
            (topic): topic is string => typeof topic === "string" && topic.trim().length > 0
          )
        : undefined;

      const normalizedQuestion: Question = {
        id:
          typeof q.id === "string" && q.id.trim().length > 0
            ? q.id
            : `q${questionIndex + 1}`,
        text,
        answers,
        explanation: coerceRichText(q.explanation),
        distractors: coerceRichText(q.distractors),
        subject: typeof q.subject === "string" ? q.subject : undefined,
        topics,
      };

      return normalizedQuestion;
    })
    .filter((question: Question | null): question is Question => Boolean(question));

  if (normalized.length === 0) {
    throw new Error("No valid questions found in session file");
  }

  return normalized;
}

function App() {
  // State
  const [subjects, setSubjects] = useState<SubjectInfo[]>([]);
  const [selectedSubject, setSelectedSubject] = useState("");
  const [topics, setTopics] = useState<TopicInfo[]>([]);
  const [selectedTopics, setSelectedTopics] = useState<string[]>([]);
  const [difficulty, setDifficulty] = useState("medium");
  const [questionCount, setQuestionCount] = useState(1);
  const [notes, setNotes] = useState("");
  const [questions, setQuestions] = useState<Question[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  const [editingIndex, setEditingIndex] = useState<number | null>(null);
  const [status, setStatus] = useState("Ready");
  const [appendMode, setAppendMode] = useState(false);

  // Zoom state (driven by native menu events)
  const [zoom, setZoom] = useState<number>(() => {
    const saved = typeof localStorage !== "undefined" ? localStorage.getItem("appZoom") : null;
    const parsed = saved ? parseFloat(saved) : 1;
    if (Number.isFinite(parsed)) {
      return Math.min(Math.max(parsed, 0.85), 1.3);
    }
    return 1;
  });

  // Streaming state
  const [streamingText, setStreamingText] = useState("");
  const [streamingComplete, setStreamingComplete] = useState(false);
  const [showPreview, setShowPreview] = useState(true);
  const [activeTab, setActiveTab] = useState<"generate" | "bank">("generate");
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  // Alert modal state
  const [alertOpen, setAlertOpen] = useState(false);
  const [alertMessage, setAlertMessage] = useState("");

  // Authentication state
  const [loginModalOpen, setLoginModalOpen] = useState(false);
  const [authError, setAuthError] = useState("");
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  // Load subjects and any previously generated questions on mount
  useEffect(() => {
    checkAuthentication();
    loadSubjects();
    loadExistingQuestions();
  }, []);

  const checkAuthentication = async () => {
    try {
      const isAuth = await invoke<boolean>("check_auth");
      setIsAuthenticated(isAuth);
    } catch (err) {
      console.error("Failed to check auth:", err);
    }
  };

  const handleLogin = async (username: string, password: string) => {
    setAuthError("");
    try {
      await invoke("authenticate", { username, password });
      setIsAuthenticated(true);
      setLoginModalOpen(false);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      setAuthError(errorMsg);
      throw new Error(errorMsg);
    }
  };

  const handleLogout = async () => {
    try {
      await invoke("clear_auth");
      setIsAuthenticated(false);
      setStatus("Logged out");
    } catch (err) {
      console.error("Failed to clear auth:", err);
    }
  };

  // Apply zoom on mount and when it changes
  useEffect(() => {
    document.documentElement.style.setProperty("--app-zoom", zoom.toString());
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("appZoom", zoom.toString());
    }
  }, [zoom]);

  // Load topics when subject changes
  useEffect(() => {
    if (selectedSubject) {
      loadTopics(selectedSubject);
    }
  }, [selectedSubject]);

  // Listen for streaming events from backend
  useEffect(() => {
    const unlisten = listen<StreamEvent>("llm-stream", (event) => {
      setStreamingText(event.payload.text);
      setStreamingComplete(event.payload.done);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  // Listen for zoom events from the native menu
  useEffect(() => {
    const clamp = (v: number) => Math.min(Math.max(v, 0.85), 1.3);
    const step = 0.05;

    const handler = (evt: { payload: string }) => {
      if (evt.payload === "in") {
        setZoom((z) => clamp(z + step));
      } else if (evt.payload === "out") {
        setZoom((z) => clamp(z - step));
      } else if (evt.payload === "reset") {
        setZoom(1);
      }
    };

    const unlistenZoom = listen("app-zoom", handler);

    return () => {
      unlistenZoom.then((f) => f());
    };
  }, []);

  const loadSubjects = async () => {
    try {
      const subjectList = await invoke<SubjectInfo[]>("get_subjects");
      setSubjects(subjectList);
      if (subjectList.length > 0) {
        setSelectedSubject(subjectList[0].id);
      }
    } catch (err) {
      console.error("Failed to load subjects:", err);
    }
  };

  const loadTopics = async (subject: string) => {
    try {
      const topicList = await invoke<TopicInfo[]>("get_topics", { subject });
      setTopics(topicList);
      setSelectedTopics([]); // Clear selected topics when subject changes
    } catch (err) {
      console.error("Failed to load topics:", err);
    }
  };

  const loadExistingQuestions = async () => {
    try {
      const stored = await invoke<Question[]>("get_questions");
      if (stored.length > 0) {
        setQuestions(stored);
        setStatus(`Restored ${stored.length} question${stored.length === 1 ? "" : "s"}`);
      }
    } catch (err) {
      console.error("Failed to restore questions:", err);
    }
  };

  const handleGenerate = async () => {
    if (selectedTopics.length === 0) {
      setStatus("Please select at least one topic");
      return;
    }

    // Check if authenticated, if not show login modal
    if (!isAuthenticated) {
      setLoginModalOpen(true);
      setStatus("Authentication required");
      return;
    }

    setIsGenerating(true);
    setStreamingText("");
    setStreamingComplete(false);
    setShowPreview(true);
    setStatus(appendMode ? "Adding more questions..." : "Generating questions...");

    try {
      const request: GenerationRequest = {
        subject: selectedSubject,
        topics: selectedTopics,
        difficulty,
        count: questionCount,
        notes: notes || null,
        append: appendMode,
      };

      const allQuestions = await invoke<Question[]>("generate_questions", {
        request,
      });
      setQuestions(allQuestions);

      if (appendMode) {
        setStatus(`Added ${questionCount} questions (${allQuestions.length} total)`);
      } else {
        setStatus(`Generated ${allQuestions.length} questions`);
      }
    } catch (err) {
      console.error("Generation failed:", err);
      const errorMsg = String(err);

      // If error suggests missing auth, show login modal
      if (errorMsg.includes("LAMBDA_URL") || errorMsg.includes("auth")) {
        setLoginModalOpen(true);
        setStatus("Authentication required");
      } else {
        setStatus(`Error: ${err}`);
      }
    } finally {
      setIsGenerating(false);
    }
  };

  const handleRegenerate = async (index: number, instructions?: string) => {
    setStatus(`Regenerating question ${index + 1}...`);
    setStreamingText("");
    setStreamingComplete(false);
    setShowPreview(true);

    try {
      const newQuestion = await invoke<Question>("regenerate_question", {
        index,
        instructions: instructions || null,
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
      setEditingIndex(questions.length);
    } catch (err) {
      console.error("Add failed:", err);
    }
  };

  const handleExportMd = async () => {
    const filePath = await save({
      defaultPath: "questions.md",
      filters: [{ name: "Markdown", extensions: ["md"] }],
    });

    if (!filePath) return;

    try {
      const content = await invoke<string>("export_to_md", {
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
      setAlertMessage(
        `File saved to ${filePath}.\nYou can import this file into Schoology as an .imscc file.`
      );
      setAlertOpen(true);
    } catch (err) {
      console.error("Export failed:", err);
      setStatus(`Export error: ${err}`);
    }
  };

  const handleExportDocx = async () => {
    const filePath = await save({
      defaultPath: "questions.docx",
      filters: [{ name: "Word Document", extensions: ["docx"] }],
    });

    if (!filePath) return;

    try {
      setStatus("Converting to Word document...");
      const data = await invoke<number[]>("export_to_docx", {
        title: "AP CS Questions",
      });
      await writeBinaryFile(filePath, new Uint8Array(data));
      setStatus(`Exported to ${filePath}`);
    } catch (err) {
      console.error("Export failed:", err);
      setStatus(`Export error: ${err}`);
    }
  };

  const handleSaveSession = async () => {
    if (questions.length === 0) {
      setStatus("No questions to save");
      return;
    }

    const filePath = await save({
      defaultPath: "rubrix-session.json",
      filters: [sessionFileFilter],
    });

    if (!filePath) return;

    try {
      const payload = {
        version: 1,
        savedAt: new Date().toISOString(),
        questions,
      };
      await writeTextFile(filePath, JSON.stringify(payload, null, 2));
      setStatus(`Session saved to ${filePath}`);
    } catch (err) {
      console.error("Save session failed:", err);
      setStatus(`Save session error: ${err}`);
    }
  };

  const handleOpenSession = async () => {
    const selection = await open({
      multiple: false,
      filters: [sessionFileFilter],
    });

    if (!selection) return;

    const filePath = Array.isArray(selection) ? selection[0] : selection;
    if (!filePath) return;

    try {
      const content = await readTextFile(filePath);
      const parsed = parseSessionQuestions(JSON.parse(content));
      setQuestions(parsed);
      await invoke("set_questions", { new_questions: parsed });
      setStatus(
        `Loaded ${parsed.length} question${parsed.length === 1 ? "" : "s"} from session`
      );
    } catch (err) {
      console.error("Open session failed:", err);
      setStatus(`Open session error: ${err}`);
    }
  };

  // Determine what to show in main area
  const showStreamingPreview = isGenerating || (streamingText && !streamingComplete);
  const showQuestions = questions.length > 0 && !showStreamingPreview;

  return (
    <div className="flex flex-col h-screen bg-background">
      <AlertModal
        open={alertOpen}
        message={alertMessage}
        onClose={() => setAlertOpen(false)}
      />

      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 border-b bg-white">
        <h1 className="text-xl font-semibold text-foreground">
          📝 Rubrix
          <span className="text-sm font-normal text-muted-foreground ml-2">
            AP CS Test Generator
          </span>
        </h1>

        <div className="flex flex-wrap gap-2 items-center">
          {/* Auth indicator */}
          {isAuthenticated && (
            <div className="flex items-center gap-2 px-3 py-1 bg-green-50 border border-green-200 rounded-md text-sm text-green-700">
              <span>🔐 Authenticated</span>
              <button
                onClick={handleLogout}
                className="text-xs underline hover:no-underline"
              >
                Logout
              </button>
            </div>
          )}

          <div className="flex rounded border overflow-hidden">
            <button
              className={`px-3 py-2 text-sm ${
                activeTab === "generate" ? "bg-primary text-white" : "bg-white"
              }`}
              onClick={() => setActiveTab("generate")}
            >
              Generate
            </button>
            <button
              className={`px-3 py-2 text-sm ${
                activeTab === "bank" ? "bg-primary text-white" : "bg-white"
              }`}
              onClick={() => setActiveTab("bank")}
            >
              Bank Editor
            </button>
          </div>
          <button
            onClick={handleOpenSession}
            className="flex items-center gap-2 px-3 py-2 text-sm border rounded-md hover:bg-secondary"
          >
            <FolderOpen className="w-4 h-4" />
            Open Session
          </button>
          <button
            onClick={handleSaveSession}
            disabled={questions.length === 0}
            className="flex items-center gap-2 px-3 py-2 text-sm border rounded-md hover:bg-secondary disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <SaveIcon className="w-4 h-4" />
            Save Session
          </button>
          {/* Toggle Preview Button (when streaming is complete but still visible) */}
          {streamingText && streamingComplete && (
            <button
              onClick={() => setShowPreview(!showPreview)}
              className="flex items-center gap-2 px-3 py-2 text-sm border rounded-md hover:bg-secondary"
            >
              {showPreview ? (
                <>
                  <EyeOff className="w-4 h-4" />
                  Hide Raw
                </>
              ) : (
                <>
                  <Eye className="w-4 h-4" />
                  Show Raw
                </>
              )}
            </button>
          )}

          {/* Export Buttons */}
          <button
            onClick={handleExportMd}
            disabled={questions.length === 0}
            className="flex items-center gap-2 px-3 py-2 text-sm border rounded-md hover:bg-secondary disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <FileText className="w-4 h-4" />
            Export .md
          </button>
          <button
            onClick={handleExportQti}
            disabled={questions.length === 0}
            className="flex items-center gap-2 px-3 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <FileDown className="w-4 h-4" />
            Export QTI
          </button>
          <button
            onClick={handleExportDocx}
            disabled={questions.length === 0}
            className="flex items-center gap-2 px-3 py-2 text-sm border rounded-md hover:bg-secondary disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <FileDown className="w-4 h-4" />
            Export Word
          </button>
        </div>
      </header>

      <div className="flex flex-1 min-h-0">
        {/* Sidebar */}
        <Sidebar
          subjects={subjects}
          selectedSubject={selectedSubject}
          onSubjectChange={setSelectedSubject}
          topics={topics}
          selectedTopics={selectedTopics}
          onTopicsChange={setSelectedTopics}
          difficulty={difficulty}
          onDifficultyChange={setDifficulty}
          questionCount={questionCount}
          onQuestionCountChange={setQuestionCount}
          notes={notes}
          onNotesChange={setNotes}
          appendMode={appendMode}
          onAppendModeChange={setAppendMode}
          existingCount={questions.length}
          onGenerate={handleGenerate}
          isGenerating={isGenerating}
          collapsed={sidebarCollapsed}
          onToggleCollapsed={() => setSidebarCollapsed((prev) => !prev)}
        />

        {/* Main Content */}
        <div className="flex-1 flex flex-col overflow-hidden">
          {/* Main Area - Either streaming preview/questions or Bank Editor */}
          {activeTab === "generate" ? (
            <main className="flex-1 overflow-hidden flex">
              {/* Streaming Preview Panel */}
              {showPreview && streamingText && (
                <div
                  className={`border-r bg-slate-50 overflow-hidden flex flex-col ${
                    showQuestions ? "w-1/2" : "flex-1"
                  }`}
                >
                  <StreamingPreview text={streamingText} isComplete={streamingComplete} />
                </div>
              )}

              {/* Questions Panel */}
              <div
                className={`overflow-auto p-6 ${
                  showPreview && streamingText ? "flex-1" : "flex-1"
                }`}
              >
                {questions.length === 0 && !isGenerating ? (
                  <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
                    <div className="text-6xl mb-4">📚</div>
                    <p className="text-lg">No questions yet</p>
                    <p className="text-sm">
                      Select topics and click "Generate Questions" to get started
                    </p>
                  </div>
                ) : questions.length === 0 && isGenerating ? (
                  <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
                    <Loader2 className="w-8 h-8 animate-spin mb-4" />
                    <p className="text-lg">Generating questions...</p>
                    <p className="text-sm">Watch the live output on the left</p>
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
              </div>
            </main>
          ) : (
            <main className="flex-1 overflow-auto p-6">
              <BankEditor subject={selectedSubject} />
            </main>
          )}

          {/* Status Bar */}
          <footer className="flex items-center justify-between px-6 py-2 border-t bg-white text-sm text-muted-foreground">
            <span className="flex items-center gap-2">
              {isGenerating && <Loader2 className="w-4 h-4 animate-spin" />}
              {status}
            </span>
            <span>{questions.length} questions</span>
          </footer>
        </div>
      </div>

      {/* Edit Modal */}
      {editingIndex !== null && (
        <EditModal
          question={questions[editingIndex]}
          onSave={handleSaveEdit}
          onClose={() => setEditingIndex(null)}
        />
      )}

      {/* Login Modal */}
      <LoginModal
        isOpen={loginModalOpen}
        onClose={() => {
          setLoginModalOpen(false);
          setAuthError("");
        }}
        onLogin={handleLogin}
        error={authError}
      />
    </div>
  );
}

export default App;
