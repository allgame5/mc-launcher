import { Package, RefreshCw, AlertCircle } from 'lucide-react';
import type { InstalledMod } from '@/types';

type ModListProps = {
  mods: InstalledMod[];
  loading: boolean;
  onUpdate: () => void;
  onRefresh: () => void;
};

export default function ModList({
  mods,
  loading,
  onUpdate,
  onRefresh,
}: ModListProps) {
  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <Package size={16} className="text-accent" />
          <span className="font-semibold text-text">Installierte Mods</span>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={onUpdate}
            disabled={loading}
            className="text-xs text-accent hover:text-accentHover underline disabled:opacity-50"
          >
            Alle aktualisieren
          </button>
          <button
            onClick={onRefresh}
            className="text-xs text-textMuted hover:text-text transition-colors p-1 rounded hover:bg-card/50"
          >
            <RefreshCw size={16} />
          </button>
        </div>
      </div>

      {loading && (
        <div className="h-32 flex items-center justify-center text-textMuted text-sm">
          Lade Mod-Status…
        </div>
      )}

      {!loading && mods.length === 0 && (
        <div className="h-32 flex items-center justify-center text-textMuted text-sm">
          Noch keine Mods installiert.
        </div>
      )}

      {!loading && mods.length > 0 && (
        <div className="overflow-y-auto">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="border-b border-border text-xs text-textMuted">
                <th className="pb-2 pr-4">Mod</th>
                <th className="pb-2 pr-4">Version</th>
                <th className="pb-2">Status</th>
              </tr>
            </thead>
            <tbody>
              {mods.map((mod) => (
                <tr
                  key={mod.version_id}
                  className="border-b border-border/50 hover:bg-card/50 transition-colors"
                >
                  <td className="py-2 pr-4 text-sm text-text">
                    {mod.project_title}
                  </td>
                  <td className="py-2 pr-4 text-sm text-textMuted font-mono">
                    {mod.version_number}
                  </td>
                  <td className="py-2 text-xs">
                    <span className="bg-success/10 text-success px-2 py-0.5 rounded">
                      <AlertCircle size={10} className="mr-1" />
                      Aktuell
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
