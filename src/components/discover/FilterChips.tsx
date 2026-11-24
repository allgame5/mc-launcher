import { Filter, Package, Download } from 'lucide-react';

type FilterChipsProps = {
  loaders: { value: string; active: boolean; onToggle: () => void }[];
  projectTypes: { value: string; active: boolean; onToggle: () => void }[];
  onVersionChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  versionValue: string;
};

export default function FilterChips({
  loaders,
  projectTypes,
  onVersionChange,
  versionValue,
}: FilterChipsProps) {
  return (
    <div className="flex flex-wrap gap-2">
      <div className="flex items-center gap-2">
        <Filter size={16} className="text-textMuted" />
        {loaders.map((chip) => (
          <button
            key={chip.value}
            onClick={chip.onToggle}
            className={`px-3 py-1.5 rounded-full text-xs transition-colors ${
              chip.active
                ? 'bg-accent text-white'
                : 'bg-card border border-border text-textMuted hover:bg-card/50'
            }`}
          >
            {chip.value}
          </button>
        ))}
      </div>

      <div className="flex items-center gap-2 ml-3">
        <Download size={16} className="text-textMuted" />
        <input
          className="input w-24 text-xs"
          type="text"
          value={versionValue}
          onChange={onVersionChange}
          placeholder="MC-Version"
        />
      </div>

      <div className="flex items-center gap-2 ml-3">
        {projectTypes.map((chip) => (
          <button
            key={chip.value}
            onClick={chip.onToggle}
            className={`px-3 py-1.5 rounded-full text-xs transition-colors ${
              chip.active
                ? 'bg-accent text-white'
                : 'bg-card border border-border text-textMuted hover:bg-card/50'
            }`}
          >
            <Package size={12} className="mr-1" />
            {chip.value}
          </button>
        ))}
      </div>
    </div>
  );
}
