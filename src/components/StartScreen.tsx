import { FilePlus, FolderOpen } from "lucide-react";

interface StartScreenProps {
  recentPaths: string[];
  status: string;
  onNew: () => void;
  onOpen: () => void;
  onOpenRecent: (path: string) => void;
  onClearRecent: () => void;
}

const MAX_RECENT_SHOWN = 8;

function splitPath(path: string): { name: string; folder: string } {
  const normalized = path.replace(/\\/g, "/");
  const idx = normalized.lastIndexOf("/");
  if (idx < 0) return { name: path, folder: "" };
  return { name: normalized.slice(idx + 1) || path, folder: normalized.slice(0, idx) };
}

/** Launch view shown while no document is open, like the chooser in a document-based app. */
export default function StartScreen({
  recentPaths,
  status,
  onNew,
  onOpen,
  onOpenRecent,
  onClearRecent,
}: StartScreenProps) {
  const recent = recentPaths.slice(0, MAX_RECENT_SHOWN);
  const showStatus = status.trim().length > 0 && status !== "Ready";

  return (
    <div className="flex-1 min-h-0 overflow-auto bg-slate-50/60">
      <div className="mx-auto flex max-w-xl flex-col gap-8 px-6 py-14">
        <div className="text-center">
          <h2 className="text-2xl font-semibold text-foreground">Catie</h2>
          <p className="mt-1 text-sm text-muted-foreground">Test Item Engine</p>
        </div>

        <div className="grid grid-cols-2 gap-3">
          <button
            type="button"
            onClick={onNew}
            className="flex flex-col items-center gap-2 rounded-lg border bg-white px-4 py-6 shadow-sm hover:bg-secondary/50"
          >
            <FilePlus className="h-6 w-6 text-primary" />
            <span className="text-sm font-medium text-foreground">New</span>
            <span className="text-xs text-muted-foreground">Start an empty question set</span>
          </button>
          <button
            type="button"
            onClick={onOpen}
            className="flex flex-col items-center gap-2 rounded-lg border bg-white px-4 py-6 shadow-sm hover:bg-secondary/50"
          >
            <FolderOpen className="h-6 w-6 text-primary" />
            <span className="text-sm font-medium text-foreground">Open…</span>
            <span className="text-xs text-muted-foreground">Choose a saved document</span>
          </button>
        </div>

        <div>
          <div className="mb-2 flex items-center justify-between">
            <h3 className="text-sm font-semibold text-foreground">Recent</h3>
            {recent.length > 0 && (
              <button
                type="button"
                onClick={onClearRecent}
                className="text-xs text-muted-foreground hover:text-foreground"
              >
                Clear
              </button>
            )}
          </div>
          {recent.length === 0 ? (
            <p className="text-sm text-muted-foreground">No recent documents yet.</p>
          ) : (
            <ul className="divide-y rounded-lg border bg-white shadow-sm">
              {recent.map((path) => {
                const { name, folder } = splitPath(path);
                return (
                  <li key={path}>
                    <button
                      type="button"
                      onClick={() => onOpenRecent(path)}
                      className="w-full px-4 py-2.5 text-left hover:bg-secondary/50"
                      title={path}
                    >
                      <div className="truncate text-sm font-medium text-foreground">{name}</div>
                      <div className="truncate text-xs text-muted-foreground">{folder}</div>
                    </button>
                  </li>
                );
              })}
            </ul>
          )}
        </div>

        {showStatus && (
          <p className="text-center text-xs text-muted-foreground" role="status">
            {status}
          </p>
        )}
      </div>
    </div>
  );
}
