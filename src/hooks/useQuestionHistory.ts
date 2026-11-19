import { useCallback, useRef, useState } from "react";
import { EditorView } from "@codemirror/view";
import { undo as cmUndo, redo as cmRedo } from "@codemirror/commands";
import type { Question } from "../types";

export interface QuestionSnapshot {
  questions: Question[];
  rawTextByQuestionId: Record<string, string>;
}

interface HistoryEntry {
  label: string;
  snapshot: QuestionSnapshot;
}

interface HistoryStacks {
  undo: HistoryEntry[];
  redo: HistoryEntry[];
}

const EMPTY: HistoryStacks = { undo: [], redo: [] };

// ---- Pure stack operations -------------------------------------------------

export function pushEntry(stacks: HistoryStacks, entry: HistoryEntry, limit: number): HistoryStacks {
  const undo = [...stacks.undo, entry];
  return { undo: undo.length > limit ? undo.slice(undo.length - limit) : undo, redo: [] };
}

/** Move the top entry of `from` to `to`, replacing its snapshot with `current`. */
function step(
  stacks: HistoryStacks,
  direction: "undo" | "redo",
  current: QuestionSnapshot
): { stacks: HistoryStacks; entry: HistoryEntry } | null {
  const from = direction === "undo" ? stacks.undo : stacks.redo;
  const to = direction === "undo" ? stacks.redo : stacks.undo;
  const entry = from[from.length - 1];
  if (!entry) return null;
  const nextFrom = from.slice(0, -1);
  const nextTo = [...to, { label: entry.label, snapshot: current }];
  return {
    entry,
    stacks: direction === "undo" ? { undo: nextFrom, redo: nextTo } : { undo: nextTo, redo: nextFrom },
  };
}

// ---- Hook --------------------------------------------------------------------

interface UseQuestionHistoryOptions {
  /** Current question state (read at call time, so pass a ref-backed getter). */
  getCurrent: () => QuestionSnapshot;
  /** Apply a snapshot to frontend + backend state. Throw to abort the undo/redo. */
  restore: (snapshot: QuestionSnapshot) => Promise<void>;
  limit?: number;
}

export function useQuestionHistory({ getCurrent, restore, limit = 50 }: UseQuestionHistoryOptions) {
  const [stacks, setStacksState] = useState<HistoryStacks>(EMPTY);
  const stacksRef = useRef<HistoryStacks>(EMPTY);
  const busyRef = useRef(false);

  const setStacks = (next: HistoryStacks) => {
    stacksRef.current = next;
    setStacksState(next);
  };

  /** Record the state *before* a change. Call right before applying the change. */
  const record = useCallback(
    (label: string) => {
      setStacks(pushEntry(stacksRef.current, { label, snapshot: getCurrent() }, limit));
    },
    [getCurrent, limit]
  );

  const clear = useCallback(() => setStacks(EMPTY), []);

  const move = useCallback(
    async (direction: "undo" | "redo"): Promise<string | null> => {
      if (busyRef.current) return null;
      const before = stacksRef.current;
      const result = step(before, direction, getCurrent());
      if (!result) return null;
      busyRef.current = true;
      try {
        setStacks(result.stacks);
        await restore(result.entry.snapshot);
        return result.entry.label;
      } catch (err) {
        setStacks(before);
        throw err;
      } finally {
        busyRef.current = false;
      }
    },
    [getCurrent, restore]
  );

  return {
    record,
    clear,
    undo: () => move("undo"),
    redo: () => move("redo"),
    undoLabel: stacks.undo[stacks.undo.length - 1]?.label ?? null,
    redoLabel: stacks.redo[stacks.redo.length - 1]?.label ?? null,
  };
}

// ---- Routing ⌘Z to whatever has focus ---------------------------------------

/**
 * If a text editor has focus, apply undo/redo to it and return true.
 * Otherwise return false so the caller can apply app-level undo.
 */
export function routeTextUndo(direction: "undo" | "redo"): boolean {
  const el = document.activeElement as HTMLElement | null;
  if (!el) return false;

  const cmRoot = el.closest(".cm-editor") as HTMLElement | null;
  if (cmRoot) {
    const view = EditorView.findFromDOM(cmRoot);
    if (view) {
      (direction === "undo" ? cmUndo : cmRedo)(view);
      return true;
    }
  }

  const isTextField =
    el instanceof HTMLTextAreaElement ||
    (el instanceof HTMLInputElement && !["checkbox", "radio", "button", "submit"].includes(el.type)) ||
    el.isContentEditable;
  if (isTextField) {
    document.execCommand(direction);
    return true;
  }
  return false;
}
