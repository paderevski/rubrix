import React from "react";

type ExportKind = "md" | "qti" | "word";
type WordPreset = "teacher_key" | "student_handout";
type MdPreset = "teacher_markdown" | "student_markdown";
type QtiPreset = "lms_quiz_package" | "lms_practice_package";

interface ExportOptionsModalProps {
  isOpen: boolean;
  kind: ExportKind | null;
  activeTab: "generate" | "bank";
  isExporting: boolean;
  wordGeneratePreset: WordPreset;
  wordBankPreset: WordPreset;
  markdownPreset: MdPreset;
  qtiPreset: QtiPreset;
  onChangeWordGeneratePreset: (preset: WordPreset) => void;
  onChangeWordBankPreset: (preset: WordPreset) => void;
  onChangeMarkdownPreset: (preset: MdPreset) => void;
  onChangeQtiPreset: (preset: QtiPreset) => void;
  onCancel: () => void;
  onConfirm: () => Promise<void>;
}

const exportTitle: Record<ExportKind, string> = {
  md: "Export Markdown",
  qti: "Export QTI",
  word: "Export Word",
};

const exportDescription: Record<ExportKind, string> = {
  md: "Export generated questions as a Markdown document.",
  qti: "Export generated questions as an IMS Common Cartridge package.",
  word: "Export questions as a Word document.",
};

export default function ExportOptionsModal({
  isOpen,
  kind,
  activeTab,
  isExporting,
  wordGeneratePreset,
  wordBankPreset,
  markdownPreset,
  qtiPreset,
  onChangeWordGeneratePreset,
  onChangeWordBankPreset,
  onChangeMarkdownPreset,
  onChangeQtiPreset,
  onCancel,
  onConfirm,
}: ExportOptionsModalProps) {
  if (!isOpen || !kind) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await onConfirm();
  };

  const isWord = kind === "word";
  const isMarkdown = kind === "md";
  const isQti = kind === "qti";
  const activeWordPreset = activeTab === "generate" ? wordGeneratePreset : wordBankPreset;
  const includeExplanations = activeWordPreset === "teacher_key";

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-lg max-h-[90vh] overflow-y-auto">
        <h2 className="text-2xl font-bold mb-2">{exportTitle[kind]}</h2>
        <p className="text-gray-600 mb-6">{exportDescription[kind]}</p>

        <form onSubmit={handleSubmit} className="space-y-4">
          {isWord ? (
            <div className="space-y-3">
              <label className="block text-sm font-medium text-gray-700">Preset</label>
              <select
                value={activeWordPreset}
                onChange={(e) => {
                  const preset = e.target.value as WordPreset;
                  if (activeTab === "generate") {
                    onChangeWordGeneratePreset(preset);
                  } else {
                    onChangeWordBankPreset(preset);
                  }
                }}
                disabled={isExporting}
                className="w-full px-3 py-2 border border-gray-300 rounded-md"
              >
                <option value="teacher_key">Teacher Key (with explanations)</option>
                <option value="student_handout">Student Handout (no explanations)</option>
              </select>

              <div className="rounded-md border border-blue-200 bg-blue-50 px-3 py-2 text-sm text-blue-800">
                {activeTab === "generate"
                  ? includeExplanations
                    ? "This export will include explanations after the answer key."
                    : "This export omits explanations for student-facing handouts."
                  : includeExplanations
                  ? "This export includes explanations and distractors for bank review."
                  : "This export omits explanations and distractors for cleaner distribution."}
              </div>
            </div>
          ) : isMarkdown ? (
            <div className="space-y-3">
              <label className="block text-sm font-medium text-gray-700">Preset</label>
              <select
                value={markdownPreset}
                onChange={(e) => onChangeMarkdownPreset(e.target.value as MdPreset)}
                disabled={isExporting}
                className="w-full px-3 py-2 border border-gray-300 rounded-md"
              >
                <option value="teacher_markdown">Teacher Markdown (answers + explanations)</option>
                <option value="student_markdown">Student Markdown (questions only)</option>
              </select>
              <div className="rounded-md border border-gray-200 bg-gray-50 px-3 py-2 text-sm text-gray-700">
                {markdownPreset === "teacher_markdown"
                  ? "Teacher Markdown includes an answer key and explanations section."
                  : "Student Markdown omits answer key and explanations for handouts."}
              </div>
            </div>
          ) : isQti ? (
            <div className="space-y-3">
              <label className="block text-sm font-medium text-gray-700">Preset</label>
              <select
                value={qtiPreset}
                onChange={(e) => onChangeQtiPreset(e.target.value as QtiPreset)}
                disabled={isExporting}
                className="w-full px-3 py-2 border border-gray-300 rounded-md"
              >
                <option value="lms_quiz_package">LMS Quiz Package (shuffled choices)</option>
                <option value="lms_practice_package">LMS Practice Package (fixed choices)</option>
              </select>
              <div className="rounded-md border border-gray-200 bg-gray-50 px-3 py-2 text-sm text-gray-700">
                {qtiPreset === "lms_quiz_package"
                  ? "Quiz package randomizes answer order in the LMS renderer."
                  : "Practice package keeps answer order fixed for review consistency."}
              </div>
            </div>
          ) : (
            <div className="rounded-md border border-gray-200 bg-gray-50 px-3 py-2 text-sm text-gray-700">
              No format-specific options yet. This modal is ready for future export settings.
            </div>
          )}

          <div className="flex gap-3 pt-2">
            <button
              type="submit"
              disabled={isExporting}
              className="flex-1 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
            >
              {isExporting ? "Exporting..." : "Continue"}
            </button>
            <button
              type="button"
              onClick={onCancel}
              disabled={isExporting}
              className="px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 disabled:bg-gray-100 disabled:cursor-not-allowed transition-colors"
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
