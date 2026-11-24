import { ChevronRight } from 'lucide-react';
import type { LauncherConfig } from '@/types';

type LauncherSettingsProps = {
  config: LauncherConfig;
  onChange: (config: LauncherConfig) => void;
  saving: boolean;
  saved: boolean;
  onSave: () => void;
};

export default function LauncherSettings({
  config,
  onChange,
  saving,
  saved,
  onSave,
}: LauncherSettingsProps) {
  return (
    <div className="flex flex-col gap-6">
      <div className="lg:col-span-2 card">
        <h3 className="font-semibold text-text mb-4">Allgemein</h3>
        <div className="space-y-5">
          <div>
            <label className="block text-sm text-textMuted mb-1">
              Standard-RAM (MB)
            </label>
            <input
              className="input w-32"
              type="number"
              value={config.default_ram_mb}
              onChange={(e) =>
                onChange({ ...config, default_ram_mb: parseInt(e.target.value) || 4096 })
              }
            />
          </div>

          <div className="flex items-center gap-3">
            <input
              id="auto_update"
              type="checkbox"
              checked={config.auto_update}
              onChange={(e) =>
                onChange({ ...config, auto_update: e.target.checked })
              }
              className="rounded bg-darker border-border text-accent focus:ring-accent"
            />
            <div className="flex-1">
              <label htmlFor="auto_update" className="text-sm text-text">
                Launcher-Auto-Updater aktivieren
              </label>
              <p className="text-xs text-textMuted mt-0.5">
                Prüft beim Start auf neue Versionen über GitHub Releases.
              </p>
            </div>
          </div>

          <div>
            <label className="block text-sm text-textMuted mb-1">
              Microsoft Client-ID
            </label>
            <input
              className="input w-full max-w-md"
              type="text"
              value={config.client_id || ''}
              placeholder="Von https://portal.azure.com"
              onChange={(e) =>
                onChange({ ...config, client_id: e.target.value || null })
              }
            />
            <p className="text-xs text-textMuted mt-1">
              Wird für den Microsoft OAuth2 Device-Code-Flow benötigt.
            </p>
          </div>
        </div>
      </div>

      <div className="card">
        <h3 className="font-semibold text-text mb-4">Java</h3>
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-textMuted mb-1">
              Benutzerdefinierter Java-Pfad
            </label>
            <div className="flex gap-2">
              <input
                className="input flex-1"
                type="text"
                value={config.java_path || ''}
                placeholder="z. B. C:\\Program Files\\Java\\jre-17\\bin\\java.exe"
                onChange={(e) =>
                  onChange({ ...config, java_path: e.target.value || null })
                }
              />
              <button
                className="btn-secondary text-xs px-3"
                onClick={() => onChange({ ...config, java_path: null })}
              >
                Reset
              </button>
            </div>
            <p className="text-xs text-textMuted mt-1">
              Leer lassen, um die automatische JRE-Erkennung (Adoptium) zu nutzen.
            </p>
          </div>
        </div>
      </div>

      <div className="flex justify-end">
        <button
          className="btn-primary"
          onClick={onSave}
          disabled={saving}
        >
          {saving ? 'Speichern…' : 'Einstellungen speichern'}
          {saved && <ChevronRight size={16} className="ml-2" />}
        </button>
      </div>
    </div>
  );
}
