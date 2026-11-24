import { Package, Download, ArrowRight } from 'lucide-react';
import type { ModrinthProject } from '@/types';

type ProjectCardProps = {
  project: ModrinthProject;
  selectedInstance: { id: string; name: string } | null;
  loading: boolean;
  onInstall: (project: ModrinthProject) => void;
};

export default function ProjectCard({
  project,
  selectedInstance,
  loading,
  onInstall,
}: ProjectCardProps) {
  const handleInstall = () => {
    if (!selectedInstance) return;
    onInstall(project);
  };

  return (
    <div className="card rounded-xl overflow-hidden hover:border-accent/50 transition-colors group">
      <div className="p-4">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <div className="w-10 h-10 rounded-lg bg-darker flex items-center justify-center">
              <Package size={20} className="text-accent" />
            </div>
            <span className="text-xs font-medium text-textMuted uppercase tracking-wider">
              {project.project_type}
            </span>
          </div>
          <div className="text-xs text-textMuted">
            {project.downloads.toLocaleString()} Downloads
          </div>
        </div>
        <h3 className="font-semibold text-text mb-1 group-hover:text-accent transition-colors">
          {project.title}
        </h3>
        <p className="text-sm text-textMuted line-clamp-2 mb-4">
          {project.description}
        </p>
        <div className="flex items-center justify-between mb-3">
          <span className="text-xs text-textMuted">{project.followers.toLocaleString()} Follower</span>
        </div>
        <button
          onClick={handleInstall}
          disabled={loading || !selectedInstance}
          className={`w-full py-2 rounded-lg text-sm font-medium transition-colors ${
            loading || !selectedInstance
              ? 'bg-card border border-border text-textMuted'
              : 'border border-accent/50 text-accent hover:bg-accent/10'
          }`}
        >
          {loading ? (
            <span className="flex items-center justify-center gap-2">
              <Download size={14} className="animate-spin" /> Installiere…
            </span>
          ) : !selectedInstance ? (
            'Instanz wählen'
          ) : (
            <span className="flex items-center justify-center gap-2">
              In <span className="font-medium">{selectedInstance.name}</span> installieren
              <ArrowRight size={14} />
            </span>
          )}
        </button>
      </div>
    </div>
  );
}
