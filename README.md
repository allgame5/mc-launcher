# Minecraft Launcher (Tauri v2 + React)

Moderner Desktop-Minecraft-Launcher im Stil der Modrinth App. Reine Client-Seite, keine eigene Server-Infrastruktur.

## Tech-Stack

- **Backend:** Rust + Tauri v2 (tokio, reqwest, serde, sha1, sha2, zip, uuid, chrono, tauri-plugin-updater, tauri-plugin-shell)
- **Frontend:** React 18 + TypeScript + Tailwind CSS + Zustand + TanStack Query
- **Plattformen:** Windows, macOS, Linux

## Voraussetzungen

- [Rust](https://rustup.rs/) (MSVC auf Windows empfohlen)
- [Node.js](https://nodejs.org/) 18+
- Ein Microsoft Azure App Registration (für OAuth2 Client-ID)

## Projektstruktur

```
mc-launcher/
├── src-tauri/
│   ├── Cargo.toml              # Rust-Dependencies
│   ├── tauri.conf.json         # Tauri-Konfiguration (Allowlist, Plugins, Updater)
│   └── src/
│       ├── main.rs             # Tauri-Einstieg, Command-Registrierung
│       ├── config.rs           # App-Daten-Verzeichnisse, Einstellungen
│       ├── error.rs            # Einheitliche Fehler-Typen
│       ├── instances/          # Instanz-Manager (CRUD, instance.json)
│       ├── launcher/           # Mojang/Java/Prozess-Pipeline
│       ├── modrinth/           # Modrinth-API-Integration
│       ├── auth/               # Microsoft OAuth2 Device-Code-Flow
│       └── updater/            # tauri-plugin-updater
└── src/                        # React-Frontend
    ├── components/             # UI-Komponenten (Sidebar, Library, Discover, etc.)
    ├── pages/                  # Seiten (Library, Discover, Accounts, Settings)
    ├── stores/                 # Zustand-State-Management
    ├── hooks/                  # React-Hooks für Tauri-Events
    ├── lib/                    # API-Wrapper (invoke) & Event-Listener
    └── types/                  # Shared TypeScript-Typen
```

## Installation & Start

### 1. Dependencies installieren

```powershell
# Im mc-launcher Verzeichnis
npm install
```

### 2. Rust-Backend prüfen

```powershell
cd src-tauri
cargo check
```

> **Hinweis für Windows:** Auf diesem PC blockiert eine Anwendungssteuerungsrichtlinie (AppLocker/WDAC) die Ausführung von Cargo-Build-Skripten (Fehler: "Eine Anwendungssteuerungsrichtlinie hat diese Datei blockiert. (os error 4551)"). Das ist ein System-Sicherheitsfeature, kein Fehler im Code. Um `cargo check` bzw. den Build auszuführen, muss die Richtlinie vorübergehend ausgesetzt oder das `target`-Verzeichnis ausgeschlossen werden (erfordert Administratorrechte).

### 3. Frontend prüfen (TypeScript)

```powershell
# Im mc-launcher Verzeichnis
npx tsc --noEmit
# Ergebnis: keine Fehler ✅
```

### 4. App im Dev-Modus starten

```powershell
npm run dev
```

### 5. Produktions-Build

```powershell
npm run build
# Dann: cargo tauri build (im src-tauri Verzeichnis, sobald cargo funktioniert)
```

## Konfiguration

### Microsoft Client-ID

Die Client-ID für den OAuth2-Device-Code-Flow kann auf zwei Arten gesetzt werden:

1. Umgebungsvariable: `MINECRAFT_LAUNCHER_CLIENT_ID=<deine-client-id>`
2. In den Einstellungen der App (Feld "Microsoft Client-ID")

Die Client-ID erhältst du über eine [Azure App Registration](https://portal.azure.com).

### App-Daten-Verzeichnis

Alle Daten werden unter dem OS-Standard-AppData-Verzeichnis gespeichert:
- **Windows:** `%APPDATA%\MinecraftLauncher\`
- **macOS:** `~/Library/Application Support/MinecraftLauncher/`
- **Linux:** `~/.local/share/MinecraftLauncher/`

Darunter:
- `instances/<id>/` – Instanz-Verzeichnisse mit `instance.json`
- `java/` – Auto-detektierte/ heruntergeladene JREs
- `cache/` – Download-Cache
- `logs/` – Launcher-Logs
- `launcher.json` – Launcher-Einstellungen
- `accounts.json` – Gespeicherte Microsoft-Tokens (lokal, nur Access/Refresh Token)

## Kernfunktionen

| Modul | Beschreibung |
|-------|-------------|
| `instances` | CRUD für Instanzen mit `instance.json`, Verzeichnisstruktur (`mods/`, `config/`, `logs/`) |
| `launcher` | Mojang Version Manifest, Client-JAR/Libraries/Assets Download mit SHA1, Fabric-Installer, Adoptium JRE-Erkennung, Minecraft-Prozessstart mit Log-Streaming |
| `modrinth` | Suche (Mods, Modpacks, Shaders, Resourcepacks), Dependency-Auflösung, SHA512-Verifizierung, lokales `installed_mods.json` Tracking |
| `auth` | Microsoft OAuth2 Device-Code-Flow → XBL → XSTS → Minecraft Services, Token-Speicherung, Account-Wechsel |
| `updater` | `tauri-plugin-updater` mit GitHub Releases Endpoint, Update-Check beim Start |

## Bekannte Einschränkungen (MVP)

- Der Fabric-Installer wird heruntergeladen, die vollständige Fabric-Installation erfordert zusätzliche Logik
- Der Minecraft-Start verwendet Platzhalter für Auth-Tokens (MVP: Offline-Modus / lokaler Spieler)
- Natives-Entpacken für Libraries ist noch nicht implementiert
- Windows `cargo check` wird durch System-Sicherheitsrichtlinie blockiert (siehe oben)

## Lizenz

MIT