import { LogOut, Bell, Github } from 'lucide-react';

export default function Header({ title }: { title: string }) {
  return (
    <header className="h-16 border-b border-border bg-dark/50 backdrop-blur px-6 flex items-center justify-between">
      <h1 className="text-xl font-semibold text-text tracking-tight">{title}</h1>
      <div className="flex items-center gap-4">
        <button className="text-textMuted hover:text-text transition-colors p-1 rounded-lg hover:bg-card/50" title="Aktualisieren">
          <Bell size={20} />
        </button>
        <button className="text-textMuted hover:text-text transition-colors p-1 rounded-lg hover:bg-card/50" title="GitHub">
          <Github size={20} />
        </button>
        <button className="text-textMuted hover:text-text transition-colors p-1 rounded-lg hover:bg-card/50" title="Abmelden">
          <LogOut size={20} />
        </button>
      </div>
    </header>
  );
}
