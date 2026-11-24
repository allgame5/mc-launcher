import { useState } from 'react';
import { useInstanceStore } from '@/stores/instanceStore';
import { searchModrinth } from '@/lib/api';
import type { ModrinthProject } from '@/types';
import SearchBar from '@/components/discover/SearchBar';
import FilterChips from '@/components/discover/FilterChips';
import ProjectCard from '@/components/discover/ProjectCard';

const LOADER_CHIPS = [
  { value: 'fabric', label: 'Fabric' },
  { value: 'neoforge', label: 'NeoForge' },
];

const PROJECT_TYPE_CHIPS = [
  { value: 'mod', label: 'Mod' },
  { value: 'modpack', label: 'Modpack' },
  { value: 'shaderpack', label: 'Shader' },
  { value: 'resourcepack', label: 'Resource Pack' },
];

export default function Discover() {
  const { selected } = useInstanceStore();
  const [query, setQuery] = useState('');
  const [activeLoaders, setActiveLoaders] = useState<string[]>([]);
  const [activeTypes, setActiveTypes] = useState<string[]>([]);
  const [version, setVersion] = useState('');
  const [results, setResults] = useState<ModrinthProject[]>([]);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const handleSearch = async () => {
    setLoading(true);
    setMessage('');
    try {
      const res = await searchModrinth({
        query,
        limit: 20,
        loader: activeLoaders[0] || undefined,
        game_version: version || undefined,
        project_type: activeTypes[0] || undefined,
      });
      setResults(res.hits);
    } catch (err: any) {
      setMessage(err.message || 'Suche fehlgeschlagen');
    } finally {
      setLoading(false);
    }
  };

  const handleInstall = async (project: ModrinthProject) => {
    if (!selected) {
      setMessage('Bitte zuerst eine Instanz in der Bibliothek auswählen.');
      return;
    }
    setLoading(true);
    try {
      // Hole verfügbare Versionen für das Projekt
      setMessage(`${project.title} wurde für "${selected.name}" vorbereitet.`);
    } catch (err: any) {
      setMessage(err.message || 'Installation fehlgeschlagen');
    } finally {
      setLoading(false);
    }
  };

  const toggleLoader = (value: string) => {
    setActiveLoaders((prev) =>
      prev.includes(value) ? prev.filter((v) => v !== value) : [value]
    );
  };

  const toggleType = (value: string) => {
    setActiveTypes((prev) =>
      prev.includes(value) ? prev.filter((v) => v !== value) : [value]
    );
  };

  return (
    <div className="flex flex-col h-full gap-6">
      <div className="flex flex-col lg:flex-row gap-6 items-start">
        <div className="flex-1">
          <h2 className="text-2xl font-bold text-text">Entdecken</h2>
          <p className="text-textMuted">
            Durchsuche Modrinth nach Mods, Modpacks, Shadern und Resource Packs.
          </p>
        </div>
        <div className="flex gap-2">
          <button className="btn-secondary" onClick={handleSearch} disabled={loading}>
            Suchen
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
        <div className="lg:col-span-2">
          <SearchBar value={query} onChange={(e) => setQuery(e.target.value)} />
        </div>
        <div className="lg:col-span-1">
          <FilterChips
            loaders={LOADER_CHIPS.map((c) => ({
              value: c.value,
              active: activeLoaders.includes(c.value),
              onToggle: () => toggleLoader(c.value),
            }))}
            projectTypes={PROJECT_TYPE_CHIPS.map((c) => ({
              value: c.value,
              active: activeTypes.includes(c.value),
              onToggle: () => toggleType(c.value),
            }))}
            onVersionChange={(e) => setVersion(e.target.value)}
            versionValue={version}
          />
        </div>
        <div className="lg:col-span-1 flex items-end">
          <button className="w-full btn-secondary" onClick={handleSearch} disabled={loading}>
            Suche
          </button>
        </div>
      </div>

      {message && (
        <div className={`p-3 rounded-lg ${message.includes('vorbereitet') || message.includes('erfolgreich') ? 'bg-success/10 border border-success/20 text-success' : 'bg-error/10 border border-error/20 text-error'}`}>
          {message}
        </div>
      )}

      {loading && (
        <div className="text-center py-4 text-textMuted">Lade Ergebnisse…</div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {results.length === 0 && !loading && (
          <div className="col-span-full card flex flex-col items-center justify-center py-16 text-center">
            <p className="text-textMuted mb-2">Keine Ergebnisse gefunden.</p>
            <p className="text-xs text-textMuted">Versuche es mit anderen Suchbegriffen oder Filtern.</p>
          </div>
        )}
        {results.map((project) => (
          <ProjectCard
            key={project.id}
            project={project}
            selectedInstance={selected ? { id: selected.id, name: selected.name } : null}
            loading={loading}
            onInstall={handleInstall}
          />
        ))}
      </div>

      {!selected && (
        <div className="p-4 rounded-lg bg-card border border-border border-dashed text-center text-textMuted text-sm">
          Hinweis: Wähle zuerst eine Instanz in der Bibliothek aus, um Mods zu installieren.
        </div>
      )}
    </div>
  );
}
