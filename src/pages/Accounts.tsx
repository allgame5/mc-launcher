import { useEffect } from 'react';
import { Plus } from 'lucide-react';
import { useAuth } from '@/hooks/useAuth';
import DeviceCodePanel from '@/components/accounts/DeviceCodePanel';
import AccountList from '@/components/accounts/AccountList';

export default function Accounts() {
  const {
    accounts,
    activeAccount,
    loading,
    error,
    deviceCode,
    polling,
    loadAccounts,
    startLogin,
    pollLogin,
    logout,
    setActive,
    clearError,
  } = useAuth();

  useEffect(() => {
    loadAccounts();
  }, []);

  return (
    <div className="flex flex-col h-full gap-6">
      <div className="flex flex-col lg:flex-row gap-6 items-start">
        <div className="flex-1">
          <h2 className="text-2xl font-bold text-text">Microsoft-Accounts</h2>
          <p className="text-textMuted">
            Verbinde dein Microsoft-Konto, um Minecraft zu starten und skins zu nutzen.
          </p>
        </div>
        <button
          className="btn-primary"
          onClick={() => startLogin()}
          disabled={loading || polling}
        >
          {polling ? 'Warte auf Bestätigung…' : <Plus size={18} className="mr-2" />} Konto hinzufügen
        </button>
      </div>

      {error && (
        <div className="p-3 rounded-lg bg-error/10 border border-error/20 text-error text-sm flex items-center gap-2">
          {error}
          <button
            onClick={clearError}
            className="ml-2 text-xs underline hover:text-error"
          >
            Schließen
          </button>
        </div>
      )}

      {deviceCode && polling && (
        <DeviceCodePanel
          deviceCode={{
            user_code: deviceCode.user_code,
            device_code: deviceCode.device_code,
            expires_in: deviceCode.expires_in,
          }}
          polling={polling}
          onPoll={() => pollLogin(deviceCode.device_code)}
          onClose={() => {
            // Device-Code schließen
          }}
        />
      )}

      <AccountList
        accounts={accounts}
        activeAccount={activeAccount}
        loading={loading}
        onSetActive={setActive}
        onLogout={logout}
      />
    </div>
  );
}
