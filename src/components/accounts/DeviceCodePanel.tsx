import { Loader2, LogIn } from 'lucide-react';

type DeviceCodePanelProps = {
  deviceCode: {
    user_code: string;
    device_code: string;
    expires_in: number;
  };
  polling: boolean;
  onPoll: () => void;
  onClose: () => void;
};

export default function DeviceCodePanel({
  deviceCode,
  polling,
  onPoll,
  onClose,
}: DeviceCodePanelProps) {
  return (
    <div className="card p-6 border-accent/50">
      <div className="flex items-center gap-2 mb-4">
        <div className="w-2 h-2 rounded-full bg-accent animate-pulse" />
        <h3 className="font-semibold text-text">Geräte-Code-Login</h3>
      </div>

      <div className="flex flex-col items-center gap-4 mb-6">
        <div className="text-xs text-textMuted">
          Code auf dieser Seite eingeben:
        </div>
        <a
          href="https://microsoft.com/link"
          target="_blank"
          rel="noopener noreferrer"
          className="text-accent underline text-sm hover:text-accentHover transition-colors"
        >
          microsoft.com/link
        </a>
        <div className="bg-darker p-4 rounded-lg border border-border text-2xl font-mono text-text">
          {deviceCode.user_code}
        </div>
        <p className="text-sm text-textMuted">
          Der Code läuft in {Math.ceil(deviceCode.expires_in / 60)} Minuten ab.
        </p>
      </div>

      <div className="flex gap-2">
        <button
          onClick={onPoll}
          disabled={polling}
          className={`flex-1 py-2 rounded-lg text-sm font-medium transition-colors ${
            polling
              ? 'bg-card border border-border text-textMuted'
              : 'bg-accent text-white hover:bg-accentHover'
          }`}
        >
          {polling ? (
            <Loader2 size={16} className="mr-2 animate-spin" />
          ) : (
            <LogIn size={16} className="mr-2" />
          )}
          {polling ? 'Warte auf Bestätigung…' : 'Code überprüfen'}
        </button>
        <button
          onClick={onClose}
          className="text-xs text-error hover:text-error underline"
        >
          Abbrechen
        </button>
      </div>
    </div>
  );
}
