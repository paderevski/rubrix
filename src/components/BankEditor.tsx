import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { QuestionBankDocument, QuestionBankEntry, TopicInfo } from "../types";
import { Loader2, Save, RotateCcw, X, Plus } from "lucide-react";
import ReactMarkdown, { Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import rehypeRaw from "rehype-raw";
import "katex/dist/katex.min.css";

const markdownComponents = {
  code(props: any) {
    const { inline, className, children, ...rest } = props;
    const match = /language-(\w+)/.exec(className || "");
    if (!inline && match) {
      return (
        <pre className="bg-slate-50 text-slate-900 border border-slate-200 rounded p-3 overflow-auto text-xs" {...rest}>
          <code>{String(children).replace(/\n$/, "")}</code>
        </pre>
      );
    }
    return (
      <code className="px-1 py-0.5 bg-slate-100 text-slate-800 rounded text-xs font-mono" {...rest}>
        {children}
      </code>
    );
  },
};

function normalizeMathDelimiters(content: string) {
  if (!content) return "";
  const normalizedEscapes = content.replace(/\\\\(?=[A-Za-z])/g, "\\");
  const envPattern =
    /(^|\n)([ \t]*)(\\{1,2}begin\{(align\*?|aligned|alignat\*?|gather\*?|multline\*?|split|eqnarray\*?|array|cases)\}[\s\S]*?\\{1,2}end\{\4\})(?=\n|$)/g;

  const wrapBareMathEnvironments = (input: string) =>
    input.replace(envPattern, (fullMatch, lineStart, indent, block, _envName, offset, source) => {
      const before = source.slice(0, offset).trimEnd();
      const after = source.slice(offset + fullMatch.length).trimStart();

      // If this block is already inside a $$...$$ region, keep it unchanged.
      if (before.endsWith("$$") && after.startsWith("$$")) {
        return fullMatch;
      }

      return `${lineStart}${indent}$$\n${block}\n$$`;
    });

  return wrapBareMathEnvironments(normalizedEscapes)
    .replace(/\\\(/g, "$")
    .replace(/\\\)/g, "$")
    .replace(/(^|[^\\])\\\[/g, "$1$$")
    .replace(/(^|[^\\])\\\]/g, "$1$$");
}

function RichMarkdown({
  content,
  className,
  components,
}: {
  content: string;
  className?: string;
  components?: Components;
}) {
  const normalized = normalizeMathDelimiters(content);
  return (
    <ReactMarkdown
      className={className}
      remarkPlugins={[remarkGfm, remarkMath]}
      rehypePlugins={[rehypeRaw, rehypeKatex]}
      components={{ ...markdownComponents, ...(components || {}) }}
    >
      {normalized}
    </ReactMarkdown>
  );
}

interface BankEditorProps {
  subject: string;
}

export default function BankEditor({ subject }: BankEditorProps) {
  const [bankFiles, setBankFiles] = useState<string[]>([]);
  const [selectedFile, setSelectedFile] = useState("");
  const [document, setDocument] = useState<QuestionBankDocument | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dirty, setDirty] = useState(false);
  const [topicOptions, setTopicOptions] = useState<TopicInfo[]>([]);

  const selected = useMemo(
    () => document?.questions.find((e) => e.id === selectedId) || null,
    [document, selectedId]
  );

  useEffect(() => {
    if (!subject) return;
    loadBankFiles();
    loadTopics();
  }, [subject]);

  const loadBankFiles = async () => {
    setLoading(true);
    setError(null);
    try {
      const files = await invoke<string[]>("list_question_bank_files", { subject });
      setBankFiles(files);
      const initialFile = files[0] ?? "";
      setSelectedFile(initialFile);
      if (!initialFile) {
        setDocument(null);
        setSelectedId(null);
        setDirty(false);
        return;
      }
      const data = await invoke<QuestionBankDocument>("load_question_bank_file", {
        subject,
        fileName: initialFile,
      });
      setDocument(data);
      setSelectedId(data.questions[0]?.id ?? null);
      setDirty(false);
    } catch (e: any) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const openBankFile = async (fileName: string) => {
    if (!fileName) return;
    setLoading(true);
    setError(null);
    try {
      const data = await invoke<QuestionBankDocument>("load_question_bank_file", {
        subject,
        fileName,
      });
      setDocument(data);
      setSelectedFile(fileName);
      setSelectedId(data.questions[0]?.id ?? null);
      setDirty(false);
    } catch (e: any) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const loadTopics = async () => {
    try {
      const data = await invoke<TopicInfo[]>("get_topics", { subject });
      setTopicOptions(data);
    } catch (e) {
      console.error("Failed to load topics", e);
    }
  };

  const updateEntry = (id: string, patch: Partial<QuestionBankEntry>) => {
    setDocument((prev) =>
      prev
        ? {
            ...prev,
            questions: prev.questions.map((entry) =>
              entry.id === id ? { ...entry, ...patch } : entry
            ),
          }
        : prev
    );
    setDirty(true);
  };

  const updateAnswer = (
    id: string,
    answerId: string,
    patch: Partial<QuestionBankEntry["answers"][number]>
  ) => {
    setDocument((prev) =>
      prev
        ? {
            ...prev,
            questions: prev.questions.map((entry) =>
              entry.id === id
                ? {
                    ...entry,
                    answers: entry.answers.map((answer) =>
                      answer.id === answerId ? { ...answer, ...patch } : answer
                    ),
                  }
                : entry
            ),
          }
        : prev
    );
    setDirty(true);
  };

  const updateDocument = (patch: Partial<QuestionBankDocument>) => {
    setDocument((prev) => (prev ? { ...prev, ...patch } : prev));
    setDirty(true);
  };

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    try {
      if (!document) return;
      await invoke("save_question_bank_file", {
        subject,
        fileName: selectedFile,
        document,
      });
      setDirty(false);
    } catch (e: any) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  };

  const handleDiscard = () => {
    openBankFile(selectedFile);
  };

  const renderList = () => (
    <div className="w-64 border-r bg-slate-50 h-full overflow-auto">
      <div className="px-3 py-2 border-b bg-white">
        <div className="flex items-center justify-between mb-2">
          <div className="text-sm font-semibold">Questions</div>
          {loading && <Loader2 className="w-4 h-4 animate-spin text-primary" />}
        </div>
        <label className="block text-xs font-medium text-slate-600 mb-1" htmlFor="bank-file-select">
          Bank file
        </label>
        <select
          id="bank-file-select"
          className="w-full border rounded px-2 py-1.5 text-sm"
          value={selectedFile}
          disabled={loading || bankFiles.length === 0}
          onChange={(event) => {
            const nextFile = event.target.value;
            if (dirty && !window.confirm("Discard unsaved changes and open another bank file?")) {
              event.target.value = selectedFile;
              return;
            }
            openBankFile(nextFile);
          }}
        >
          {bankFiles.length === 0 && <option value="">No bank files found</option>}
          {bankFiles.map((fileName) => (
            <option key={fileName} value={fileName}>{fileName}</option>
          ))}
        </select>
      </div>
      <div className="divide-y">
        {(document?.questions ?? []).map((q) => (
          <button
            key={q.id}
            onClick={() => setSelectedId(q.id)}
            className={`w-full text-left px-3 py-2 text-sm hover:bg-primary/5 ${
              q.id === selectedId ? "bg-primary/10" : ""
            }`}
          >
            <div className="font-semibold">{q.id}</div>
            <div className="text-xs text-slate-600 line-clamp-2">{q.text}</div>
          </button>
        ))}
        {(document?.questions.length ?? 0) === 0 && (
          <div className="px-3 py-4 text-xs text-muted-foreground">
            No questions loaded.
          </div>
        )}
      </div>
    </div>
  );

  const renderEditor = () => {
    if (!selected) {
      return (
        <div className="flex-1 flex items-center justify-center text-sm text-muted-foreground">
          Select a question to edit.
        </div>
      );
    }

    return (
      <div className="flex-1 overflow-auto">
        <div className="flex items-center gap-2 px-4 py-3 border-b bg-white sticky top-0">
          <div className="text-sm font-semibold">Editing {selected.id}</div>
          {dirty && <span className="text-xs text-amber-600">Unsaved</span>}
          <div className="ml-auto flex gap-2">
            <button
              onClick={handleDiscard}
              className="flex items-center gap-1 px-3 py-1 text-sm border rounded hover:bg-secondary"
              disabled={loading}
            >
              <RotateCcw className="w-4 h-4" />
              Discard
            </button>
            <button
              onClick={handleSave}
              disabled={saving || loading || !dirty}
              className="flex items-center gap-1 px-3 py-1 text-sm border rounded bg-primary text-white disabled:opacity-50"
            >
              {saving ? <Loader2 className="w-4 h-4 animate-spin" /> : <Save className="w-4 h-4" />}
              Save
            </button>
          </div>
        </div>

        {error && (
          <div className="m-4 p-3 rounded border border-red-200 bg-red-50 text-sm text-red-800">
            {error}
          </div>
        )}

        <div className="p-4 space-y-6">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 items-stretch">
            {/* Left: Question + Preview */}
            <div className="flex flex-col gap-3 h-full">
              <label className="block text-xs font-semibold text-slate-700 mb-1">Question</label>
              <div className="prose prose-sm max-w-none prose-pre:!bg-slate-50 prose-pre:!text-slate-900 prose-pre:!border prose-pre:!border-slate-200 prose-code:!bg-slate-100 prose-code:!text-slate-800 border border-blue-100 rounded p-3 bg-blue-50">
                <RichMarkdown content={selected.text} />
              </div>
              <textarea
                className="w-full border rounded p-2 text-sm flex-1 min-h-[200px]"
                rows={8}
                value={selected.text}
                onChange={(e) => updateEntry(selected.id, { text: e.target.value })}
              />
            </div>

            {/* Right: Metadata */}
            <div className="flex flex-col gap-4 h-full">
              <div>
                <label className="block text-xs font-semibold text-slate-700 mb-1">Source</label>
                <input
                  className="w-full border rounded p-2 text-sm"
                  value={document?.source ?? ""}
                  onChange={(e) => updateDocument({ source: e.target.value })}
                />
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                <div>
                  <label className="block text-xs font-semibold text-slate-700 mb-1">Difficulty</label>
                  <input
                    className="w-full border rounded p-2 text-sm"
                    value={selected.difficulty}
                    onChange={(e) => updateEntry(selected.id, { difficulty: e.target.value })}
                  />
                </div>
                <div>
                  <label className="block text-xs font-semibold text-slate-700 mb-1">Cognitive Level</label>
                  <input
                    className="w-full border rounded p-2 text-sm"
                    value={selected.cognitive_level}
                    onChange={(e) => updateEntry(selected.id, { cognitive_level: e.target.value })}
                  />
                </div>
                <div>
                  <label className="block text-xs font-semibold text-slate-700 mb-1">Status</label>
                  <input
                    className="w-full border rounded p-2 text-sm"
                    value={selected.status}
                    onChange={(e) => updateEntry(selected.id, { status: e.target.value })}
                  />
                </div>
              </div>

              <div>
                <div>
                  <label className="block text-xs font-semibold text-slate-700 mb-1">Topics</label>
                  <div className="space-y-2">
                    {selected.topics.length === 0 && (
                      <div className="text-xs text-muted-foreground">No topics selected.</div>
                    )}
                    {selected.topics.map((topicId, idx) => {
                      const missing = topicId && !topicOptions.some((t) => t.id === topicId);
                      const selectedTopic = topicOptions.find((t) => t.id === topicId);
                      const titleText = selectedTopic?.name || (missing ? `(missing) ${topicId}` : "Select a topic");
                      const children = selectedTopic?.children || [];
                      const subtopics = selected.subtopics || [];
                      const subValue = subtopics[idx] || "";
                      return (
                      <div key={`${topicId}-${idx}`} className="flex items-center gap-2">
                        <div className="flex-1 min-w-0">
                          <select
                            className="w-full border rounded px-2 py-1.5 text-sm truncate"
                            value={topicId}
                            title={titleText}
                            onChange={(e) => {
                              const next = [...selected.topics];
                              next[idx] = e.target.value;
                              const nextSubs = [...subtopics];
                              nextSubs[idx] = "";
                              updateEntry(selected.id, { topics: next, subtopics: nextSubs });
                            }}
                          >
                            <option value="">Select a topic</option>
                            {missing && (
                              <option value={topicId}>{`(missing) ${topicId}`}</option>
                            )}
                            {topicOptions.map((t) => (
                              <option key={t.id} value={t.id}>
                                {t.name}
                              </option>
                            ))}
                          </select>
                          {children.length > 0 && (
                            <div className="mt-2">
                              <select
                                className="w-full border rounded px-2 py-1.5 text-sm truncate"
                                value={subValue}
                                title={subValue || "Select subtopic"}
                                onChange={(e) => {
                                  const nextSubs = [...subtopics];
                                  nextSubs[idx] = e.target.value;
                                  updateEntry(selected.id, { subtopics: nextSubs });
                                }}
                              >
                                <option value="">Select subtopic (optional)</option>
                                {children.map((child) => (
                                  <option key={child.id} value={child.id}>
                                    {child.name}
                                  </option>
                                ))}
                              </select>
                            </div>
                          )}
                        </div>
                        <button
                          onClick={() => {
                            const next = selected.topics.filter((_, i) => i !== idx);
                            const nextSubs = (selected.subtopics || []).filter((_, i) => i !== idx);
                            updateEntry(selected.id, { topics: next, subtopics: nextSubs });
                          }}
                          className="h-8 w-8 flex items-center justify-center border rounded hover:bg-secondary"
                          title="Remove topic"
                        >
                          <X className="w-4 h-4" />
                        </button>
                      </div>
                    );
                    })}
                    <button
                      onClick={() => {
                        const first = topicOptions[0]?.id ?? "";
                        const nextSubs = [...(selected.subtopics || []), ""];
                        updateEntry(selected.id, { topics: [...selected.topics, first], subtopics: nextSubs });
                      }}
                      className="flex items-center gap-2 px-3 py-1.5 text-sm border rounded hover:bg-secondary"
                      disabled={topicOptions.length === 0}
                    >
                      <Plus className="w-4 h-4" />
                      Add topic
                    </button>
                  </div>
                </div>
              </div>

              <div>
                <label className="block text-xs font-semibold text-slate-700 mb-1">Explanation</label>
                <textarea
                  className="w-full border rounded p-2 text-sm"
                  rows={4}
                  value={selected.explanation}
                  onChange={(e) => updateEntry(selected.id, { explanation: e.target.value })}
                />
              </div>

              <div className="flex-1" />
            </div>
          </div>

          {/* Answers Block */}
          <div className="space-y-3">
            <div className="text-sm font-semibold">Answers</div>
            {selected.answers.map((answer, idx) => (
              <div
                key={answer.id}
                className="flex items-center gap-2 border rounded px-3 py-2 bg-white"
              >
                <label className="flex items-center gap-2 text-xs">
                  <input
                    type="checkbox"
                    checked={answer.is_correct}
                    onChange={(e) =>
                      updateAnswer(selected.id, answer.id, { is_correct: e.target.checked })
                    }
                  />
                  Correct
                </label>
                <span className="w-6 h-6 flex items-center justify-center rounded-full border text-xs font-semibold text-slate-700">
                  {String.fromCharCode(65 + idx)}
                </span>
                <div className="flex-1 min-w-0 text-xs text-slate-700 truncate border rounded px-2 py-1 bg-slate-50">
                  <RichMarkdown
                    content={answer.text}
                    className="prose prose-xs max-w-none prose-p:my-0 prose-ul:my-0 prose-ol:my-0"
                    components={{
                      p({ children }: any) {
                        return <span className="inline">{children}</span>;
                      },
                      ul({ children }: any) {
                        return <span className="inline">{children}</span>;
                      },
                      ol({ children }: any) {
                        return <span className="inline">{children}</span>;
                      },
                      li({ children }: any) {
                        return <span className="inline">{children}</span>;
                      },
                    }}
                  />
                </div>
                <input
                  className="flex-1 min-w-0 border rounded px-2 py-1 text-sm"
                  value={answer.text}
                  onChange={(e) => updateAnswer(selected.id, answer.id, { text: e.target.value })}
                  placeholder="Answer text"
                />
                <input
                  className="flex-1 min-w-0 border rounded px-2 py-1 text-sm"
                  value={answer.explanation}
                  onChange={(e) => updateAnswer(selected.id, answer.id, { explanation: e.target.value })}
                  placeholder="Answer explanation"
                />
              </div>
            ))}
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="flex h-full border rounded-lg overflow-hidden">
      {renderList()}
      {renderEditor()}
    </div>
  );
}
