import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { QuestionBankEntry } from "../types";
import { Loader2, Save, RotateCcw } from "lucide-react";
import ReactMarkdown from "react-markdown";
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
        <pre className="bg-slate-900 text-slate-100 rounded p-3 overflow-auto text-xs" {...rest}>
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
  return content
    .replace(/\\\(/g, "$")
    .replace(/\\\)/g, "$")
    .replace(/\\\[/g, "$$")
    .replace(/\\\]/g, "$$");
}

function RichMarkdown({ content }: { content: string }) {
  const normalized = normalizeMathDelimiters(content);
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm, remarkMath]}
      rehypePlugins={[rehypeRaw, rehypeKatex]}
      components={markdownComponents}
    >
      {normalized}
    </ReactMarkdown>
  );
}

interface BankEditorProps {
  subject: string;
}

export default function BankEditor({ subject }: BankEditorProps) {
  const [entries, setEntries] = useState<QuestionBankEntry[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dirty, setDirty] = useState(false);

  const selected = useMemo(
    () => entries.find((e) => e.id === selectedId) || null,
    [entries, selectedId]
  );

  useEffect(() => {
    if (!subject) return;
    loadBank();
  }, [subject]);

  const loadBank = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await invoke<QuestionBankEntry[]>("load_question_bank", { subject });
      setEntries(data);
      setSelectedId(data[0]?.id ?? null);
      setDirty(false);
    } catch (e: any) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const updateEntry = (id: string, patch: Partial<QuestionBankEntry>) => {
    setEntries((prev) =>
      prev.map((e) => (e.id === id ? { ...e, ...patch } : e))
    );
    setDirty(true);
  };

  const updateOption = (
    id: string,
    optionId: string,
    patch: Partial<QuestionBankEntry["options"][number]>
  ) => {
    setEntries((prev) =>
      prev.map((e) => {
        if (e.id !== id) return e;
        return {
          ...e,
          options: e.options.map((o) =>
            o.id === optionId ? { ...o, ...patch } : o
          ),
        };
      })
    );
    setDirty(true);
  };

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    try {
      await invoke("save_question_bank", { subject, entries });
      setDirty(false);
    } catch (e: any) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  };

  const handleDiscard = () => {
    loadBank();
  };

  const renderList = () => (
    <div className="w-64 border-r bg-slate-50 h-full overflow-auto">
      <div className="px-3 py-2 flex items-center justify-between border-b bg-white">
        <div className="text-sm font-semibold">Questions</div>
        {loading && <Loader2 className="w-4 h-4 animate-spin text-primary" />}
      </div>
      <div className="divide-y">
        {entries.map((q) => (
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
        {entries.length === 0 && (
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

        <div className="p-4 space-y-4">
          <div>
            <label className="block text-xs font-semibold text-slate-700 mb-1">Question Text</label>
            <textarea
              className="w-full border rounded p-2 text-sm"
              rows={6}
              value={selected.text}
              onChange={(e) => updateEntry(selected.id, { text: e.target.value })}
            />
            <div className="mt-2 text-xs font-semibold text-slate-700">Preview</div>
            <div className="prose prose-sm max-w-none border rounded p-3 bg-white">
              <RichMarkdown content={selected.text} />
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
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
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-xs font-semibold text-slate-700 mb-1">Topics (comma-separated codes)</label>
              <input
                className="w-full border rounded p-2 text-sm"
                value={selected.topics.join(", ")}
                onChange={(e) =>
                  updateEntry(selected.id, {
                    topics: e.target.value
                      .split(",")
                      .map((s) => s.trim())
                      .filter(Boolean),
                  })
                }
              />
            </div>
            <div>
              <label className="block text-xs font-semibold text-slate-700 mb-1">Skills (comma-separated)</label>
              <input
                className="w-full border rounded p-2 text-sm"
                value={selected.skills.join(", ")}
                onChange={(e) =>
                  updateEntry(selected.id, {
                    skills: e.target.value
                      .split(",")
                      .map((s) => s.trim())
                      .filter(Boolean),
                  })
                }
              />
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

          <div className="space-y-3">
            <div className="text-sm font-semibold">Answers</div>
            {selected.options.map((opt) => (
              <div key={opt.id} className="border rounded p-3 space-y-2">
                <div className="flex gap-2">
                  <input
                    className="w-24 border rounded p-2 text-sm"
                    value={opt.id}
                    onChange={(e) =>
                      updateOption(selected.id, opt.id, { id: e.target.value })
                    }
                  />
                  <label className="flex items-center gap-2 text-xs">
                    <input
                      type="checkbox"
                      checked={opt.is_correct}
                      onChange={(e) =>
                        updateOption(selected.id, opt.id, { is_correct: e.target.checked })
                      }
                    />
                    Correct
                  </label>
                </div>
                <textarea
                  className="w-full border rounded p-2 text-sm"
                  rows={2}
                  value={opt.text}
                  onChange={(e) => updateOption(selected.id, opt.id, { text: e.target.value })}
                />
                <div className="text-xs font-semibold text-slate-700">Preview</div>
                <div className="prose prose-xs max-w-none border rounded p-2 bg-white">
                  <RichMarkdown content={opt.text} />
                </div>
              </div>
            ))}
          </div>

          <div>
            <label className="block text-xs font-semibold text-slate-700 mb-1">Distractors (common errors)</label>
            <textarea
              className="w-full border rounded p-2 text-sm"
              rows={3}
              value={selected.distractors.common_errors.join("\n")}
              onChange={(e) =>
                updateEntry(selected.id, {
                  distractors: {
                    ...selected.distractors,
                    common_errors: e.target.value
                      .split("\n")
                      .map((s) => s.trim())
                      .filter(Boolean),
                  },
                })
              }
            />
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
