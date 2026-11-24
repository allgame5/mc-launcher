import { Play, Save, Trash2, Monitor } from 'lucide-react';
import type { Instance } from '@/types';

type InstanceCardProps = {
  instance: Instance;
  isSelected: boolean;
  onSelect: () => void;
  onLaunch: () => void;
  onDownload: () => void;
  onDelete: () => void;
};

export default function InstanceCard({
  instance,
  isSelected,
  onSelect,
  onLaunch,
  onDownload,
  onDelete,
}: InstanceCardProps) {
  return (
    <div
      className={`card cursor-pointer group border rounded-xl p-4 transition-all hover:border-accent/50 ${isSelected ? 'border-accent bg-accent/5' : 'border-border hover:bg-card/50'}`}
      onClick={onSelect}
    >
      <div className="flex flex-col h-full">
        <div className="flex items-start justify-between mb-3">
          <div className="w-10 h-10 rounded-lg bg-darker flex items-center justify-center text-accent">
            <Monitor size={20} />
          </div>
          <div className="flex items-center gap-1">
            {instance.mod_loader === 'fabric' && (
              <span className="text-xs bg-accent/10 text-accent px-2 py-0.5 rounded">Fabric</span>
            )}
            {instance.mod_loader === 'neoforge' && (
              <span className="text-xs bg-accent/10 text-accent px-2 py-0.5 rounded">NeoForge</span>
            )}
            {instance.mod_loader === 'vanilla' && (
              <span className="text-xs bg-text/10 text-text px-2 py-0.5 rounded">Vanilla</span>
            )}
          </div>
        </div>
        <h3 className="font-semibold text-text mb-1 truncate">{instance.name}</h3>
        <p className="text-sm text-textMuted mb-3">
          {instance.mc_version} • {instance.ram_mb} MB RAM
        </p>
        <div className="flex items-center justify-between mt-auto gap-2">
          <button
            onClick={(e) => {
              e.stopPropagation();
              onLaunch();
            }}
            className="flex-1 text-xs bg-accent text-white px-3 py-1.5 rounded hover:bg-accentHover transition-colors"
          >
            <Play size={14} className="mr-1" /> Starten
          </button>
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDownload();
            }}
            className="text-xs text-textMuted hover:text-text p-1.5 rounded hover:bg-card/50 transition-colors"
            title="Dateien herunterladen"
          >
            <Save size={14} />
          </button>
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDelete();
            }}
            className="text-xs text-error hover:text-error p-1.5 rounded hover:bg-error/10 transition-colors"
            title="Löschen"
          >
            <Trash2 size={14} />
          </button>
        </div>
      </div>
    </div>
  );
}
