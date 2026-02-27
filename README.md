# RepoGuardia

> A save backup and restore tool for the game [R.E.P.O](https://store.steampowered.com/app/3241660/REPO/) by Semiwork. Participating in [Flavortown](https://flavortown.hackclub.com/projects/4936)!

**English** | [Français](README-FR.md)

![Alt](https://repobeats.axiom.co/api/embed/e3e267bb39578f6e0e2db4dc89e69aba06e9c099.svg "Repobeats analytics image")

![GitHub Activity Graph](https://github-readme-activity-graph.vercel.app/graph?username=Tr3yWay996&repo=RepoGuardia&theme=react-dark&hide_border=true)

---

## What is RepoGuardia?

If you've ever lost a R.E.P.O save after an update, a crash, or just bad luck because a game breaking bug hapened and you didn't had the nosavedelete mod, you know the pain. RepoGuardia is a small desktop app that lets you back up your R.E.P.O game saves and restore them whenever you need to. No terminal knowledge required (fortunatelly), no file juggling by  because honestly, who doesn't like when things are just smooth and not pain ? Just click, backup, and restart your save. (After restoring, if you are on the main menu to load saves, you need to return to the main menu first then come back so R.E.P.O sees the new save, this limitation is only due to how R.E.P.O process is handling game saves fetching, wich is understandable and not an issue in the slightest)

It runs on **Windows** and **Linux**. macOS is not supported for now since R.E.P.O doesn't seem to run there natively, but that could change down the road if there's demand.
Main menu screenshot:
<img width="798" height="594" alt="image" src="https://github.com/user-attachments/assets/f77c8b3d-8ade-473c-8203-dfc4ab1a7414" />  
<sub>This is the dev build launched using the command pnpm tauri dev, the compiled and bundled result might be slightly different</sub>

## The backstory [Lore]

This project started out of pure necessity. Back when R.E.P.O was freshly released, even before even the very first mods were even a thing, yeah those times were the very early days of R.E.P.O and the begining of a whole lot of terror in a fun way, so having the inconvenience of deleting the save on game loss I needed a way to protect my hard worked on and precious saves (Yes, i'm aware that loosing saves is part of the game, but i played, and still do now with my friends where we wipe the map from A to Z for maximum profit, dudes seeing number going up, dudes happy ahah). So I threw together a quick Python + PySide6 (Qt6) GUI app that did the job, buuuttt because i didn't care to learn anything and just outright **needed** this tool i built it using at first GPT 4.1 at the time, then sonnet 3.5 after seeing the limitations of G4.1, but anyway that isnt the point, i got the tool up to what i wanted and even polished its look by prompting it the changes i wanted, again back then i didn't care to learn, more on that because it's literally why i even made this one, and in RUST out of all things!

Later, after being lovingly peer-pressured by some mates who kept telling me how great Rust was and asking when I'd finally join the "Rusting community" as i call it, I decided to use this project as my learning material, so here we are! The first version was a terminal-only (TUI = Terminal User Interface) tool I built for myself with a ton of debuging prints and even custom macro prints for colored versions, its in this repo btw, just in the TUI branch if you wanna take a look, it's deprecated tho and may contain comments i left for myself at the time, its more for an archive now. Soooo to go bacl to it, once that version worked, I figured it would be nice to see a public usage of my own work for once and to do so I would need to make it actually usable for people who don't live in a terminal like some (:3). So I wrapped it in a proper GUI using [Tauri v2](https://v2.tauri.app/) with a Vue 3 + TypeScript frontend and PNPM as the package manager because why not, I had used it in the past already and i licked it.

And so here we are now in 2026 with RepoGuardia.

## Features

- **Backup your saves** — Pick a save, click a button, done. I made what i call a deduplication system, where if you make a backup of a save but you already did a backup, fear not because it'll put a .xxx (numbers, e.g. 741) at the end of each backup (and this program is specifically made for this).
- **Restore any backup** — browse your backups, pick one, and it gets copied back to the game's save directory.
- **Delete unwanted backups** — Select the backup you decide to leave behind, hit delete and boom, gone.
- **Put labels on em** — Yeah that's right, not only is it timestamped so you can determine where in the day / night you did them to know what level you were at, but you can also stop gueesing and start writing it down using the renaming feature, by default they don't have name.
- **List saves and backups** — See all your current saves and all your existing backups at a glance, sorted by date.
- **Configurable paths** — She settings page lets you change the game save location and the backup destination to whatever you want.
- **Cross-platform** — Sorks on Windows and Linux out of the box, pic the right package for your use and run it, on windows Tauri compiled apps work by installing btw.

<img width="799" height="597" alt="image" src="https://github.com/user-attachments/assets/5d1f4607-65ff-49c2-afe4-cbaca90a0077" />
<sub>That's where you do the backups</sub>

## Installation / Direct exec (talking about linux's .appimage)

Head over to the [Releases](https://github.com/Tr3yWay996/RepoGuardia/releases) page and grab the installer for your platform:

| Platform | Formats |
|----------|---------|
| Windows  | `.exe`, `.msi` (both are installers as mentioned earlier) |
| Linux    | `.deb`, `.rpm`, `.AppImage` |

Download, install, launch. That's it.

### First-time setup

When you open RepoGuardia for the first time, go to **Settings** and configure:

1. **Game saves path** — Where R.E.P.O stores its saves. By default this is:
   - Windows: `%AppData%\..\LocalLow\semiwork\REPO\saves`
   - Linux: None, because i can't guees where it is on your own proton sub file-system, and if it turns out i can then i'll add it whenever i can.
2. **Backup location** — That's where you want RepoGuardia to store backups. The game's own default backup directory is in `LocalLow\semiwork\REPO\backups` (prone to change, i encountered steam putting back deleted test backups into it a lot of time, then it synched and stoped but yeah its a thing that can happen, **be aware of that ! Thx**), but you can point this anywhere you like of course.
3. **Game version** — It's optional, just helps you keep track of which version your saves are from and would in the future separate saves from other versions. (Unused for now, use the label to write the game version if you want, but i'm sure cross version compatibility is properly done in R.E.P.O, thx Semiwork)

<img width="799" height="597" alt="image" src="https://github.com/user-attachments/assets/0f822997-c6bb-4455-b8b8-e365d68cd0c8" />
<sub>Settings page preview</sub>

## How to use it

The main menu gives you everything you need:

- **Backup save** — Lists your saves and lets you pick which one to back up. A copy is created in your backup directory with a unique ID and a metadata file.
- **Manage backup** — Lists your backups and lets you pick one to restore the original save, delete it and even putting a custom label onto it. Example: You are at the level 5, and you play with two friends named Thomson and the other Xander, well now you can back up the save showing at the top of the list (they are sorted by last modification date) and putting its label to “Level 5 with Thomson and Xander”, cool isn't it eh.

Each backup includes a `backup_metadata.json` file that tracks the original save name, the backup name, the creation timestamp, and the label.

<img width="799" height="597" alt="image" src="https://github.com/user-attachments/assets/17830ca6-d9c8-4564-b015-1df8b8d1a9dd" />
<sub>Backup list page preview</sub>
## Roadmap

- [x] Backup saves
- [x] Restore backups
- [x] List saves and backups with timestamps
- [x] Configurable save and backup paths
- [x] Cross-platform support (Windows + Linux)
- [x] Per-backup metadata (JSON)
- [x] Delete backups from the app
- [x] Rename backups
- [x] Custom labels and tags on backups (using the metadata file)

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
