import { useEffect } from 'react';
import { useInstanceStore } from '@/stores/instanceStore';
import { launchMinecraft, getJavaPath } from '@/lib/api';
import { useLauncherEvents } from '@/hooks/useInstances';
import InstanceGrid from '@/components/library/InstanceGrid';
import InstanceDetail from '@/components/library/InstanceDetail';
import ConsoleLog from '@/components/library/ConsoleLog';
import ModList from '@/components/library/ModList';

export default function Library() {
  const {
    instances,
    selected,
    loading,
    error,
    addInstance,
    removeInstance,
    downloadInstanceFiles,
    setRunning,
    selectInstance,
    logs,
    isRunning,
    clearLogs,
  } = useInstanceStore();

  useEffect(() => {
    useLauncherEvents(selected?.id || '');
  }, [selected?.id]);

  const handleCreate = (data: {
    name: string;
    mc_version: string;
    mod_loader: string;
    ram_mb: number;
  }) => {
    addInstance({
      name: data.name,
      mc_version: data.mc_version,
      mod_loader: data.mod_loader,
      loader_version: null,
      ram_mb: data.ram_mb,
    });
  };

  const handleLaunch = async (instanceId: string) => {
    const instance = instances.find((i) => i.id === instanceId);
    if (!instance) return;
    const javaPath = await getJavaPath(instance.mc_version);
    setRunning(true);
    await launchMinecraft({
      instance_id: instanceId,
      java_path: javaPath || undefined,
      ram_mb: instance.ram_mb,
    });
  };

  const handleDownload = (instanceId: string) => {
    downloadInstanceFiles(instanceId);
  };

  const handleDelete = (instanceId: string) => {
    removeInstance(instanceId);
  };

  return (
    <div className="flex flex-col h-full gap-6">
      <InstanceGrid
        instances={instances}
        selected={selected}
        loading={loading}
        error={error}
        onSelect={(id) => selectInstance(id)}
        onCreate={handleCreate}
        onLaunch={handleLaunch}
        onDownload={handleDownload}
        onDelete={handleDelete}
      />

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {selected && (
          <div className="lg:col-span-4 space-y-4">
            <InstanceDetail
              instance={selected}
              loading={loading}
              onLaunch={() => handleLaunch(selected.id)}
              onDownload={() => handleDownload(selected.id)}
              onDelete={() => handleDelete(selected.id)}
            />
            <ConsoleLog
              instanceId={selected.id}
              logs={logs}
              isRunning={isRunning}
              onClear={() => clearLogs()}
            />
            <ModList
              mods={[]}
              loading={false}
              onUpdate={() => {}}
              onRefresh={() => {}}
            />
          </div>
        )}
      </div>
    </div>
  );
}
