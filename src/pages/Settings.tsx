import { useState, useEffect } from 'react';
import { getConfig, saveConfig } from '@/lib/api';
import type { LauncherConfig } from '@/types';
import LauncherSettings from '@/components/settings/LauncherSettings';

export default function Settings() {
  const [config, setConfig] = useState<LauncherConfig>({
    default_ram_mb: 4096,
    java_path: null,
    auto_update: true,
    client_id: null,
  });
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    getConfig().then(setConfig).catch(() => {});
  }, []);

  const handleSave = async () => {
    setSaving(true);
    try {
      await saveConfig(config);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (err: any) {
      alert(err.message || 'Fehler beim Speichern');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="flex flex-col h-full gap-6">
      <div className="flex-1">
        <h2 className="text-2xl font-bold text-text">Einstellungen</h2>
        <p className="text-textMuted">Konfiguriere den Launcher nach deinen Wünschen.</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        <LauncherSettings
          config={config}
          onChange={setConfig}
          saving={saving}
          saved={saved}
          onSave={handleSave}
        />
      </div>
    </div>
  );
}
