interface SaveChangesModalProps {
  open: boolean;
  documentName: string;
  onSave: () => void;
  onDontSave: () => void;
  onCancel: () => void;
}

export default function SaveChangesModal({
  open,
  documentName,
  onSave,
  onDontSave,
  onCancel,
}: SaveChangesModalProps) {
  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
      <div className="w-full max-w-md rounded-lg bg-white shadow-xl border">
        <div className="px-5 py-4 border-b">
          <h2 className="text-lg font-semibold text-foreground">Save changes?</h2>
        </div>

        <div className="px-5 py-4 text-sm text-foreground">
          <p>
            Do you want to save changes to <span className="font-medium">{documentName}</span> before
            closing?
          </p>
        </div>

        <div className="px-5 py-3 border-t flex items-center justify-end gap-2">
          <button
            onClick={onCancel}
            className="px-3 py-1.5 text-sm rounded-md border hover:bg-secondary"
          >
            Cancel
          </button>
          <button
            onClick={onDontSave}
            className="px-3 py-1.5 text-sm rounded-md border hover:bg-secondary"
          >
            Don't Save
          </button>
          <button
            onClick={onSave}
            className="px-3 py-1.5 text-sm rounded-md bg-primary text-primary-foreground hover:bg-primary/90"
            autoFocus
          >
            Save
          </button>
        </div>
      </div>
    </div>
  );
}
