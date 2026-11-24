import { useAuthStore } from '@/stores/authStore';

export function useAuth() {
  const store = useAuthStore();
  return {
    accounts: store.accounts,
    activeAccount: store.activeAccount,
    loading: store.loading,
    error: store.error,
    deviceCode: store.deviceCode,
    polling: store.polling,
    loadAccounts: store.loadAccounts,
    startLogin: store.startLogin,
    pollLogin: store.pollLogin,
    logout: store.logout,
    setActive: store.setActive,
    clearError: store.clearError,
  };
}
