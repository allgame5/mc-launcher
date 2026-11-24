export type ModLoader = 'vanilla' | 'fabric' | 'neoforge';

export interface Instance {
  id: string;
  name: string;
  mc_version: string;
  mod_loader: ModLoader;
  loader_version: string | null;
  ram_mb: number;
  icon: string | null;
  last_played: string | null;
  created_at: string;
}

export interface CreateInstanceRequest {
  name: string;
  mc_version: string;
  mod_loader: string;
  loader_version: string | null;
  ram_mb?: number;
}

export interface EditInstanceRequest {
  id: string;
  name?: string;
  ram_mb?: number;
  icon?: string;
}

export interface InstalledMod {
  project_id: string;
  version_id: string;
  version_number: string;
  project_title: string;
  file_name: string;
  sha512: string;
  installed_path: string;
  installed_at: string;
}

export interface ModrinthProject {
  id: string;
  title: string;
  slug: string;
  description: string;
  project_type: string;
  icon_url: string | null;
  downloads: number;
  followers: number;
  categories: string[];
}

export interface ModrinthVersion {
  id: string;
  project_id: string;
  version_number: string;
  game_versions: string[];
  loaders: string[];
  files: Array<{
    name: string;
    url: string;
    filename: string;
    size: number;
    hashes: { sha512: string; sha1?: string };
    primary: boolean;
  }>;
  dependencies: Array<{
    project_id: string;
    version_id: string;
    dependency_type: string;
  }>;
}

export interface StoredAccount {
  username: string;
  access_token: string;
  refresh_token: string;
  expires_at: number | null;
  is_active: boolean;
  xuid: string | null;
}

export interface LauncherConfig {
  default_ram_mb: number;
  java_path: string | null;
  auto_update: boolean;
  client_id: string | null;
}

export interface DeviceCodeResponse {
  user_code: string;
  device_code: string;
  verification_uri: string;
  expires_in: number;
  interval: number;
  message: string;
}

export interface LauncherEventPayload {
  line?: string;
  pid?: number;
  exit_code?: number | null;
  message?: string;
  [key: string]: any;
}

export interface LauncherEvent {
  event: 'log' | 'started' | 'stopped' | 'error';
  payload: LauncherEventPayload;
}
