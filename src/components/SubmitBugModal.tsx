import React, { useState } from "react";
import { BugSeverity } from "../types";

interface SubmitBugModalProps {
  isOpen: boolean;
  isSubmitting: boolean;
  onClose: () => void;
  onSubmit: (payload: {
    title: string;
    description: string;
    steps: string;
    expectedBehavior: string;
    actualBehavior: string;
    severity: BugSeverity;
    reporterEmail: string;
    includeDiagnostics: boolean;
  }) => Promise<void>;
}

export default function SubmitBugModal({
  isOpen,
  isSubmitting,
  onClose,
  onSubmit,
}: SubmitBugModalProps) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [steps, setSteps] = useState("");
  const [expectedBehavior, setExpectedBehavior] = useState("");
  const [actualBehavior, setActualBehavior] = useState("");
  const [severity, setSeverity] = useState<BugSeverity>("medium");
  const [reporterEmail, setReporterEmail] = useState("");
  const [includeDiagnostics, setIncludeDiagnostics] = useState(true);
  const [localError, setLocalError] = useState("");

  if (!isOpen) return null;

  const resetAndClose = () => {
    setLocalError("");
    onClose();
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLocalError("");

    if (!title.trim()) {
      setLocalError("Please provide a short title.");
      return;
    }

    if (!description.trim()) {
      setLocalError("Please describe what happened.");
      return;
    }

    await onSubmit({
      title: title.trim(),
      description: description.trim(),
      steps,
      expectedBehavior,
      actualBehavior,
      severity,
      reporterEmail,
      includeDiagnostics,
    });
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
        <h2 className="text-2xl font-bold mb-2">Submit Bug</h2>
        <p className="text-gray-600 mb-6">
          Send a standardized bug report JSON to your AWS endpoint.
        </p>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Title</label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md"
              placeholder="Short summary"
              disabled={isSubmitting}
              autoFocus
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md min-h-[100px]"
              placeholder="What happened?"
              disabled={isSubmitting}
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Steps to reproduce (one per line)
            </label>
            <textarea
              value={steps}
              onChange={(e) => setSteps(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md min-h-[100px]"
              placeholder="1. ...&#10;2. ..."
              disabled={isSubmitting}
            />
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Expected</label>
              <textarea
                value={expectedBehavior}
                onChange={(e) => setExpectedBehavior(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md min-h-[80px]"
                disabled={isSubmitting}
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Actual</label>
              <textarea
                value={actualBehavior}
                onChange={(e) => setActualBehavior(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md min-h-[80px]"
                disabled={isSubmitting}
              />
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Severity</label>
              <select
                value={severity}
                onChange={(e) => setSeverity(e.target.value as BugSeverity)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md"
                disabled={isSubmitting}
              >
                <option value="low">Low</option>
                <option value="medium">Medium</option>
                <option value="high">High</option>
                <option value="critical">Critical</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Contact email (optional)
              </label>
              <input
                type="email"
                value={reporterEmail}
                onChange={(e) => setReporterEmail(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md"
                placeholder="you@example.com"
                disabled={isSubmitting}
              />
            </div>
          </div>

          <label className="flex items-center gap-2 text-sm text-gray-700">
            <input
              type="checkbox"
              checked={includeDiagnostics}
              onChange={(e) => setIncludeDiagnostics(e.target.checked)}
              disabled={isSubmitting}
            />
            Include app diagnostics context
          </label>

          {localError && (
            <div className="p-3 bg-red-50 border border-red-200 rounded-md text-sm text-red-800">
              {localError}
            </div>
          )}

          <div className="flex gap-3 pt-2">
            <button
              type="submit"
              disabled={isSubmitting}
              className="flex-1 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
            >
              {isSubmitting ? "Submitting..." : "Submit Bug"}
            </button>
            <button
              type="button"
              onClick={resetAndClose}
              disabled={isSubmitting}
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
