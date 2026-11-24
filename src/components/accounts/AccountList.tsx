import { User, CheckCircle, LogOut, Loader2 } from 'lucide-react';
import type { StoredAccount } from '@/types';

type AccountListProps = {
  accounts: StoredAccount[];
  activeAccount: StoredAccount | null;
  loading: boolean;
  onSetActive: (username: string) => void;
  onLogout: (username: string) => void;
};

export default function AccountList({
  accounts,
  activeAccount,
  loading,
  onSetActive,
  onLogout,
}: AccountListProps) {
  return (
    <div className="card">
      <div className="p-4 border-b border-border">
        <h3 className="font-semibold text-text">Verknüpfte Accounts</h3>
      </div>

      {accounts.length === 0 ? (
        <div className="p-8 text-center text-textMuted">
          <User size={48} className="mx-auto mb-3 opacity-50" />
          <p className="text-sm">Noch kein Konto verknüpft.</p>
        </div>
      ) : (
        <div className="divide-y divide-border">
          {accounts.map((acc) => (
            <div
              key={acc.username}
              className={`p-4 flex items-center justify-between group hover:bg-card/50 transition-colors ${
                activeAccount?.username === acc.username
                  ? 'bg-accent/5 border-l-2 border-l-accent'
                  : ''
              }`}
            >
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center text-accent">
                  <User size={20} />
                </div>
                <div>
                  <h4 className="font-medium text-text">{acc.username}</h4>
                  <p className="text-xs text-textMuted">
                    {acc.xuid ? `XUID: ${acc.xuid}` : 'XUID: Unbekannt'} •{' '}
                    {acc.is_active ? 'Aktiv' : 'Inaktiv'}
                  </p>
                </div>
              </div>
              <div className="flex items-center gap-2">
                {!acc.is_active && (
                  <button
                    onClick={() => onSetActive(acc.username)}
                    className="text-xs text-accent hover:text-accentHover underline"
                  >
                    Aktivieren
                  </button>
                )}
                {acc.is_active && (
                  <span className="flex items-center gap-1 text-xs bg-success/20 text-success px-2 py-0.5 rounded">
                    <CheckCircle size={12} />
                    Aktiv
                  </span>
                )}
                <button
                  onClick={() => onLogout(acc.username)}
                  className="text-xs text-error hover:text-error underline ml-2 flex items-center gap-1"
                >
                  <LogOut size={14} />
                  Abmelden
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      {loading && (
        <div className="p-4 text-center text-textMuted">
          <Loader2 size={20} className="animate-spin" />
        </div>
      )}
    </div>
  );
}
