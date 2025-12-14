interface AlertModalProps {
  open: boolean;
  message: string;
  onClose: () => void;
}

export default function AlertModal({ open, message, onClose }: AlertModalProps) {
  if (!open) return null;
  // Split message into two sentences (first period followed by space or newline)
  let first = message;
  let second = "";
  const match = message.match(/^(.*?[.!?])\s+([\s\S]*)$/);
  if (match) {
    first = match[1];
    second = match[2];
  }
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-30">
      <div className="bg-white rounded-lg shadow-lg p-6 max-w-sm w-full">
        <div className="mb-4 text-base text-foreground">
          <span className="break-all">{first}</span>
          {second && <span className="block whitespace-normal mt-4"> {second}</span>}
        </div>
        <div className="flex justify-end">
          <button
            className="px-4 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90"
            onClick={onClose}
            autoFocus
          >
            OK
          </button>
        </div>
      </div>
    </div>
  );
}
