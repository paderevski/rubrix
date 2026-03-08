interface OpenRecentModalProps {
  open: boolean;
  recentPaths: string[];
  onOpenPath: (path: string) => void;
  onClear: () => void;
  onClose: () => void;
}

function fileNameFromPath(path: string): string {
  const normalized = path.replace(/\\/g, "/");
  const parts = normalized.split("/");
  return parts[parts.length - 1] || path;
}

export default function OpenRecentModal({
  open,
  recentPaths,
  onOpenPath,
  onClear,
  onClose,
}: OpenRecentModalProps) {
  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
      <div className="w-full max-w-2xl rounded-lg bg-white shadow-xl border">
        <div className="px-5 py-4 border-b flex items-center justify-between">
          <h2 className="text-lg font-semibold text-foreground">Open Recent</h2>
          <button
            onClick={onClose}
            className="text-sm text-muted-foreground hover:text-foreground"
          >
            Close
          </button>
        </div>

        <div className="p-5 max-h-[60vh] overflow-auto">
          {recentPaths.length === 0 ? (
            <p className="text-sm text-muted-foreground">No recent files yet.</p>
          ) : (
            <div className="space-y-2">
              {recentPaths.map((path) => (
                <button
                  key={path}
                  onClick={() => onOpenPath(path)}
                  className="w-full text-left rounded-md border px-3 py-2 hover:bg-secondary/50"
                  title={path}
                >
                  <div className="text-sm font-medium text-foreground">{fileNameFromPath(path)}</div>
                  <div className="text-xs text-muted-foreground truncate">{path}</div>
                </button>
              ))}
            </div>
          )}
        </div>

        <div className="px-5 py-3 border-t flex items-center justify-between">
          <button
            onClick={onClear}
            disabled={recentPaths.length === 0}
            className="text-sm text-muted-foreground hover:text-foreground disabled:opacity-40 disabled:cursor-not-allowed"
          >
            Clear Recent
          </button>
          <button
            onClick={onClose}
            className="px-3 py-1.5 text-sm rounded-md bg-primary text-primary-foreground hover:bg-primary/90"
          >
            Done
          </button>
        </div>
      </div>
    </div>
  );
}
