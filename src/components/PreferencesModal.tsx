import { useEffect, useState } from "react";
import { SubjectInfo } from "../types";

interface PreferencesModalProps {
  isOpen: boolean;
  subjects: SubjectInfo[];
  selectedSubject: string;
  selectedDifficulty: string;
  onSave: (subjectId: string, difficulty: string) => void;
  onClose: () => void;
}

export default function PreferencesModal({
  isOpen,
  subjects,
  selectedSubject,
  selectedDifficulty,
  onSave,
  onClose,
}: PreferencesModalProps) {
  const [draftSubject, setDraftSubject] = useState("");
  const [draftDifficulty, setDraftDifficulty] = useState("medium");

  useEffect(() => {
    if (!isOpen) return;
    setDraftSubject(selectedSubject);
    setDraftDifficulty(selectedDifficulty || "medium");
  }, [isOpen, selectedSubject, selectedDifficulty]);

  if (!isOpen) return null;

  const canSave = draftSubject.trim().length > 0;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
        <h2 className="text-2xl font-bold mb-2">Preferences</h2>
        <p className="text-gray-600 mb-6">
          Choose your default subject for generation and bank editing.
        </p>

        <label className="block text-sm font-medium text-gray-700 mb-2">Subject</label>
        <select
          value={draftSubject}
          onChange={(e) => setDraftSubject(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md mb-6"
          autoFocus
        >
          {subjects.map((subject) => (
            <option key={subject.id} value={subject.id}>
              {subject.name} ({subject.topic_count} topics)
            </option>
          ))}
        </select>

        <label className="block text-sm font-medium text-gray-700 mb-2">Default Difficulty</label>
        <div className="grid grid-cols-3 gap-2 mb-6">
          {["easy", "medium", "hard"].map((level) => (
            <button
              key={level}
              type="button"
              onClick={() => setDraftDifficulty(level)}
              className={`px-3 py-2 text-sm rounded-md capitalize border transition-colors ${
                draftDifficulty === level
                  ? "bg-blue-600 text-white border-blue-600"
                  : "bg-white text-gray-700 border-gray-300 hover:bg-gray-50"
              }`}
            >
              {level}
            </button>
          ))}
        </div>

        <div className="flex gap-3">
          <button
            type="button"
            onClick={() => onSave(draftSubject, draftDifficulty)}
            disabled={!canSave}
            className="flex-1 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
          >
            Save
          </button>
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
