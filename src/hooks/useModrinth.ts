import { useEffect } from 'react';
import { onUpdateAvailable } from '@/lib/api';

export function useUpdateAvailable(callback: (available: boolean, payload: any) => void) {
  useEffect(() => {
    const unsub = onUpdateAvailable((payload: any) => {
      callback(true, payload);
    });

    return () => {
      unsub.then((u: any) => u());
    };
  }, [callback]);
}
