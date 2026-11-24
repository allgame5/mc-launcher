import { useState } from 'react';
import { Plus, AlertCircle } from 'lucide-react';
import type { Instance } from '@/types';
import InstanceCard from '@/components/library/InstanceCard';

type InstanceGridProps = {
  instances: Instance[];
  selected: Instance | null;
  loading: boolean;
  error: string | null;
  onSelect: (id: string) => void;
  onCreate: (data: {
    name: string;
    mc_version: string;
    mod_loader: string;
    ram_mb: number;
  }) => void;
  onLaunch: (instanceId: string) => void;
  onDownload: (instanceId: string) => void;
  onDelete: (instanceId: string) => void;
};

export default function InstanceGrid({
  instances,
  selected,
  loading,
  error,
  onSelect,
  onCreate,
  onLaunch,
  onDownload,
  onDelete,
}: InstanceGridProps) {
  const [form, setForm] = useState({
    name: '',
    mc_version: '1.20.1',
    mod_loader: 'vanilla',
    ram_mb: 4096,
  });

  const handleCreate = () => {
    if (!form.name.trim()) return;
    onCreate({
      name: form.name,
      mc_version: form.mc_version,
      mod_loader: form.mod_loader,
      ram_mb: form.ram_mb,
    });
    setForm({ name: '', mc_version: '1.20.1', mod_loader: 'vanilla', ram_mb: 4096 });
  };

  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col sm:flex-row sm:items-end gap-4">
        <div className="flex-1">
          <h2 className="text-2xl font-bold text-text">Instanzen</h2>
          <p className="text-textMuted">
            Verwalte deine Minecraft-Instanzen und Modpacks.
          </p>
        </div>
        <div className="flex gap-2 w-full sm:w-auto">
          <input
            className="input w-full sm:w-auto max-w-xs"
            placeholder="Instanzname"
            value={form.name}
            onChange={(e) => setForm({ ...form, name: e.target.value })}
          />
          <select
            className="input w-auto"
            value={form.mc_version}
            onChange={(e) => setForm({ ...form, mc_version: e.target.value })}
          >
            <option>1.20.1</option>
            <option>1.19.4</option>
            <option>1.18.2</option>
            <option>1.16.5</option>
          </select>
          <select
            className="input w-auto"
            value={form.mod_loader}
            onChange={(e) => setForm({ ...form, mod_loader: e.target.value })}
          >
            <option>vanilla</option>
            <option>fabric</option>
            <option>neoforge</option>
          </select>
          <input
            className="input w-24"
            type="number"
            value={form.ram_mb}
            onChange={(e) => setForm({ ...form, ram_mb: parseInt(e.target.value) || 4096 })}
          />
          <button
            className="btn-primary"
            onClick={handleCreate}
            disabled={loading}
          >
            <Plus size={18} /> Erstellen
          </button>
        </div>
      </div>

      {error && (
        <div className="mb-4 p-3 rounded-lg bg-error/10 border border-error/20 text-error text-sm flex items-center gap-2">
          <AlertCircle size={16} /> {error}
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {instances.map((inst) => (
          <InstanceCard
            key={inst.id}
            instance={inst}
            isSelected={selected?.id === inst.id}
            onSelect={() => onSelect(inst.id)}
            onLaunch={() => onLaunch(inst.id)}
            onDownload={() => onDownload(inst.id)}
            onDelete={() => onDelete(inst.id)}
          />
        ))}
        {instances.length === 0 && (
          <div className="col-span-full card flex flex-col items-center justify-center py-16 text-center">
            <div className="w-16 h-16 rounded-full bg-darker flex items-center justify-center text-textMuted mb-4">
              <Plus size={32} />
            </div>
            <p className="text-textMuted mb-2">Keine Instanzen vorhanden.</p>
            <p className="text-xs text-textMuted">Erstelle deine erste Instanz oben.</p>
          </div>
        )}
      </div>
    </div>
  );
}
