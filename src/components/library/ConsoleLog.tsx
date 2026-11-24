import { Activity } from 'lucide-react';

type ConsoleLogProps = {
  instanceId: string | null;
  logs: string[];
  isRunning: boolean;
  onClear: () => void;
};

export default function ConsoleLog({
  instanceId,
  logs,
  isRunning,
  onClear,
}: ConsoleLogProps) {
  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center gap-2 mb-2">
        <div className={`w-2 h-2 rounded-full ${isRunning ? 'bg-success animate-pulse' : 'bg-border'}`} />
        <Activity size={16} className="text-textMuted" />
        <span className="font-semibold text-text">Konsolen-Log</span>
        {!instanceId && <span className="ml-auto text-xs text-textMuted">Wähle eine Instanz aus</span>}
      </div>
      <div className="card h-full overflow-hidden flex flex-col">
        <div className="p-3 border-b border-border bg-darker/50 flex items-center justify-between">
          <span className="text-xs text-textMuted font-mono">latest.log</span>
          <button
            onClick={onClear}
            disabled={!instanceId}
            className="text-xs text-textMuted hover:text-text disabled:opacity-50 transition-colors"
          >
            Leeren
          </button>
        </div>
        <div className="flex-1 p-4 font-mono text-xs text-textMuted overflow-y-auto">
          {logs.length === 0 ? (
            <p className="text-textMuted/70">
              {instanceId ? 'Noch keine Logs vorhanden. Starte die Instanz.' : 'Wähle eine Instanz aus.'}
            </p>
          ) : (
            <>
              {logs.map((line, i) => (
                <div key={i} className="line-break-anywhere whitespace-pre-wrap break-all">
                  {line}
                </div>
              ))}
            </>
          )}
        </div>
      </div>
    </div>
  );
}
