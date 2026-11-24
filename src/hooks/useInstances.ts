import { useEffect } from 'react';
import { useInstanceStore } from '@/stores/instanceStore';
import {
  onLauncherLog,
  onLauncherStarted,
  onLauncherStopped,
  onLauncherError,
} from '@/lib/api';

export function useLauncherEvents(instanceId: string) {
  const { addLog, setRunning, clearLogs } = useInstanceStore();
  const unsubs: (() => void)[] = [];

  useEffect(() => {
    clearLogs();

    onLauncherLog((line: string) => {
      addLog(line);
    }).then((unsub) => unsubs.push(unsub));

    onLauncherStarted(() => {
      setRunning(true);
    }).then((unsub) => unsubs.push(unsub));

    onLauncherStopped(() => {
      setRunning(false);
    }).then((unsub) => unsubs.push(unsub));

    onLauncherError((msg: string) => {
      addLog(`[ERROR] ${msg}`);
      setRunning(false);
    }).then((unsub) => unsubs.push(unsub));

    return () => {
      unsubs.forEach((unsub) => unsub());
      unsubs.length = 0;
    };
  }, [instanceId, addLog, setRunning, clearLogs]);
}
