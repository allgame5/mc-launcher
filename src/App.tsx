import { useState } from 'react';
import Sidebar from './components/layout/Sidebar';
import Header from './components/layout/Header';
import Library from './pages/Library';
import Discover from './pages/Discover';
import Accounts from './pages/Accounts';
import SettingsPage from './pages/Settings';

export type View = 'library' | 'discover' | 'accounts' | 'settings';

export default function App() {
  const [view, setView] = useState<View>('library');

  const navItems: { id: View; label: string; icon: React.ElementType }[] = [
    { id: 'library', label: 'Bibliothek', icon: LayoutIcon },
    { id: 'discover', label: 'Entdecken', icon: SparklesIcon },
    { id: 'accounts', label: 'Accounts', icon: UsersIcon },
    { id: 'settings', label: 'Einstellungen', icon: SettingsIcon },
  ];

  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar items={navItems} active={view} onSelect={setView} />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Header title={navItems.find((i) => i.id === view)?.label ?? ''} />
        <main className="flex-1 overflow-y-auto p-6">
          {view === 'library' && <Library />}
          {view === 'discover' && <Discover />}
          {view === 'accounts' && <Accounts />}
          {view === 'settings' && <SettingsPage />}
        </main>
      </div>
    </div>
  );
}

const LayoutIcon = (props: { size?: number }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={props.size ?? 20}
    height={props.size ?? 20}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <rect width="7" height="7" x="3" y="3" rx="1" />
    <rect width="7" height="7" x="14" y="3" rx="1" />
    <rect width="7" height="7" x="14" y="14" rx="1" />
    <rect width="7" height="7" x="3" y="14" rx="1" />
  </svg>
);
const SparklesIcon = (props: { size?: number }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={props.size ?? 20}
    height={props.size ?? 20}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M5 3 4 5 2 5l1-2 2 2L3 5l-1 2 2-2zM12 6l1 2 2 2 2-2 1-2-1-2-2-2-2 2-1 2zM18 18l1-2 2-2 2 2 1 2-1 2-2 2-2-2 1-2zM3 18l1 2 2 2 2-2 1-2-1-2-2-2-2 2-1 2z" />
  </svg>
);
const UsersIcon = (props: { size?: number }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={props.size ?? 20}
    height={props.size ?? 20}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
    <circle cx="9" cy="7" r="4" />
    <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
    <path d="M16 3.13a4 4 0 0 1 0 7.75" />
  </svg>
);
const SettingsIcon = (props: { size?: number }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={props.size ?? 20}
    height={props.size ?? 20}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <circle cx="12" cy="12" r="3" />
    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1-1.51H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33 1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2h.09a1.65 1.65 0 0 0 1.82-.33a1.65 1.65 0 0 0 1 1.51V9a1.65 1.65 0 0 0 1.82.33l.06.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1-1.51h.09a2 2 0 0 1 2 2 2 2 0 0 1-2 2H9a1.65 1.65 0 0 0-1 1.51" />
  </svg>
);
