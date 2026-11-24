import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  Instance,
  CreateInstanceRequest,
  EditInstanceRequest,
  InstalledMod,
  ModrinthProject,
  StoredAccount,
  LauncherConfig,
} from '../types';

// ---------- Config ----------
export async function getConfig(): Promise<LauncherConfig> {
  return invoke('get_config');
}

export async function saveConfig(config: LauncherConfig): Promise<boolean> {
  return invoke('save_config', { config });
}

// ---------- Instances ----------
export async function listInstances(): Promise<Instance[]> {
  return invoke('list_instances');
}

export async function createInstance(request: CreateInstanceRequest): Promise<Instance> {
  return invoke('create_instance', { request });
}

export async function getInstance(id: string): Promise<Instance> {
  return invoke('get_instance', { instance_id: id });
}

export async function editInstance(request: EditInstanceRequest): Promise<Instance> {
  return invoke('edit_instance', { request });
}

export async function deleteInstance(id: string): Promise<boolean> {
  return invoke('delete_instance', { instance_id: id });
}

// ---------- Launcher ----------
export async function downloadInstance(instanceId: string): Promise<boolean> {
  return invoke('download_instance', { instance_id: instanceId });
}

export async function verifyInstanceFiles(instanceId: string): Promise<boolean> {
  return invoke('verify_instance_files', { instance_id: instanceId });
}

export async function launchMinecraft(payload: {
  instance_id: string;
  java_path?: string;
  ram_mb?: number;
}): Promise<void> {
  return invoke('launch_minecraft', payload);
}

export async function getJavaPath(mcVersion: string): Promise<string | null> {
  return invoke('get_java_path', { mc_version: mcVersion });
}

// ---------- Modrinth ----------
export interface SearchResult {
  hits: ModrinthProject[];
  total: number;
}

export async function searchModrinth(payload: {
  query: string;
  limit?: number;
  offset?: number;
  loader?: string;
  game_version?: string;
  project_type?: string;
}): Promise<SearchResult> {
  return invoke('search_modrinth', { request: payload });
}

export interface InstallModRequest {
  instance_id: string;
  project_id: string;
  version_id: string;
  project_type: string;
}

export async function installModrinthProject(request: InstallModRequest): Promise<{
  installed: InstalledMod[];
  skipped: string[];
}> {
  return invoke('install_modrinth_project', { request });
}

export async function listInstalledMods(instanceId: string): Promise<InstalledMod[]> {
  return invoke('list_installed_mods', { instance_id: instanceId });
}

export async function updateInstalledMods(instanceId: string): Promise<{
  installed: InstalledMod[];
  skipped: string[];
}> {
  return invoke('update_installed_mods', { instance_id: instanceId });
}

// ---------- Auth ----------
export interface DeviceCodeResponse {
  user_code: string;
  device_code: string;
  verification_uri: string;
  expires_in: number;
  interval: number;
  message: string;
}

export async function authenticateMicrosoft(clientId?: string): Promise<DeviceCodeResponse> {
  return invoke('authenticate_microsoft', { client_id: clientId });
}

export async function deviceCodePoll(payload: {
  device_code: string;
  client_id?: string;
}): Promise<StoredAccount> {
  return invoke('device_code_poll', payload);
}

export async function getStoredAccounts(): Promise<StoredAccount[]> {
  return invoke('get_stored_accounts');
}

export async function logoutAccount(username: string): Promise<boolean> {
  return invoke('logout_account', { username });
}

export async function setActiveAccount(username: string): Promise<boolean> {
  return invoke('set_active_account', { username });
}

// ---------- Updater ----------
export async function checkForUpdates(): Promise<boolean> {
  return invoke('check_for_updates');
}

// ---------- Event listeners ----------
export function onLauncherLog(callback: (line: string) => void) {
  return listen<any>('launcher:log', (event) => {
    callback(event.payload.line ?? '');
  });
}

export function onLauncherStarted(callback: (pid: number) => void) {
  return listen<any>('launcher:started', (event) => {
    callback(event.payload.pid ?? 0);
  });
}

export function onLauncherStopped(callback: (code: number | null) => void) {
  return listen<any>('launcher:stopped', (event) => {
    callback(event.payload.exit_code ?? null);
  });
}

export function onLauncherError(callback: (message: string) => void) {
  return listen<any>('launcher:error', (event) => {
    callback(event.payload.message ?? '');
  });
}

export function onUpdateAvailable(callback: (payload: any) => void) {
  return listen('updater:available', (event) => {
    callback(event.payload);
  });
}
