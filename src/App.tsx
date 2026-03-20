import { useState, useEffect, useRef, useMemo } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/api/dialog";
import { writeBinaryFile, writeTextFile } from "@tauri-apps/api/fs";
import Sidebar from "./components/Sidebar";
import QuestionList from "./components/QuestionList";
import EditModal from "./components/EditModal";
import BankEditor from "./components/BankEditor";
import LoginModal from "./components/LoginModal";
import SubmitBugModal from "./components/SubmitBugModal";
import ExportOptionsModal from "./components/ExportOptionsModal";
import PreferencesModal from "./components/PreferencesModal";
import OpenRecentModal from "./components/OpenRecentModal";
import SaveChangesModal from "./components/SaveChangesModal";
import {
  Question,
  TopicInfo,
  SubjectInfo,
  GenerationRequest,
  WordExportOptions,
  Answer,
  BugSubmissionInput,
  SubmitBugResult,
} from "./types";
import {
  Loader2,
  X,
} from "lucide-react";
import AlertModal from "./components/AlertModal";

// Event payload from Rust backend
interface StreamEvent {
  text: string;
  done: boolean;
  remaining_tokens?: number;
}

interface RegenerateAllQuestionResult {
  index: number;
  question?: Question | null;
  error?: string | null;
}

interface RegenerateAllProgressEvent {
  completed: number;
  total: number;
  index: number;
  success: boolean;
}

const catieFileFilter = { name: "Catie Document", extensions: ["kt"] };
const legacySessionFileFilter = { name: "Legacy Session", extensions: ["json", "md"] };
const remainingTokensStorageKey = "remainingTokens";
const exportPresetStorageKey = "exportPresets";
const preferredSubjectStorageKey = "preferredSubject";
const preferredDifficultyStorageKey = "preferredDifficulty";
const recentDocumentsStorageKey = "recentDocuments";

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
      console.warn(`[Catie Session] ${message}`, meta);
    } else {
      console.warn(`[Catie Session] ${message}`);
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
          : typeof q.question === "string" && q.question.trim().length > 0
          ? q.question
          : stringifyValue(q.text) ?? "Untitled question";

      const answers = coerceAnswers(q, questionIndex);

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
        explanation: coerceRichText(q.explanation) ?? coerceRichText(q.solution),
        rubric: coerceRichText(q.rubric),
        distractors: coerceRichText(q.distractors),
        subject: typeof q.subject === "string" ? q.subject : undefined,
        topics,
        difficulty:
          typeof q.difficulty === "string" && q.difficulty.trim().length > 0
            ? q.difficulty
            : undefined,
      };

      return normalizedQuestion;
    })
    .filter((question: Question | null): question is Question => Boolean(question));

  if (normalized.length === 0) {
    throw new Error("No valid questions found in session file");
  }

  return normalized;
}

function parseMarkdownQuestions(content: string): Question[] {
  const warn = (message: string) => {
    console.warn(`[Catie Markdown] ${message}`);
  };

  // Split by numbered questions (1., 2., 3., etc.)
  const questionBlocks = content.split(/^\d+\.\s+/m).filter(block => block.trim().length > 0);

  if (questionBlocks.length === 0) {
    throw new Error("No questions found in markdown file");
  }

  const questions: Question[] = [];

  questionBlocks.forEach((block, index) => {
    const lines = block.split('\n');

    // Find where answers start (first line starting with "a.")
    let answerStartIndex = lines.findIndex(line => line.trim().startsWith('a.'));

    if (answerStartIndex === -1) {
      warn(`Question ${index + 1} has no answers; skipping.`);
      return;
    }

    // Question text is everything before the first answer
    const questionText = lines.slice(0, answerStartIndex).join('\n').trim();

    if (!questionText) {
      warn(`Question ${index + 1} has no text; skipping.`);
      return;
    }

    // Extract answers (lines starting with "a.")
    const answers: Answer[] = [];
    let currentAnswer = '';
    let answerIndex = 0;

    for (let i = answerStartIndex; i < lines.length; i++) {
      const line = lines[i];

      if (line.startsWith('a.')) {
        // Save previous answer if exists
        if (currentAnswer) {
          answers.push({
            text: currentAnswer.trim(),
            is_correct: answerIndex === 0, // First answer is correct
          });
          answerIndex++;
        }
        // Start new answer, removing "a." prefix and any markdown formatting
        currentAnswer = line.substring(2).trim().replace(/\*\*/g, '');
      } else if (currentAnswer) {
        // Continue multi-line answer
        currentAnswer += '\n' + line;
      }
    }

    // Add last answer
    if (currentAnswer) {
      answers.push({
        text: currentAnswer.trim(),
        is_correct: answerIndex === 0, // First answer is correct
      });
    }

    if (answers.length === 0) {
      warn(`Question ${index + 1} has no valid answers after parsing; skipping.`);
      return;
    }

    questions.push({
      id: `q${index + 1}`,
      text: questionText,
      answers,
      explanation: undefined,
      rubric: undefined,
      distractors: undefined,
      subject: undefined,
      topics: undefined,
      difficulty: undefined,
    });
  });

  if (questions.length === 0) {
    throw new Error("No valid questions found in markdown file");
  }

  return questions;
}

