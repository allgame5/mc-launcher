import { useState } from 'react';
import { X, ChevronRight } from 'lucide-react';
import type { View } from '@/App';

type NavItem = {
  id: View;
  label: string;
  icon: React.ElementType;
};

type SidebarProps = {
  items: NavItem[];
  active: View;
  onSelect: (id: View) => void;
};

export default function Sidebar({ items, active, onSelect }: SidebarProps) {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside className={`w-64 lg:w-64 border-r border-border bg-darker/50 flex flex-col ${collapsed ? 'w-16' : ''}`}>
      <div className="h-16 border-b border-border flex items-center justify-center px-4">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 rounded bg-accent flex items-center justify-center">
            <span className="text-white text-xs font-bold">M</span>
          </div>
          {!collapsed && (
            <span className="font-bold text-xl text-text tracking-tighter">
              MC Launcher
            </span>
          )}
        </div>
        <button
          className="lg:hidden p-2 text-textMuted hover:text-text"
          onClick={() => setCollapsed(!collapsed)}
        >
          {collapsed ? <ChevronRight size={20} /> : <X size={20} />}
        </button>
      </div>

      <nav className="flex-1 px-2 py-4 overflow-y-auto">
        {items.map((item) => {
          const Icon = item.icon;
          const isActive = active === item.id;
          return (
            <button
              key={item.id}
              onClick={() => onSelect(item.id)}
              className={`sidebar-item ${isActive ? 'sidebar-item.active' : ''} ${collapsed ? 'justify-center px-2' : ''}`}
              title={collapsed ? item.label : undefined}
            >
              <Icon size={20} />
              {!collapsed && <span>{item.label}</span>}
            </button>
          );
        })}
      </nav>

      <div className="p-4 border-t border-border">
        <div className="text-xs font-semibold text-textMuted uppercase tracking-wider mb-3">
          Version 0.1.0
        </div>
        <button className="sidebar-item">
          <span className="text-xs text-textMuted">Feedback</span>
        </button>
      </div>
    </aside>
  );
}
