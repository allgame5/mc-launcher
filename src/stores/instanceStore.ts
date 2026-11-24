import { create } from 'zustand';
import {
  listInstances,
  createInstance,
  editInstance,
  deleteInstance,
  getInstance,
  downloadInstance,
  getJavaPath,
} from '@/lib/api';
import type { Instance, CreateInstanceRequest, EditInstanceRequest } from '@/types';

interface InstanceState {
  instances: Instance[];
  selected: Instance | null;
  loading: boolean;
  error: string | null;
  logs: string[];
  isRunning: boolean;

  fetchInstances: () => Promise<void>;
  selectInstance: (id: string) => Promise<void>;
  addInstance: (request: CreateInstanceRequest) => Promise<void>;
  updateInstance: (request: EditInstanceRequest) => Promise<void>;
  removeInstance: (id: string) => Promise<void>;
  downloadInstanceFiles: (id: string) => Promise<void>;
  checkJavaPath: (mcVersion: string) => Promise<string | null>;
  addLog: (line: string) => void;
  clearLogs: () => void;
  setRunning: (running: boolean) => void;
}

export const useInstanceStore = create<InstanceState>((set) => ({
  instances: [],
  selected: null,
  loading: false,
  error: null,
  logs: [],
  isRunning: false,

  fetchInstances: async () => {
    set({ loading: true, error: null });
    try {
      const instances = await listInstances();
      set({ instances, loading: false });
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Laden der Instanzen', loading: false });
    }
  },

  selectInstance: async (id: string) => {
    try {
      const instance = await getInstance(id);
      set({ selected: instance });
    } catch (err: any) {
      set({ error: err.message || 'Instanz nicht gefunden' });
    }
  },

  addInstance: async (request: CreateInstanceRequest) => {
    set({ loading: true, error: null });
    try {
      const newInstance = await createInstance(request);
      set((state) => ({
        instances: [newInstance, ...state.instances],
        selected: newInstance,
        loading: false,
      }));
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Erstellen der Instanz', loading: false });
    }
  },

  updateInstance: async (request: EditInstanceRequest) => {
    set({ loading: true, error: null });
    try {
      const updated = await editInstance(request);
      set((state) => ({
        instances: state.instances.map((i) => (i.id === updated.id ? updated : i)),
        selected: state.selected?.id === updated.id ? updated : state.selected,
        loading: false,
      }));
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Speichern', loading: false });
    }
  },

  removeInstance: async (id: string) => {
    set({ loading: true, error: null });
    try {
      await deleteInstance(id);
      set((state) => ({
        instances: state.instances.filter((i) => i.id !== id),
        selected: state.selected?.id === id ? null : state.selected,
        loading: false,
      }));
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Löschen', loading: false });
    }
  },

  downloadInstanceFiles: async (id: string) => {
    set({ loading: true, error: null });
    try {
      await downloadInstance(id);
      set({ loading: false });
    } catch (err: any) {
      set({ error: err.message || 'Download fehlgeschlagen', loading: false });
    }
  },

  checkJavaPath: async (mcVersion: string) => {
    try {
      return await getJavaPath(mcVersion);
    } catch {
      return null;
    }
  },

  addLog: (line: string) => {
    set((state) => ({
      logs: [...state.logs, line].slice(-500),
    }));
  },

  clearLogs: () => {
    set({ logs: [] });
  },

  setRunning: (running: boolean) => {
    set({ isRunning: running });
  },
}));