function App() {
  type ExportKind = "md" | "qti" | "word";
  type WordPreset = "teacher_key" | "student_handout";
  type MdPreset = "teacher_markdown" | "student_markdown";
  type QtiPreset = "lms_quiz_package" | "lms_practice_package";
  type DocumentMode = "blank" | "new" | "open";

  // State
  const [subjects, setSubjects] = useState<SubjectInfo[]>([]);
  const [selectedSubject, setSelectedSubject] = useState("");
  const [topics, setTopics] = useState<TopicInfo[]>([]);
  const [selectedTopics, setSelectedTopics] = useState<string[]>([]);
  const [questionType, setQuestionType] = useState("multiple_choice");
  const [frqQuestionType, setFrqQuestionType] = useState("series");
  const questionTypeOptions = [
    { value: "multiple_choice", label: "Multiple Choice" },
    { value: "frq", label: "FRQ" },
  ];
  const frqQuestionTypeOptions = [
    { value: "series", label: "Series" },
    { value: "polar", label: "Polar" },
    { value: "particle_motion", label: "Particle Motion" },
    { value: "parametric", label: "Parametric" },
    { value: "differential_equations", label: "Differential Equations" },
    { value: "limits_continuity", label: "Limits and Continuity" },
  ];
  const [difficulty, setDifficulty] = useState(() => {
    if (typeof localStorage === "undefined") return "medium";
    const saved = localStorage.getItem(preferredDifficultyStorageKey);
    return saved === "easy" || saved === "hard" || saved === "medium" ? saved : "medium";
  });
  const [questionCount, setQuestionCount] = useState(1);
  const [notes, setNotes] = useState("");
  const [questions, setQuestions] = useState<Question[]>([]);
  const [rawTextByQuestionId, setRawTextByQuestionId] = useState<Record<string, string>>({});
  const [isGenerating, setIsGenerating] = useState(false);
  const [editingIndex, setEditingIndex] = useState<number | null>(null);
  const [status, setStatus] = useState("Ready");
  const [currentDocumentPath, setCurrentDocumentPath] = useState<string | null>(null);
  const [documentMode, setDocumentMode] = useState<DocumentMode>("blank");
  const [openRecentOpen, setOpenRecentOpen] = useState(false);
  const [saveChangesOpen, setSaveChangesOpen] = useState(false);
  const [recentDocuments, setRecentDocuments] = useState<string[]>(() => {
    if (typeof localStorage === "undefined") return [];
    try {
      const raw = localStorage.getItem(recentDocumentsStorageKey);
      if (!raw) return [];
      const parsed = JSON.parse(raw);
      return Array.isArray(parsed)
        ? parsed.filter((item): item is string => typeof item === "string" && item.trim().length > 0)
        : [];
    } catch {
      return [];
    }
  });
  const [remainingTokens, setRemainingTokens] = useState<number | null>(() => {
    if (typeof localStorage === "undefined") return null;
    const saved = localStorage.getItem(remainingTokensStorageKey);
    if (!saved) return null;
    const parsed = Number(saved);
    return Number.isFinite(parsed) ? parsed : null;
  });

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
  const [regeneratingQuestionId, setRegeneratingQuestionId] = useState<string | null>(null);
  const [isRegeneratingAll, setIsRegeneratingAll] = useState(false);
  const latestStreamingTextRef = useRef("");
  const [activeTab, setActiveTab] = useState<"generate" | "bank">("generate");
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [wordGeneratePreset, setWordGeneratePreset] = useState<WordPreset>(() => {
    if (typeof localStorage === "undefined") return "teacher_key";
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return "teacher_key";
      const parsed = JSON.parse(saved);
      return parsed.wordGeneratePreset === "student_handout" ? "student_handout" : "teacher_key";
    } catch {
      return "teacher_key";
    }
  });
  const [wordBankPreset, setWordBankPreset] = useState<WordPreset>(() => {
    if (typeof localStorage === "undefined") return "teacher_key";
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return "teacher_key";
      const parsed = JSON.parse(saved);
      return parsed.wordBankPreset === "student_handout" ? "student_handout" : "teacher_key";
    } catch {
      return "teacher_key";
    }
  });
  const [markdownPreset, setMarkdownPreset] = useState<MdPreset>(() => {
    if (typeof localStorage === "undefined") return "teacher_markdown";
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return "teacher_markdown";
      const parsed = JSON.parse(saved);
      return parsed.markdownPreset === "student_markdown" ? "student_markdown" : "teacher_markdown";
    } catch {
      return "teacher_markdown";
    }
  });
  const [qtiPreset, setQtiPreset] = useState<QtiPreset>(() => {
    if (typeof localStorage === "undefined") return "lms_quiz_package";
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return "lms_quiz_package";
      const parsed = JSON.parse(saved);
      return parsed.qtiPreset === "lms_practice_package"
        ? "lms_practice_package"
        : "lms_quiz_package";
    } catch {
      return "lms_quiz_package";
    }
  });
  const [wordIncludeChoices, setWordIncludeChoices] = useState<boolean>(() => {
    if (typeof localStorage === "undefined") return true;
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return true;
      const parsed = JSON.parse(saved);
      return parsed.wordIncludeChoices !== false;
    } catch {
      return true;
    }
  });
  const [wordVersionCount, setWordVersionCount] = useState<number>(() => {
    if (typeof localStorage === "undefined") return 1;
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return 1;
      const parsed = JSON.parse(saved);
      const value = Number.parseInt(String(parsed.wordVersionCount ?? 1), 10);
      if (!Number.isFinite(value)) return 1;
      return Math.min(Math.max(value, 1), 20);
    } catch {
      return 1;
    }
  });
  const [wordShuffleChoices, setWordShuffleChoices] = useState<boolean>(() => {
    if (typeof localStorage === "undefined") return false;
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return false;
      const parsed = JSON.parse(saved);
      return parsed.wordShuffleChoices === true;
    } catch {
      return false;
    }
  });
  const [wordShuffleQuestions, setWordShuffleQuestions] = useState<boolean>(() => {
    if (typeof localStorage === "undefined") return false;
    try {
      const saved = localStorage.getItem(exportPresetStorageKey);
      if (!saved) return false;
      const parsed = JSON.parse(saved);
      return parsed.wordShuffleQuestions === true;
    } catch {
      return false;
    }
  });
  const [exportOptionsOpen, setExportOptionsOpen] = useState(false);
  const [pendingExportKind, setPendingExportKind] = useState<ExportKind | null>(null);
  const [isExporting, setIsExporting] = useState(false);

  // Alert modal state
  const [alertOpen, setAlertOpen] = useState(false);
  const [alertMessage, setAlertMessage] = useState("");

  // Authentication state
  const [loginModalOpen, setLoginModalOpen] = useState(false);
  const [authError, setAuthError] = useState("");
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isDevMode, setIsDevMode] = useState(false);
  const [submitBugOpen, setSubmitBugOpen] = useState(false);
  const [isSubmittingBug, setIsSubmittingBug] = useState(false);
  const [preferencesOpen, setPreferencesOpen] = useState(false);
  const [savedQuestionsSnapshot, setSavedQuestionsSnapshot] = useState<string | null>(null);

  const documentName = currentDocumentPath
    ? currentDocumentPath.split(/[/\\]/).pop() || currentDocumentPath
    : documentMode === "new"
    ? "Untitled.kt"
    : null;
  const questionsSnapshot = useMemo(() => JSON.stringify(questions), [questions]);
  const isDocumentDirty = useMemo(() => {
    if (documentMode === "blank") return false;
    if (documentMode === "new") return questions.length > 0;
    if (savedQuestionsSnapshot === null) return false;
    return questionsSnapshot !== savedQuestionsSnapshot;
  }, [documentMode, questions.length, questionsSnapshot, savedQuestionsSnapshot]);

  const markdownPresetLabel =
    markdownPreset === "teacher_markdown" ? "Teacher Markdown" : "Student Markdown";
  const qtiPresetLabel =
    qtiPreset === "lms_quiz_package" ? "LMS Quiz Package" : "LMS Practice Package";
  const wordGeneratePresetLabel =
    wordGeneratePreset === "teacher_key" ? "Teacher Key" : "Student Handout";
  const wordBankPresetLabel =
    wordBankPreset === "teacher_key" ? "Teacher Key" : "Student Handout";

  // Load subjects and any previously generated questions on mount
  useEffect(() => {
    checkDevMode();
    checkAuthentication();
    loadSubjects();
    void invoke("set_questions", { newQuestions: [] as Question[] }).catch((err) => {
      console.warn("Failed to initialize empty document state:", err);
    });
  }, []);

  useEffect(() => {
    if (typeof localStorage === "undefined") return;
    localStorage.setItem(recentDocumentsStorageKey, JSON.stringify(recentDocuments.slice(0, 10)));
  }, [recentDocuments]);

  const checkDevMode = async () => {
    try {
      const devMode = await invoke<boolean>("is_dev_mode");
      setIsDevMode(devMode);
    } catch (err) {
      console.error("Failed to check dev mode:", err);
    }
  };

  const checkAuthentication = async () => {
    try {
      const isAuth = await invoke<boolean>("auto_authenticate");
      setIsAuthenticated(isAuth);
    } catch (err) {
      console.error("Failed to check auth:", err);
    }
  };

  const normalizeAuthError = (raw: string): string => {
    const text = raw.trim();
    const lower = text.toLowerCase();

    if (lower.includes("user not found") || lower.includes("404")) {
      return "User not recognized. Check your username and try again.";
    }
    if (lower.includes("invalid password") || lower.includes("401")) {
      return "Password incorrect. Please retry.";
    }
    if (lower.includes("failed to connect") || lower.includes("network") || lower.includes("timeout")) {
      return "Unable to reach authentication server. Please retry.";
    }
    if (
      lower.includes("bedrock_gateway_url") ||
      lower.includes("gateway mode is required") ||
      lower.includes("no gateway credentials")
    ) {
      return "Authentication service is not configured. Contact support.";
    }
    if (lower.includes("login did not complete")) {
      return "Login did not complete. Please retry.";
    }

    return text || "Authentication failed. Please retry.";
  };

  const handleLogin = async (username: string, password: string) => {
    setAuthError("");
    try {
      await invoke("authenticate", { username, password });

      // Authenticate only counts as success if backend confirms cached auth state.
      const confirmed = await invoke<boolean>("check_auth");
      if (!confirmed) {
        throw new Error("Login did not complete. Please retry.");
      }

      setIsAuthenticated(true);
      setStatus("Login successful");
    } catch (err) {
      const rawMsg = err instanceof Error ? err.message : String(err);
      const errorMsg = normalizeAuthError(rawMsg);
      setAuthError(errorMsg);
      setIsAuthenticated(false);
      setStatus(`Login failed: ${errorMsg}`);
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

  useEffect(() => {
    if (typeof localStorage === "undefined") return;
    if (remainingTokens === null) return;
    localStorage.setItem(remainingTokensStorageKey, remainingTokens.toString());
  }, [remainingTokens]);

  useEffect(() => {
    if (typeof localStorage === "undefined") return;
    const payload = {
      wordGeneratePreset,
      wordBankPreset,
      markdownPreset,
      qtiPreset,
      wordIncludeChoices,
      wordVersionCount,
      wordShuffleChoices,
      wordShuffleQuestions,
    };
    localStorage.setItem(exportPresetStorageKey, JSON.stringify(payload));
  }, [
    wordGeneratePreset,
    wordBankPreset,
    markdownPreset,
    qtiPreset,
    wordIncludeChoices,
    wordVersionCount,
    wordShuffleChoices,
    wordShuffleQuestions,
  ]);

  useEffect(() => {
    if (typeof localStorage === "undefined") return;
    if (!selectedSubject) return;
    localStorage.setItem(preferredSubjectStorageKey, selectedSubject);
  }, [selectedSubject]);

  useEffect(() => {
    if (typeof localStorage === "undefined") return;
    if (difficulty !== "easy" && difficulty !== "medium" && difficulty !== "hard") return;
    localStorage.setItem(preferredDifficultyStorageKey, difficulty);
  }, [difficulty]);

  // Load topics when subject changes
  useEffect(() => {
    if (selectedSubject) {
      loadTopics(selectedSubject);
    }
  }, [selectedSubject]);

  useEffect(() => {
    if (questionType === "frq" && questionCount !== 1) {
      setQuestionCount(1);
    }
  }, [questionType, questionCount]);

  // Listen for streaming events from backend
  useEffect(() => {
    const unlisten = listen<StreamEvent>("llm-stream", (event) => {
      setStreamingText(event.payload.text);
      latestStreamingTextRef.current = event.payload.text;
      setStreamingComplete(event.payload.done);
      if (typeof event.payload.remaining_tokens === "number") {
        setRemainingTokens(event.payload.remaining_tokens);
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    const unlisten = listen<RegenerateAllProgressEvent>("regenerate-all-progress", (event) => {
      if (!isRegeneratingAll) {
        return;
      }

      const completed = Math.max(0, event.payload.completed || 0);
      const total = Math.max(0, event.payload.total || 0);
      const remaining = Math.max(total - completed, 0);

      if (total > 0) {
        setStatus(`Regenerating ${total} questions... ${remaining} remaining (${completed}/${total})`);
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [isRegeneratingAll]);

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

  // Listen for general app actions from native menu
  useEffect(() => {
    const unlistenAction = listen<string>("app-action", (event) => {
      const action = event.payload;

      if (action === "submit_bug") {
        setSubmitBugOpen(true);
      } else if (action === "new_document") {
        void handleNewDocument();
      } else if (action === "add_custom_question") {
        setActiveTab("generate");
        void handleAddQuestion();
      } else if (action === "close_document") {
        void handleCloseDocumentRequest();
      } else if (action === "open_session") {
        void handleOpenDocument();
      } else if (action === "open_recent") {
        setOpenRecentOpen(true);
      } else if (action === "save_session") {
        void handleSaveDocument();
      } else if (action === "export_md") {
        openExportOptions("md");
      } else if (action === "export_qti") {
        openExportOptions("qti");
      } else if (action === "export_word") {
        openExportOptions("word");
      } else if (action === "open_preferences") {
        setPreferencesOpen(true);
      } else if (action === "switch_generate") {
        setActiveTab("generate");
      } else if (action === "switch_bank") {
        setActiveTab("bank");
      } else if (action === "toggle_raw_preview") {
        if (streamingText) {
          setShowPreview((prev) => !prev);
        } else {
          setStatus("No raw stream available yet");
        }
      } else if (action === "regenerate_all_questions") {
        if (activeTab !== "generate") {
          setStatus("Switch to Generator tab to regenerate questions");
        } else {
          void handleRegenerateAll();
        }
      }
    });

    return () => {
      unlistenAction.then((f) => f());
    };
  }, [streamingText, activeTab, questions.length, selectedSubject, isGenerating, isRegeneratingAll, isAuthenticated]);

  useEffect(() => {
    void invoke("set_menu_state", {
      canExportQuestions: questions.length > 0,
      canExportWord:
        activeTab === "generate" ? questions.length > 0 : Boolean(selectedSubject),
      canSaveSession: questions.length > 0,
      hasRawPreview: Boolean(streamingText),
      canRegenerateAll:
        activeTab === "generate" && questions.length > 0 && !isGenerating && !isRegeneratingAll,
    }).catch((err) => {
      console.warn("Failed to sync menu state:", err);
    });
  }, [questions.length, activeTab, selectedSubject, streamingText, isGenerating, isRegeneratingAll]);

  const loadSubjects = async () => {
    try {
      const subjectList = await invoke<SubjectInfo[]>("get_subjects");
      setSubjects(subjectList);
      if (subjectList.length > 0) {
        const saved =
          typeof localStorage !== "undefined"
            ? localStorage.getItem(preferredSubjectStorageKey)
            : null;
        const hasSaved = saved && subjectList.some((s) => s.id === saved);
        setSelectedSubject(hasSaved ? (saved as string) : subjectList[0].id);
      }
    } catch (err) {
      console.error("Failed to load subjects:", err);
    }
  };

  const handleSavePreferences = (subjectId: string, preferredDifficulty: string) => {
    setSelectedSubject(subjectId);
    if (
      preferredDifficulty === "easy" ||
      preferredDifficulty === "medium" ||
      preferredDifficulty === "hard"
    ) {
      setDifficulty(preferredDifficulty);
    }
    setSelectedTopics([]);
    setPreferencesOpen(false);
    const label = subjects.find((s) => s.id === subjectId)?.name ?? subjectId;
    setStatus(`Preferences saved • Subject: ${label} • Difficulty: ${preferredDifficulty}`);
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

  const handleGenerate = async () => {
    if (isRegeneratingAll) {
      setStatus("Please wait for Regenerate All to finish");
      return;
    }

    if (questionType !== "frq" && selectedTopics.length === 0) {
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
    const previousQuestionCount = questions.length;
    setStreamingText("");
    latestStreamingTextRef.current = "";
    setStreamingComplete(false);
    setShowPreview(true);
    setRegeneratingQuestionId(null);
    setStatus("Adding more questions...");

    try {
      const request: GenerationRequest = {
        subject: selectedSubject,
        topics: selectedTopics,
        difficulty,
        count: questionCount,
        notes: notes || null,
        append: true,
        question_type: questionType,
        frq_question_type: questionType === "frq" ? frqQuestionType : undefined,
      };

      const allQuestions = await invoke<Question[]>("generate_questions", {
        request,
      });
      setQuestions(allQuestions);
      if (documentMode === "blank") {
        setDocumentMode("new");
      }
      const streamSnapshot = latestStreamingTextRef.current.trim();
      setRawTextByQuestionId((prev) => {
        const next: Record<string, string> = {};

        // Keep existing raw text only for IDs still present.
        for (const question of allQuestions) {
          const existing = prev[question.id];
          if (existing) {
            next[question.id] = existing;
          }
        }

        if (streamSnapshot) {
          const targets = allQuestions.slice(previousQuestionCount);
          for (const question of targets) {
            next[question.id] = streamSnapshot;
          }
        }

        return next;
      });

      const addedCount = Math.max(allQuestions.length - previousQuestionCount, 0);
      const failedCount = Math.max(questionCount - addedCount, 0);

      if (failedCount > 0) {
        const qLabel = questionCount === 1 ? "question" : "questions";
        const failLabel = failedCount === 1 ? "failed" : "failed";
        setStatus(
          `Generated ${addedCount} of ${questionCount} ${qLabel}, ${failedCount} ${failLabel} (${allQuestions.length} total)`
        );
      } else {
        setStatus(`Added ${addedCount} questions (${allQuestions.length} total)`);
      }
    } catch (err) {
      console.error("Generation failed:", err);
      const errorMsg = String(err);

      // If error suggests missing auth, show login modal
      if (
        errorMsg.includes("BEDROCK_GATEWAY_URL") ||
        errorMsg.toLowerCase().includes("gateway") ||
        errorMsg.toLowerCase().includes("auth")
      ) {
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
    if (isGenerating || isRegeneratingAll) {
      return;
    }

    setStatus(`Regenerating question ${index + 1}...`);
    const previousQuestionId = questions[index]?.id;
    setRegeneratingQuestionId(previousQuestionId ?? null);
    setStreamingText("");
    latestStreamingTextRef.current = "";
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
      const streamSnapshot = latestStreamingTextRef.current.trim();
      setRawTextByQuestionId((prev) => {
        const next = { ...prev };
        if (previousQuestionId && previousQuestionId !== newQuestion.id) {
          delete next[previousQuestionId];
        }
        if (streamSnapshot) {
          next[newQuestion.id] = streamSnapshot;
        }
        return next;
      });
      setStatus("Question regenerated");
    } catch (err) {
      console.error("Regeneration failed:", err);
      setStatus(`Error: ${err}`);
    } finally {
      setRegeneratingQuestionId(null);
    }
  };

  const handleRegenerateAll = async () => {
    if (isGenerating || isRegeneratingAll) {
      return;
    }

    if (questions.length === 0) {
      setStatus("No questions to regenerate");
      return;
    }

    if (!isAuthenticated) {
      setLoginModalOpen(true);
      setStatus("Authentication required");
      return;
    }

    setIsRegeneratingAll(true);
    const total = questions.length;
    const previousQuestionIds = questions.map((question) => question.id);
    setRegeneratingQuestionId(null);
    setStreamingText("");
    latestStreamingTextRef.current = "";
    setStreamingComplete(true);
    setShowPreview(false);
    setStatus(`Regenerating ${total} questions...`);

    try {
      const results = await invoke<RegenerateAllQuestionResult[]>(
        "regenerate_all_questions_parallel",
        {
          maxConcurrency: 3,
        }
      );

      const succeededIndexes = results
        .filter((item): item is RegenerateAllQuestionResult & { question: Question } => Boolean(item.question))
        .map((item) => item.index);
      const failedResults = results.filter((item) => !item.question);

      setQuestions((prev) => {
        const updated = [...prev];
        for (const item of results) {
          if (!item.question) continue;
          if (item.index < 0 || item.index >= updated.length) continue;
          updated[item.index] = item.question;
        }
        return updated;
      });

      setRawTextByQuestionId((prev) => {
        const next = { ...prev };
        for (const index of succeededIndexes) {
          const questionId = previousQuestionIds[index];
          if (questionId) {
            delete next[questionId];
          }
        }
        return next;
      });

      const successCount = succeededIndexes.length;
      const failureCount = failedResults.length;

      if (failureCount === 0) {
        setStatus(`Regenerated all ${successCount} questions`);
      } else {
        console.error("Regenerate all partial failures:", failedResults);

        const failedLabels = failedResults.map((item) => `Q${item.index + 1}`);
        const maxInline = 8;
        const visible = failedLabels.slice(0, maxInline);
        const overflow = failedLabels.length - visible.length;
        const failedInline =
          overflow > 0 ? `${visible.join(", ")}, +${overflow} more` : visible.join(", ");

        setStatus(
          `Regenerated ${successCount}/${total} questions (${failureCount} failed: ${failedInline})`
        );

        const detailLines = failedResults.map(
          (item) => `Q${item.index + 1}: ${(item.error || "Unknown error").trim()}`
        );
        setAlertMessage(`Some questions failed to regenerate:\n\n${detailLines.join("\n")}`);
        setAlertOpen(true);
      }
    } catch (err) {
      console.error("Regenerate all failed:", err);
      setStatus(`Error: ${err}`);
    } finally {
      setRegeneratingQuestionId(null);
      setIsRegeneratingAll(false);
    }
  };

  const handleEdit = (index: number) => {
    setEditingIndex(index);
  };

  const handleSaveEdit = async (question: Question) => {
    if (editingIndex === null) return;
    const previousId = questions[editingIndex]?.id;

    try {
      await invoke("update_question", { index: editingIndex, question });
      setQuestions((prev) => {
        const updated = [...prev];
        updated[editingIndex] = question;
        return updated;
      });
      if (previousId && previousId !== question.id) {
        setRawTextByQuestionId((rawPrev) => {
          const next = { ...rawPrev };
          if (rawPrev[previousId]) {
            next[question.id] = rawPrev[previousId];
          }
          delete next[previousId];
          return next;
        });
      }
      setEditingIndex(null);
      setStatus("Question updated");
    } catch (err) {
      console.error("Update failed:", err);
      setStatus(`Error: ${err}`);
    }
  };

  const handleDelete = async (index: number) => {
    try {
      const deletedId = questions[index]?.id;
      await invoke("delete_question", { index });
      setQuestions((prev) => prev.filter((_, i) => i !== index));
      if (deletedId) {
        setRawTextByQuestionId((prev) => {
          const next = { ...prev };
          delete next[deletedId];
          return next;
        });
      }
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
      if (documentMode === "blank") {
        setDocumentMode("new");
      }
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
      setIsExporting(true);
      const options = {
        include_explanations: markdownPreset === "teacher_markdown",
        include_answer_key: markdownPreset === "teacher_markdown",
      };
      const content = await invoke<string>("export_to_md", {
        title: "Quiz",
        options,
      });
      await writeTextFile(filePath, content);
      setStatus(`Exported to ${filePath} • Preset: ${markdownPresetLabel}`);
    } catch (err) {
      console.error("Export failed:", err);
      setStatus(`Export error: ${err}`);
    } finally {
      setIsExporting(false);
    }
  };

  const handleExportQti = async () => {
    const filePath = await save({
      defaultPath: "questions.imscc",
      filters: [{ name: "IMS Common Cartridge", extensions: ["imscc", "zip"] }],
    });

    if (!filePath) return;

    try {
      setIsExporting(true);
      const options = {
        shuffle_choices: qtiPreset === "lms_quiz_package",
      };
      const data = await invoke<number[]>("export_to_qti", {
        title: "Quiz",
        options,
      });
      await writeBinaryFile(filePath, new Uint8Array(data));
      setStatus(`Exported to ${filePath} • Preset: ${qtiPresetLabel}`);
      setAlertMessage(
        `File saved to ${filePath}.\nYou can import this file into Schoology as an .imscc file.`
      );
      setAlertOpen(true);
    } catch (err) {
      console.error("Export failed:", err);
      setStatus(`Export error: ${err}`);
    } finally {
      setIsExporting(false);
    }
  };

  const handleExportDocx = async () => {
    const exportingBank = activeTab === "bank";
    const defaultName = exportingBank
      ? `${selectedSubject || "question-bank"}.docx`
      : "questions.docx";

    const filePath = await save({
      defaultPath: defaultName,
      filters: [{ name: "Word Document", extensions: ["docx"] }],
    });

    if (!filePath) return;

    try {
      setIsExporting(true);
      const normalizedVersionCount = Math.min(Math.max(wordVersionCount, 1), 20);
      const includeChoices = wordIncludeChoices;
      const useChoiceShuffle = includeChoices && normalizedVersionCount > 1 && wordShuffleChoices;
      const useQuestionShuffle = normalizedVersionCount > 1 && wordShuffleQuestions;

      if (exportingBank) {
        if (!selectedSubject) {
          setStatus("Select a subject before exporting the question bank");
          return;
        }

        setStatus("Converting question bank to Word document...");
        const options: WordExportOptions = {
          include_explanations: wordBankPreset === "teacher_key",
          include_choices: includeChoices,
          version_count: normalizedVersionCount,
          shuffle_choices: useChoiceShuffle,
          shuffle_questions: useQuestionShuffle,
        };
        const data = await invoke<number[]>("export_question_bank_to_docx", {
          subject: selectedSubject,
          title: `${selectedSubject} Question Bank`,
          options,
        });
        await writeBinaryFile(filePath, new Uint8Array(data));
        setStatus(
          `Exported to ${filePath} • Preset: ${wordBankPresetLabel} • Versions: ${normalizedVersionCount}`
        );
      } else {
        setStatus("Converting to Word document...");
        const options: WordExportOptions = {
          include_explanations: wordGeneratePreset === "teacher_key",
          include_choices: includeChoices,
          version_count: normalizedVersionCount,
          shuffle_choices: useChoiceShuffle,
          shuffle_questions: useQuestionShuffle,
        };
        const data = await invoke<number[]>("export_to_docx", {
          title: "Quiz",
          options,
        });
        await writeBinaryFile(filePath, new Uint8Array(data));
        setStatus(
          `Exported to ${filePath} • Preset: ${wordGeneratePresetLabel} • Versions: ${normalizedVersionCount}`
        );
      }
    } catch (err) {
      console.error("Export failed:", err);
      setStatus(`Export error: ${err}`);
    } finally {
      setIsExporting(false);
    }
  };

  const canRunExport = (kind: ExportKind): boolean => {
    if (kind === "word") {
      return activeTab === "generate" ? questions.length > 0 : Boolean(selectedSubject);
    }
    return questions.length > 0;
  };

  const openExportOptions = (kind: ExportKind) => {
    if (!canRunExport(kind)) {
      if (kind === "word") {
        setStatus(
          activeTab === "generate"
            ? "Generate questions before exporting Word"
            : "Select a subject before exporting the question bank"
        );
      } else {
        setStatus("Generate questions before exporting");
      }
      return;
    }

    setPendingExportKind(kind);
    setExportOptionsOpen(true);
  };

  const confirmExportFromOptions = async () => {
    if (!pendingExportKind) return;

    const kind = pendingExportKind;
    setExportOptionsOpen(false);
    setPendingExportKind(null);

    if (kind === "md") {
      await handleExportMd();
      return;
    }

    if (kind === "qti") {
      await handleExportQti();
      return;
    }

    await handleExportDocx();
  };

  const updateRecentDocuments = (filePath: string) => {
    setRecentDocuments((prev) => [filePath, ...prev.filter((p) => p !== filePath)].slice(0, 10));
  };

  const resetToBlankState = async () => {
    setQuestions([]);
    setRawTextByQuestionId({});
    setStreamingText("");
    latestStreamingTextRef.current = "";
    setStreamingComplete(false);
    setRegeneratingQuestionId(null);
    setCurrentDocumentPath(null);
    setDocumentMode("blank");
    setSavedQuestionsSnapshot(null);
    setActiveTab("generate");
    setStatus("Ready");
    try {
      await invoke("set_questions", { newQuestions: [] as Question[] });
    } catch (err) {
      console.error("Failed to clear backend question state:", err);
    }
  };

  const handleCloseDocumentRequest = async () => {
    if (documentMode === "blank") return;
    if (isDocumentDirty) {
      setSaveChangesOpen(true);
      return;
    }
    await resetToBlankState();
  };

  const handleNewDocument = async () => {
    setQuestions([]);
    setRawTextByQuestionId({});
    setStreamingText("");
    latestStreamingTextRef.current = "";
    setStreamingComplete(false);
    setRegeneratingQuestionId(null);
    setCurrentDocumentPath(null);
    setDocumentMode("new");
    setSavedQuestionsSnapshot(JSON.stringify([]));
    setStatus("New document • Unsaved (Untitled.kt)");
    try {
      await invoke("set_questions", { newQuestions: [] as Question[] });
    } catch (err) {
      console.error("Failed to clear backend question state:", err);
    }
  };

  const openDocumentFromPath = async (filePath: string) => {
    const content = await invoke<string>("read_document_file", { path: filePath });

    // Detect file type and parse accordingly
    let parsed: Question[];
    if (filePath.toLowerCase().endsWith(".md")) {
      parsed = parseMarkdownQuestions(content);
    } else {
      parsed = parseSessionQuestions(JSON.parse(content));
    }

    setQuestions(parsed);
    setRawTextByQuestionId({});
    setCurrentDocumentPath(filePath);
    setDocumentMode("open");
    setSavedQuestionsSnapshot(JSON.stringify(parsed));
    updateRecentDocuments(filePath);
    await invoke("set_questions", { newQuestions: parsed });
    setStatus(`Opened ${parsed.length} question${parsed.length === 1 ? "" : "s"} from ${filePath}`);
  };

  const handleOpenRecentPath = async (filePath: string) => {
    try {
      await openDocumentFromPath(filePath);
      setOpenRecentOpen(false);
    } catch (err) {
      console.error("Open recent failed:", err);
      setRecentDocuments((prev) => prev.filter((p) => p !== filePath));
      setStatus(`Open recent error: ${err}`);
    }
  };

  const handleClearRecent = () => {
    setRecentDocuments([]);
    setStatus("Cleared recent documents");
  };

  const handleSaveDocument = async (): Promise<boolean> => {
    if (questions.length === 0) {
      setStatus("No questions to save");
      return false;
    }

    let filePath = currentDocumentPath;

    if (!filePath) {
      filePath = await save({
        defaultPath: "untitled.kt",
        filters: [catieFileFilter],
      });
    }

    if (!filePath) return false;

    try {
      const payload = {
        version: 1,
        savedAt: new Date().toISOString(),
        questions,
      };
      await invoke("write_document_file", {
        path: filePath,
        content: JSON.stringify(payload, null, 2),
      });
      setCurrentDocumentPath(filePath);
      setDocumentMode("open");
      setSavedQuestionsSnapshot(JSON.stringify(questions));
      updateRecentDocuments(filePath);
      setStatus(`Saved ${filePath}`);
      return true;
    } catch (err) {
      console.error("Save session failed:", err);
      setStatus(`Save session error: ${err}`);
      return false;
    }
  };

  const handleOpenDocument = async () => {
    const selection = await open({
      multiple: false,
      filters: [catieFileFilter, legacySessionFileFilter],
    });

    if (!selection) return;

    const filePath = Array.isArray(selection) ? selection[0] : selection;
    if (!filePath) return;

    try {
      await openDocumentFromPath(filePath);
    } catch (err) {
      console.error("Open session failed:", err);
      setStatus(`Open session error: ${err}`);
    }
  };

  const handleSubmitBug = async (payload: {
    title: string;
    description: string;
    steps: string;
    expectedBehavior: string;
    actualBehavior: string;
    severity: "low" | "medium" | "high" | "critical";
    reporterEmail: string;
    includeDiagnostics: boolean;
  }) => {
    setIsSubmittingBug(true);
    setStatus("Submitting bug report...");

    try {
      const stepsToReproduce = payload.steps
        .split("\n")
        .map((line) => line.trim())
        .filter((line) => line.length > 0);

      const userAgent = typeof navigator !== "undefined" ? navigator.userAgent : "unknown";

      const input: BugSubmissionInput = {
        title: payload.title,
        description: payload.description,
        steps_to_reproduce: stepsToReproduce,
        expected_behavior: payload.expectedBehavior || undefined,
        actual_behavior: payload.actualBehavior || undefined,
        severity: payload.severity,
        reporter_email: payload.reporterEmail || undefined,
        include_diagnostics: payload.includeDiagnostics,
        client_context: {
          selected_subject: selectedSubject || null,
          selected_topics: selectedTopics,
          question_count: questions.length,
          active_tab: activeTab,
          status,
          is_authenticated: isAuthenticated,
          is_dev_mode: isDevMode,
          app_zoom: zoom,
          preview_visible: showPreview,
          streaming_chars: streamingText.length,
          user_agent: userAgent,
          captured_at: new Date().toISOString(),
        },
      };

      const result = await invoke<SubmitBugResult>("submit_bug_report", { input });
      setStatus(`Bug submitted (${result.event_id})`);
      setAlertMessage(
        result.upstream_url
          ? `Bug submitted successfully.\nEvent ID: ${result.event_id}\nLink: ${result.upstream_url}`
          : `Bug submitted successfully.\nEvent ID: ${result.event_id}`
      );
      setAlertOpen(true);
      setSubmitBugOpen(false);
    } catch (err) {
      console.error("Bug submission failed:", err);
      setStatus(`Bug submission failed: ${err}`);
      throw err;
    } finally {
      setIsSubmittingBug(false);
    }
  };

  // Determine what to show in main area
  const showStreamingCard = isGenerating;
  const topicMetaById = useMemo(() => {
    const meta: Record<string, { label: string; kind: "topic" | "subtopic" }> = {};
    for (const topic of topics) {
      meta[topic.id] = { label: topic.name, kind: "topic" };
      for (const child of topic.children ?? []) {
        meta[child.id] = { label: child.name, kind: "subtopic" };
      }
    }
    return meta;
  }, [topics]);
  const selectedSubjectName = subjects.find((s) => s.id === selectedSubject)?.name ?? selectedSubject;
  const appHeading = `${selectedSubjectName || "Catie"} Question Generator`;

  return (
    <div className="flex flex-col h-screen bg-background">
      <AlertModal
        open={alertOpen}
        message={alertMessage}
        onClose={() => setAlertOpen(false)}
      />
      <OpenRecentModal
        open={openRecentOpen}
        recentPaths={recentDocuments}
        onOpenPath={handleOpenRecentPath}
        onClear={handleClearRecent}
        onClose={() => setOpenRecentOpen(false)}
      />
      <SaveChangesModal
        open={saveChangesOpen}
        documentName={documentName ?? "Untitled.kt"}
        onCancel={() => setSaveChangesOpen(false)}
        onDontSave={() => {
          setSaveChangesOpen(false);
          void resetToBlankState();
        }}
        onSave={() => {
          void (async () => {
            const saved = await handleSaveDocument();
            if (!saved) return;
            setSaveChangesOpen(false);
            await resetToBlankState();
          })();
        }}
      />

      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 border-b bg-white">
        <h1 className="text-lg font-semibold text-foreground">{appHeading}</h1>
        <span className="text-xs text-muted-foreground">
          Start with File &gt; New or Open. Switch modes in View menu (`CmdOrCtrl+1/2`).
        </span>
      </header>

      <div className="flex flex-1 min-h-0">
        {/* Sidebar */}
        <div
          className={documentMode === "blank" ? "pointer-events-none opacity-50" : ""}
          aria-disabled={documentMode === "blank"}
        >
          <Sidebar
            topics={topics}
            questionType={questionType}
            questionTypeOptions={questionTypeOptions}
            onQuestionTypeChange={setQuestionType}
            frqQuestionType={frqQuestionType}
            frqQuestionTypeOptions={frqQuestionTypeOptions}
            onFrqQuestionTypeChange={setFrqQuestionType}
            selectedTopics={selectedTopics}
            onTopicsChange={setSelectedTopics}
            difficulty={difficulty}
            onDifficultyChange={setDifficulty}
            questionCount={questionCount}
            onQuestionCountChange={setQuestionCount}
            notes={notes}
            onNotesChange={setNotes}
            existingCount={questions.length}
            onGenerate={handleGenerate}
            isGenerating={isGenerating}
            collapsed={sidebarCollapsed}
            onToggleCollapsed={() => setSidebarCollapsed((prev) => !prev)}
          />
        </div>

        {/* Main Content */}
        <div className="flex-1 flex flex-col overflow-hidden">
          <div className="border-b bg-slate-50/80 px-4 pt-2">
            <div className="min-h-9 flex items-end">
              {documentName ? (
                <div className="inline-flex items-center gap-2 rounded-t-md border border-b-0 bg-white px-3 py-1.5 text-sm shadow-sm max-w-full">
                  <span className="truncate max-w-[22rem]" title={documentName}>
                    {documentName}
                    {isDocumentDirty ? " *" : ""}
                  </span>
                  <button
                    type="button"
                    onClick={() => {
                      void handleCloseDocumentRequest();
                    }}
                    className="rounded p-0.5 text-muted-foreground hover:bg-secondary hover:text-foreground"
                    title="Close document"
                  >
                    <X className="w-3.5 h-3.5" />
                  </button>
                </div>
              ) : (
                <span className="px-2 py-1.5 text-xs text-muted-foreground">No document open</span>
              )}
            </div>
          </div>

          {/* Main Area - Either streaming preview/questions or Bank Editor */}
          {activeTab === "generate" ? (
            <main className="flex-1 overflow-auto overscroll-contain p-6">
              {questions.length === 0 && !showStreamingCard ? (
                  <div className="h-full flex items-center justify-center text-muted-foreground">
                    <p className="text-sm">
                      {documentMode === "blank"
                        ? "Choose File > New or Open to get started."
                        : "Add a question to get started."}
                    </p>
                  </div>
              ) : (
                <QuestionList
                  questions={questions}
                  topicMetaById={topicMetaById}
                  rawTextByQuestionId={rawTextByQuestionId}
                  regeneratingQuestionId={regeneratingQuestionId}
                  isRegeneratingAll={isRegeneratingAll}
                  regenerationStreamingText={streamingText}
                  regenerationStreamingComplete={streamingComplete}
                  showStreamingCard={showStreamingCard}
                  streamingText={streamingText}
                  streamingComplete={streamingComplete}
                  showRawStream={showPreview}
                  onToggleRawStream={() => setShowPreview((prev) => !prev)}
                  onRegenerate={handleRegenerate}
                  onEdit={handleEdit}
                  onDelete={handleDelete}
                />
              )}
            </main>
          ) : (
            <main className="flex-1 overflow-auto overscroll-contain p-6">
              <BankEditor subject={selectedSubject} />
            </main>
          )}

          {/* Status Bar */}
          <footer className="flex flex-wrap items-center justify-between gap-2 px-6 py-2 border-t bg-white text-sm text-muted-foreground">
            <span className="flex items-center gap-2 min-w-0">
              {(isGenerating || isRegeneratingAll) && <Loader2 className="w-4 h-4 animate-spin" />}
              <span className="truncate">{status}</span>
            </span>
            <div className="flex flex-wrap items-center justify-end gap-2">
              {isDevMode && (
                <span className="px-2 py-0.5 rounded border border-yellow-300 bg-yellow-50 text-yellow-800 text-xs font-medium">
                  DEV MODE
                </span>
              )}
              {isAuthenticated && (
                <span className="inline-flex items-center gap-2 px-2 py-0.5 rounded border border-green-200 bg-green-50 text-green-700 text-xs font-medium">
                  Authenticated
                  <button
                    onClick={handleLogout}
                    className="underline hover:no-underline"
                    title="Clears saved login"
                  >
                    Logout
                  </button>
                </span>
              )}
              <span>
                {questions.length} questions
                {` • ${remainingTokens !== null ? remainingTokens.toLocaleString() : "N/A"} tokens left`}
              </span>
            </div>
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

      <SubmitBugModal
        isOpen={submitBugOpen}
        isSubmitting={isSubmittingBug}
        onClose={() => setSubmitBugOpen(false)}
        onSubmit={handleSubmitBug}
      />

      <ExportOptionsModal
        isOpen={exportOptionsOpen}
        kind={pendingExportKind}
        activeTab={activeTab}
        isExporting={isExporting}
        wordGeneratePreset={wordGeneratePreset}
        wordBankPreset={wordBankPreset}
        markdownPreset={markdownPreset}
        qtiPreset={qtiPreset}
        wordIncludeChoices={wordIncludeChoices}
        wordVersionCount={wordVersionCount}
        wordShuffleChoices={wordShuffleChoices}
        wordShuffleQuestions={wordShuffleQuestions}
        onChangeWordGeneratePreset={setWordGeneratePreset}
        onChangeWordBankPreset={setWordBankPreset}
        onChangeMarkdownPreset={setMarkdownPreset}
        onChangeQtiPreset={setQtiPreset}
        onChangeWordIncludeChoices={setWordIncludeChoices}
        onChangeWordVersionCount={setWordVersionCount}
        onChangeWordShuffleChoices={setWordShuffleChoices}
        onChangeWordShuffleQuestions={setWordShuffleQuestions}
        onCancel={() => {
          if (isExporting) return;
          setExportOptionsOpen(false);
          setPendingExportKind(null);
        }}
        onConfirm={confirmExportFromOptions}
      />

      <PreferencesModal
        isOpen={preferencesOpen}
        subjects={subjects}
        selectedSubject={selectedSubject}
        selectedDifficulty={difficulty}
        onSave={handleSavePreferences}
        onClose={() => setPreferencesOpen(false)}
      />
    </div>
  );
}

export default App;
