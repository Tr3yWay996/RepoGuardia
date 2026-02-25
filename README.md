# RepoGuardia

> A save backup and restore tool for the game [R.E.P.O](https://store.steampowered.com/app/3241660/REPO/) by Semiwork.

**English** | [Français](README-FR.md)

![Alt](https://repobeats.axiom.co/api/embed/e3e267bb39578f6e0e2db4dc89e69aba06e9c099.svg "Repobeats analytics image")

---

## What is RepoGuardia?

If you've ever lost a R.E.P.O save after an update, a crash, or just bad luck — you know the pain. RepoGuardia is a small desktop app that lets you back up your R.E.P.O game saves and restore them whenever you need to. No terminal knowledge required, no file juggling by hand. Just click, backup, and sleep well.

It runs on **Windows** and **Linux**. macOS is not supported for now since R.E.P.O doesn't seem to run there natively, but that could change down the road if there's demand.

<!-- INSERT SCREENSHOT: Main menu overview -->

## The backstory

This project started out of pure necessity. Back when R.E.P.O was freshly released — before mods were even a thing — I needed a way to protect my saves. So I threw together a quick Python + PySide6 (Qt6) GUI app that did the job.

Later, after being lovingly peer-pressured by friends who kept telling me how great Rust was and asking when I'd finally join the "Rusting community", I decided to use this project as my learning material. The first version was a terminal-only (TUI) tool I built for myself. Once that worked, I figured it would be nice to make it actually usable for people who don't live in a terminal — so I wrapped it in a proper GUI using [Tauri v2](https://v2.tauri.app/) with a Vue 3 + TypeScript frontend.

And here we are.

## Features

- **Backup your saves** — pick a save, click a button, done. Each backup gets timestamped metadata so you always know what's what.
- **Restore any backup** — browse your backups, pick one, and it gets copied back to the game's save directory.
- **List saves and backups** — see all your current saves and all your existing backups at a glance, sorted by date.
- **Configurable paths** — the settings page lets you change the game save location and the backup destination to whatever you want.
- **Cross-platform** — works on Windows and Linux out of the box.

<!-- INSERT SCREENSHOT: Backup in action -->

## Installation

Head over to the [Releases](https://github.com/Tr3yWay996/RepoGuardia/releases) page and grab the installer for your platform:

| Platform | Formats |
|----------|---------|
| Windows  | `.exe`, `.msi` (both are installers) |
| Linux    | `.deb`, `.rpm`, `.AppImage` |

Download, install, launch. That's it.

### First-time setup

When you open RepoGuardia for the first time, go to **Settings** and configure:

1. **Game saves path** — where R.E.P.O stores its saves. By default this is:
   - Windows: `%AppData%\..\LocalLow\semiwork\REPO\saves`
   - Linux: `~/.config/unity3d/semiwork/REPO/saves` (may vary depending on your setup)
2. **Backup location** — where you want RepoGuardia to store backups. The game's own default backup directory is in `LocalLow\semiwork\REPO\backups`, but you can point this anywhere you like.
3. **Game version** — optional, just helps you keep track of which version your saves are from.

<!-- INSERT SCREENSHOT: Settings page -->

## How to use it

The main menu gives you everything you need:

- **List all saves** — shows every save folder the game currently has, with the last modified date.
- **Backup save** — lists your saves and lets you pick which one to back up. A copy is created in your backup directory with a unique ID and a metadata file.
- **List all backups** — shows all backups you've made, sorted by most recent.
- **Restore backup** — lists your backups and lets you pick one to restore back into the game's save folder.

Each backup includes a `backup_metadata.json` file that tracks the original save name, the backup name, and the creation timestamp.

<!-- INSERT SCREENSHOT: Backup list view -->

## Roadmap

- [x] Backup saves
- [x] Restore backups
- [x] List saves and backups with timestamps
- [x] Configurable save and backup paths
- [x] Cross-platform support (Windows + Linux)
- [x] Per-backup metadata (JSON)
- [ ] Delete backups from the app
- [ ] Rename backups
- [ ] Custom labels and tags on backups (using the metadata file)

---

## Technical details

This section is for people who are curious about how the app is built, or who want to contribute.

### Tech stack

| Layer     | Technology |
|-----------|------------|
| Backend   | **Rust** (via [Tauri v2](https://v2.tauri.app/)) |
| Frontend  | **Vue 3** + **TypeScript** (with `<script setup>` SFCs) |
| Bundler   | **Vite** |
| Package manager | **pnpm** |
| CI/CD     | **GitHub Actions** (Tauri action for building releases) |

### Project structure

```
RepoGuardia/
├── src/                          # Vue frontend
│   ├── views/
│   │   ├── Menu.vue              # Main menu / home screen
│   │   ├── Settings.vue          # Configuration page
│   │   ├── List_Saves.vue        # List game saves
│   │   ├── List_Backups.vue      # List existing backups
│   │   ├── Do_Backup.vue         # Perform a backup
│   │   └── Do_Restore.vue        # Restore a backup
│   ├── router/index.ts           # Vue Router setup
│   ├── App.vue                   # Root component with navigation
│   └── main.ts                   # App entry point
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri app bootstrap + command registration
│   │   ├── backup_convertion.rs  # Backup logic (copy + metadata creation)
│   │   ├── restore.rs            # Restore logic (metadata read + copy back)
│   │   ├── some_listing_action.rs# Directory scanning for saves and backups
│   │   └── some_config_action.rs # Config file management (JSON, per-OS)
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri app configuration
├── package.json                  # Node dependencies + scripts
└── vite.config.ts                # Vite configuration
```

### How it works under the hood

The frontend communicates with the Rust backend through Tauri's `invoke` system. Each button in the UI calls a Tauri command, and the Rust side does the actual file operations.

**Backup flow:**
1. The frontend calls `invoke("do_backup", { saveName })`.
2. Rust reads the configured save path, generates a unique backup folder name using a timestamp-based ID.
3. It creates the backup directory, writes a `backup_metadata.json` with the original name and creation date, then copies the entire save folder using `dircpy`.

**Restore flow:**
1. The frontend calls `invoke("do_restore", { saveName })`.
2. Rust locates the backup folder, reads its `backup_metadata.json` to find the original save name.
3. It copies the backup contents back into the game's save directory.

**Configuration:**
- Config is stored as a JSON file in the OS-appropriate app config directory (Tauri handles the path).
- On Linux: `config-linux.json` — On Windows: `config-windows.json`.
- The config holds three values: `default_saves_path`, `destination_base_path`, and `game_version`.

### Key Rust dependencies

| Crate | Purpose |
|-------|---------|
| `tauri` v2 | App framework, IPC, windowing |
| `serde` / `serde_json` | JSON serialization for config and metadata |
| `dircpy` | Recursive directory copying (backup & restore) |
| `walkdir` | Directory traversal for listing saves/backups |
| `chrono` | Timestamp formatting |
| `lazy_static` | Global config path state |

### Building from source

You'll need [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/), and [pnpm](https://pnpm.io/) installed.

```bash
# Clone the repo
git clone https://github.com/Tr3yWay996/RepoGuardia.git
cd RepoGuardia

# Install frontend dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

The production build will output platform-specific installers in `src-tauri/target/release/bundle/`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

Made by **Tr3yWay** (aka DarkDeception) — [GitHub](https://github.com/Tr3yWay996)
