import type { Instance } from '@/types';

type InstanceDetailProps = {
  instance: Instance | null;
  loading: boolean;
  onLaunch: () => void;
  onDownload: () => void;
  onDelete: () => void;
};

export default function InstanceDetail({
  instance,
  loading,
  onLaunch,
  onDownload,
  onDelete,
}: InstanceDetailProps) {
  if (!instance) return null;

  return (
    <div className="lg:w-96 space-y-4 flex flex-col">
      <div className="card flex-1">
        <h3 className="font-semibold text-text mb-4">Instanz-Details</h3>
        <div className="space-y-3 text-sm mb-6">
          <div className="flex justify-between border-b border-border pb-2">
            <span className="text-textMuted">Name</span>
            <span className="text-text font-medium">{instance.name}</span>
          </div>
          <div className="flex justify-between border-b border-border pb-2">
            <span className="text-textMuted">Version</span>
            <span className="text-text font-medium">{instance.mc_version}</span>
          </div>
          <div className="flex justify-between border-b border-border pb-2">
            <span className="text-textMuted">Modloader</span>
            <span className="text-text font-medium">{instance.mod_loader}</span>
          </div>
          <div className="flex justify-between border-b border-border pb-2">
            <span className="text-textMuted">Loader-Version</span>
            <span className="text-text font-medium">
              {instance.loader_version ?? '–'}
            </span>
          </div>
          <div className="flex justify-between border-b border-border pb-2">
            <span className="text-textMuted">RAM</span>
            <span className="text-text font-medium">{instance.ram_mb} MB</span>
          </div>
          <div className="flex justify-between border-b border-border pb-2">
            <span className="text-textMuted">Status</span>
            <span className={`text-xs font-medium ${loading ? 'text-warning' : 'text-success'}`}>
              {loading ? 'Lädt…' : 'Bereit'}
            </span>
          </div>
        </div>
        <div className="pt-4 border-t border-border flex gap-2">
          <button
            className="flex-1 btn-primary"
            onClick={onLaunch}
            disabled={loading}
          >
            Starten
          </button>
          <button
            className="flex-1 btn-secondary"
            onClick={onDownload}
            disabled={loading}
          >
            Download
          </button>
        </div>
      </div>

      <button
        onClick={onDelete}
        className="w-full py-2.5 rounded-lg border border-error/50 text-error hover:bg-error/10 transition-colors text-sm font-medium"
      >
        Instanz löschen
      </button>
    </div>
  );
}
