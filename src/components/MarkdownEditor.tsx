import { useEffect, useRef, type ReactNode } from "react";
import { EditorState, Prec } from "@codemirror/state";
import {
  EditorView,
  keymap,
  drawSelection,
  highlightActiveLine,
  placeholder as placeholderExt,
} from "@codemirror/view";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import {
  bracketMatching,
  indentOnInput,
  syntaxHighlighting,
  defaultHighlightStyle,
  LanguageDescription,
} from "@codemirror/language";
import { markdown } from "@codemirror/lang-markdown";
import { java } from "@codemirror/lang-java";

const codeLanguages = [
  LanguageDescription.of({
    name: "java",
    alias: ["Java"],
    support: java(),
  }),
];

const editorTheme = EditorView.theme({
  "&": {
    fontSize: "0.875rem",
    backgroundColor: "white",
    border: "1px solid hsl(214.3 31.8% 91.4%)",
    borderRadius: "0.375rem",
  },
  "&.cm-focused": {
    outline: "2px solid hsl(221.2 83.2% 53.3% / 0.5)",
    outlineOffset: "1px",
  },
  ".cm-scroller": {
    fontFamily: '"SF Mono", "Fira Code", "Fira Mono", Menlo, Monaco, monospace',
    lineHeight: "1.5",
    maxHeight: "24rem",
    overflow: "auto",
  },
  ".cm-content": { padding: "0.5rem 0" },
  ".cm-line": { padding: "0 0.75rem" },
  ".cm-activeLine": { backgroundColor: "hsl(210 40% 96.1% / 0.6)" },
});

interface MarkdownEditorProps {
  initialValue: string;
  onSave: (value: string) => void;
  onCancel: () => void;
  placeholder?: string;
  /** Extra controls shown in the footer, left of Cancel/Save. */
  extraActions?: ReactNode;
}

/**
 * Markdown-aware plain-text editor (CodeMirror 6). The text is edited and saved
 * verbatim — no round-tripping through a document model.
 */
export default function MarkdownEditor({
  initialValue,
  onSave,
  onCancel,
  placeholder,
  extraActions,
}: MarkdownEditorProps) {
  const hostRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);
  // Keep latest callbacks reachable from the (once-created) keymap.
  const saveRef = useRef(onSave);
  const cancelRef = useRef(onCancel);
  saveRef.current = onSave;
  cancelRef.current = onCancel;

  useEffect(() => {
    if (!hostRef.current) return;
    const view = new EditorView({
      parent: hostRef.current,
      state: EditorState.create({
        doc: initialValue,
        extensions: [
          Prec.highest(
            keymap.of([
              {
                key: "Mod-Enter",
                run: (v) => {
                  saveRef.current(v.state.doc.toString());
                  return true;
                },
              },
              {
                key: "Escape",
                run: () => {
                  cancelRef.current();
                  return true;
                },
              },
            ])
          ),
          history(),
          drawSelection(),
          highlightActiveLine(),
          indentOnInput(),
          bracketMatching(),
          EditorState.tabSize.of(4),
          EditorView.lineWrapping,
          keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
          markdown({ codeLanguages }),
          syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
          editorTheme,
          ...(placeholder ? [placeholderExt(placeholder)] : []),
        ],
      }),
    });
    viewRef.current = view;
    view.focus();
    view.dispatch({ selection: { anchor: view.state.doc.length } });
    return () => {
      view.destroy();
      viewRef.current = null;
    };
    // initialValue is only the starting document; edits live in CodeMirror.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const save = () => {
    if (viewRef.current) onSave(viewRef.current.state.doc.toString());
  };

  return (
    <div className="space-y-2">
      <div ref={hostRef} />
      <div className="flex items-center justify-end gap-2 text-xs">
        <span className="mr-auto text-muted-foreground">⌘Enter to save · Esc to cancel</span>
        {extraActions}
        <button
          type="button"
          onClick={onCancel}
          className="px-2.5 py-1 rounded border bg-white hover:bg-secondary transition-colors"
        >
          Cancel
        </button>
        <button
          type="button"
          onClick={save}
          className="px-2.5 py-1 rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
        >
          Save
        </button>
      </div>
    </div>
  );
}
