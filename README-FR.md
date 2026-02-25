# RepoGuardia

> Un outil de sauvegarde et de restauration pour le jeu [R.E.P.O](https://store.steampowered.com/app/3241660/REPO/) par Semiwork.

[English](README.md) | **Français**
![Alt](https://repobeats.axiom.co/api/embed/e3e267bb39578f6e0e2db4dc89e69aba06e9c099.svg "Repobeats analytics image")
![GitHub Activity Graph](https://github-readme-activity-graph.vercel.app/graph?username=Tr3yWay996&repo=RepoGuardia&theme=react-dark&hide_border=true)

---

## C'est quoi RepoGuardia ?

Si tu as déjà perdu une sauvegarde R.E.P.O après une mise à jour, un crash, ou juste un coup de malchance — tu connais la douleur. RepoGuardia est une petite application de bureau qui te permet de sauvegarder tes parties R.E.P.O et de les restaurer quand tu en as besoin. Pas besoin de connaître le terminal, pas besoin de jongler avec les fichiers à la main. Tu cliques, ça sauvegarde, et tu dors tranquille.

L'application tourne sur **Windows** et **Linux**. macOS n'est pas supporté pour le moment vu que R.E.P.O ne semble pas tourner dessus nativement, mais ça pourrait changer si la demande est là.

<!-- INSERT SCREENSHOT: Aperçu du menu principal -->

## La petite histoire

Ce projet est né par pure nécessité. Quand R.E.P.O venait tout juste de sortir — avant même que les mods existent — j'avais besoin d'un moyen de protéger mes sauvegardes. Du coup j'ai bricolé une app en Python + PySide6 (Qt6) qui faisait le boulot.

Plus tard, après m'être fait gentiment harceler par des amis qui n'arrêtaient pas de me dire à quel point Rust c'est bien et qui me demandaient quand j'allais enfin rejoindre la "communauté Rust", j'ai décidé d'utiliser ce projet comme support d'apprentissage. La première version était un outil en ligne de commande (TUI) que j'avais fait pour moi. Une fois que ça marchait, je me suis dit que ce serait sympa de le rendre utilisable pour des gens qui ne vivent pas dans un terminal — alors j'ai enveloppé le tout dans une vraie interface graphique avec [Tauri v2](https://v2.tauri.app/), Vue 3 et TypeScript en frontend.

Et voilà où on en est.

## Fonctionnalités

- **Sauvegarde tes parties** — choisis une save, clique sur un bouton, c'est fait. Chaque backup reçoit des métadonnées horodatées pour que tu saches toujours ce qui est quoi.
- **Restaure n'importe quel backup** — parcours tes sauvegardes, choisis-en une, et elle est recopiée dans le dossier de saves du jeu.
- **Liste les saves et les backups** — vois toutes tes saves actuelles et tous tes backups existants d'un coup d'œil, triés par date.
- **Chemins configurables** — la page de réglages te permet de changer l'emplacement des saves du jeu et la destination des backups comme tu veux.
- **Multi-plateforme** — fonctionne sur Windows et Linux directement.

<!-- INSERT SCREENSHOT: Backup en cours -->

## Installation

Direction la page [Releases](https://github.com/Tr3yWay996/RepoGuardia/releases) pour télécharger l'installeur correspondant à ta plateforme :

| Plateforme | Formats |
|------------|---------|
| Windows    | `.exe`, `.msi` (les deux sont des installeurs) |
| Linux      | `.deb`, `.rpm`, `.AppImage` |

Tu télécharges, tu installes, tu lances. C'est tout.

### Premier lancement

Quand tu ouvres RepoGuardia pour la première fois, va dans **Settings** et configure :

1. **Chemin des saves du jeu** — là où R.E.P.O stocke ses sauvegardes. Par défaut c'est :
   - Windows : `%AppData%\..\LocalLow\semiwork\REPO\saves`
   - Linux : `~/.config/unity3d/semiwork/REPO/saves` (peut varier selon ta config)
2. **Emplacement des backups** — là où tu veux que RepoGuardia stocke les sauvegardes. Le dossier de backup par défaut du jeu est dans `LocalLow\semiwork\REPO\backups`, mais tu peux mettre n'importe quel chemin.
3. **Version du jeu** — optionnel, ça t'aide juste à savoir de quelle version viennent tes saves.

<!-- INSERT SCREENSHOT: Page des réglages -->

## Comment l'utiliser

Le menu principal te donne tout ce qu'il faut :

- **List all saves** — affiche chaque dossier de sauvegarde que le jeu possède actuellement, avec la date de dernière modification.
- **Backup save** — liste tes saves et te laisse choisir laquelle sauvegarder. Une copie est créée dans ton dossier de backup avec un identifiant unique et un fichier de métadonnées.
- **List all backups** — affiche tous les backups que tu as faits, triés du plus récent au plus ancien.
- **Restore backup** — liste tes backups et te laisse en choisir un pour le restaurer dans le dossier de saves du jeu.

Chaque backup inclut un fichier `backup_metadata.json` qui garde en mémoire le nom original de la save, le nom du backup, et la date de création.

<!-- INSERT SCREENSHOT: Liste des backups -->

## Feuille de route

- [x] Sauvegarder les saves
- [x] Restaurer les backups
- [x] Lister saves et backups avec horodatage
- [x] Chemins de saves et backups configurables
- [x] Support multi-plateforme (Windows + Linux)
- [x] Métadonnées par backup (JSON)
- [ ] Supprimer des backups depuis l'app
- [ ] Renommer les backups
- [ ] Labels et tags personnalisés sur les backups (via le fichier de métadonnées)

---

## Détails techniques

Cette section est pour les curieux qui veulent savoir comment l'app est construite, ou qui veulent contribuer.

### Stack technique

| Couche | Technologie |
|--------|-------------|
| Backend | **Rust** (via [Tauri v2](https://v2.tauri.app/)) |
| Frontend | **Vue 3** + **TypeScript** (avec `<script setup>` SFCs) |
| Bundler | **Vite** |
| Gestionnaire de paquets | **pnpm** |
| CI/CD | **GitHub Actions** (Tauri action pour le build des releases) |

### Structure du projet

```
RepoGuardia/
├── src/                          # Frontend Vue
│   ├── views/
│   │   ├── Menu.vue              # Menu principal / écran d'accueil
│   │   ├── Settings.vue          # Page de configuration
│   │   ├── List_Saves.vue        # Liste des saves du jeu
│   │   ├── List_Backups.vue      # Liste des backups existants
│   │   ├── Do_Backup.vue         # Effectuer un backup
│   │   └── Do_Restore.vue        # Restaurer un backup
│   ├── router/index.ts           # Configuration du Vue Router
│   ├── App.vue                   # Composant racine avec navigation
│   └── main.ts                   # Point d'entrée de l'app
├── src-tauri/                    # Backend Rust
│   ├── src/
│   │   ├── main.rs               # Bootstrap Tauri + enregistrement des commandes
│   │   ├── backup_convertion.rs  # Logique de backup (copie + création métadonnées)
│   │   ├── restore.rs            # Logique de restauration (lecture métadonnées + copie)
│   │   ├── some_listing_action.rs# Scan des dossiers saves et backups
│   │   └── some_config_action.rs # Gestion du fichier de config (JSON, par OS)
│   ├── Cargo.toml                # Dépendances Rust
│   └── tauri.conf.json           # Configuration de l'app Tauri
├── package.json                  # Dépendances Node + scripts
└── vite.config.ts                # Configuration Vite
```

### Comment ça marche sous le capot

Le frontend communique avec le backend Rust via le système `invoke` de Tauri. Chaque bouton de l'interface appelle une commande Tauri, et c'est le côté Rust qui fait les opérations sur les fichiers.

**Flux de backup :**
1. Le frontend appelle `invoke("do_backup", { saveName })`.
2. Rust lit le chemin de save configuré, génère un nom de dossier de backup unique avec un ID basé sur le timestamp.
3. Il crée le dossier de backup, écrit un `backup_metadata.json` avec le nom original et la date de création, puis copie tout le dossier de save avec `dircpy`.

**Flux de restauration :**
1. Le frontend appelle `invoke("do_restore", { saveName })`.
2. Rust localise le dossier de backup, lit son `backup_metadata.json` pour trouver le nom original de la save.
3. Il copie le contenu du backup dans le dossier de saves du jeu.

**Configuration :**
- La config est stockée dans un fichier JSON dans le dossier de config de l'app approprié à l'OS (Tauri gère le chemin).
- Sur Linux : `config-linux.json` — Sur Windows : `config-windows.json`.
- La config contient trois valeurs : `default_saves_path`, `destination_base_path`, et `game_version`.

### Dépendances Rust principales

| Crate | Rôle |
|-------|------|
| `tauri` v2 | Framework applicatif, IPC, fenêtrage |
| `serde` / `serde_json` | Sérialisation JSON pour la config et les métadonnées |
| `dircpy` | Copie récursive de dossiers (backup & restauration) |
| `walkdir` | Parcours de dossiers pour lister saves/backups |
| `chrono` | Formatage des timestamps |
| `lazy_static` | État global du chemin de config |

### Compiler depuis les sources

Il faut [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/), et [pnpm](https://pnpm.io/) installés.

```bash
# Cloner le dépôt
git clone https://github.com/Tr3yWay996/RepoGuardia.git
cd RepoGuardia

# Installer les dépendances frontend
pnpm install

# Lancer en mode développement
pnpm tauri dev

# Compiler pour la production
pnpm tauri build
```

Le build de production génère les installeurs spécifiques à la plateforme dans `src-tauri/target/release/bundle/`.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour les détails.

## Auteur

Fait par **Tr3yWay** (alias DarkDeception) — [GitHub](https://github.com/Tr3yWay996)
