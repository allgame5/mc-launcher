import { Search } from 'lucide-react';

type SearchBarProps = {
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  placeholder?: string;
};

export default function SearchBar({
  value,
  onChange,
  placeholder = 'Suche nach Mods, Modpacks, Shadern…',
}: SearchBarProps) {
  return (
    <div className="relative w-full max-w-md">
      <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <Search size={18} className="text-textMuted" />
      </div>
      <input
        className="input w-full pl-10"
        value={value}
        onChange={onChange}
        placeholder={placeholder}
      />
    </div>
  );
}
