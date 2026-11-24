import { create } from 'zustand';
import {
  authenticateMicrosoft,
  deviceCodePoll,
  getStoredAccounts,
  logoutAccount,
  setActiveAccount,
} from '@/lib/api';
import type { StoredAccount } from '@/types';
import type { DeviceCodeResponse } from '@/lib/api';

interface AuthState {
  accounts: StoredAccount[];
  activeAccount: StoredAccount | null;
  loading: boolean;
  error: string | null;
  deviceCode: DeviceCodeResponse | null;
  polling: boolean;

  loadAccounts: () => Promise<void>;
  startLogin: (clientId?: string) => Promise<void>;
  pollLogin: (deviceCode: string, clientId?: string) => Promise<void>;
  logout: (username: string) => Promise<void>;
  setActive: (username: string) => Promise<void>;
  clearError: () => void;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  accounts: [],
  activeAccount: null,
  loading: false,
  error: null,
  deviceCode: null,
  polling: false,

  loadAccounts: async () => {
    set({ loading: true, error: null });
    try {
      const accounts = await getStoredAccounts();
      const active = accounts.find((a) => a.is_active) || null;
      set({ accounts, activeAccount: active, loading: false });
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Laden der Accounts', loading: false });
    }
  },

  startLogin: async (clientId?: string) => {
    set({ loading: true, error: null, polling: true });
    try {
      const code = await authenticateMicrosoft(clientId);
      set({ deviceCode: code, loading: false, polling: true });
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Starten des Logins', loading: false, polling: false });
    }
  },

  pollLogin: async (deviceCode: string, clientId?: string) => {
    set({ polling: true, error: null });
    try {
      const account = await deviceCodePoll({ device_code: deviceCode, client_id: clientId });
      const accounts = [...get().accounts];
      const existing = accounts.findIndex((a) => a.username === account.username);
      if (existing >= 0) {
        accounts[existing] = account;
      } else {
        accounts.push(account);
      }
      const active = accounts.find((a) => a.is_active) || account;
      // Setze den neu eingeloggten als Active, falls noch keiner aktiv ist
      if (!accounts.find((a) => a.is_active)) {
        active.is_active = true;
        await get().setActive(account.username);
      }
      set({ accounts, activeAccount: active, polling: false, deviceCode: null });
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Pollen des Logins', polling: false });
    }
  },

  logout: async (username: string) => {
    set({ loading: true });
    try {
      await logoutAccount(username);
      set((state) => ({
        accounts: state.accounts.filter((a) => a.username !== username),
        activeAccount: state.activeAccount?.username === username ? null : state.activeAccount,
        loading: false,
      }));
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Abmelden', loading: false });
    }
  },

  setActive: async (username: string) => {
    try {
      await setActiveAccount(username);
      set((state) => ({
        accounts: state.accounts.map((a) => ({ ...a, is_active: a.username === username })),
        activeAccount: state.accounts.find((a) => a.username === username) || null,
      }));
    } catch (err: any) {
      set({ error: err.message || 'Fehler beim Setzen des aktiven Accounts' });
    }
  },

  clearError: () => {
    set({ error: null });
  },
}));
