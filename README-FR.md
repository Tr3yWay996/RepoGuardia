# RepoGuardia

> Un outil de sauvegarde et de restauration pour le jeu [R.E.P.O](https://store.steampowered.com/app/3241660/REPO/) de Semiwork. Participant à [Flavortown](https://flavortown.hackclub.com/projects/4936) !

**Anglais** | [Français](README-FR.md)

![Alt](https://repobeats.axiom.co/api/embed/e3e267bb39578f6e0e2db4dc89e69aba06e9c099.svg "Image analytique Repobeats")

![Graphique d'activité GitHub](https://github-readme-activity-graph.vercel.app/graph?username=Tr3yWay996&repo=RepoGuardia&theme=react-dark&hide_border=true)

---

## Qu'est-ce que RepoGuardia ?

Si vous avez déjà perdu une sauvegarde R.E.P.O après une mise à jour, un plantage ou simplement par malchance à cause d’un bug qui brise le jeu et que vous n’aviez pas le mod nosavedelete, vous connaissez la douleur. RepoGuardia est une petite application de bureau qui vous permet de sauvegarder vos parties de R.E.P.O et de les restaurer quand vous en avez besoin. Pas besoin de connaissances en terminal (heureusement), ni de jongler avec les fichiers parce qu’honnêtement, qui aime quand les choses sont compliquées et pénibles ? Il suffit de cliquer, sauvegarder, et redémarrer votre partie. (Après restauration, si vous êtes dans le menu principal pour charger une partie, vous devez d’abord retourner au menu principal puis revenir pour que R.E.P.O voie la nouvelle sauvegarde, cette limitation est uniquement due à la façon dont R.E.P.O gère la récupération des sauvegardes, ce qui est compréhensible et pas du tout problématique.)

Il fonctionne sous **Windows** et **Linux**. macOS n’est pas encore pris en charge puisque R.E.P.O ne semble pas y tourner nativement, mais cela pourrait changer si la demande s'en fait sentir.

## L’histoire [Lore]

Ce projet est né d’une pure nécessité. À la sortie de R.E.P.O, bien avant même les premiers mods, ces jours très tôt de R.E.P.O étaient aussi les débuts d’un sacré carnage amusant. Comme la suppression des sauvegardes à la perte du jeu posait problème, j’avais besoin d’un moyen de protéger mes sauvegardes précieuses et laborieuses. (Oui, je sais que perdre des sauvegardes fait partie du jeu, mais je jouais, et je joue toujours avec mes amis où on nettoie la carte de A à Z pour un maximum de profit, deux mecs qui lisent des gros chiffres qui montent de plus en plus, ça fait toujours plaisir d'atendre le million ahah). J’ai donc bricolé rapidement une application Python + PySide6 (Qt6) avec interface graphique, mais parce que je ne voulais pas apprendre quoi que ce soit et que j’avais juste **besoin** de cet outil, je l’ai construite au début avec GPT 4.1 puis sonnet 3.5 après avoir vu les limites de l’autre, mais peu importe, ce n’est pas le sujet puisque je n’ai jamais publié cette version Python, et ne le ferais jamais. J’avais l’outil prêt, je lui avais même donné un look de plus en plus soigné en lui demandant via prompts, encore une fois je ne voulais pas apprendre à l’époque, c’est pour cela que j’ai décidé de refaire cet outil, cette fois en RUST !

Plus tard, après avoir été poussé (doucement) par des potes qui me disaient combien Rust était top et me demandaient quand je rejoindrais la "Rusting comunity" comme j’appelle ça, j’ai décidé d’utiliser ce projet pour apprendre, et me voilà ! La première version était un outil en mode terminal (TUI = Interface Utilisateur par Terminal) que j’ai fait pour moi-même avec beaucoup d’impressions de débogage et même des macros personnalisées pour versions en couleur, c’est dans ce repo, dans la branche TUI si vous voulez jeter un œil, c’est déprécié et contient sûrement des commentaires que je me suis laissés, c’est surtout un archive maintenant. Une fois que cette version a fonctionné, je me suis dit que ça serait bien de voir un usage public de mon boulot, alors pour ça, il fallait que ça soit utilisable par des gens qui ne vivent pas dans un terminal comme certains (:3). Je l’ai donc embarqué dans une interface graphique avec [Tauri v2](https://v2.tauri.app/) et un frontend Vue 3 + TypeScript, avec PNPM comme gestionnaire de paquets parce que pourquoi pas, je l’avais déjà utilisé dans le passé et je l’aime bien.

Et voilà où nous en sommes en 2026 avec RepoGuardia.

## Captures d’écran

Menu principal au démarrage de l’app  
<img width="799" height="628" alt="image" src="https://github.com/user-attachments/assets/b117c5ee-675f-4066-9126-7ff1bde08171" />  
   
Menu des réglages  
<img width="799" height="627" alt="image" src="https://github.com/user-attachments/assets/f0ca1fdf-8e2c-4a0b-b847-9a7744aee06f" />   
   
Menu des sauvegardes  
<img width="799" height="627" alt="image" src="https://github.com/user-attachments/assets/2016d4d2-dd0b-4cd9-a5d5-7dd9e41d40e6" />   

Menu gestion des sauvegardes  
<img width="799" height="627" alt="image" src="https://github.com/user-attachments/assets/4087e955-8ea9-4bdf-8f43-51f7a992ab03" />   


## Fonctionnalités

- **Sauvegardez vos parties** — Choisissez une sauvegarde, cliquez sur un bouton, c’est fini. J’ai conçu un système de déduplication, ce qui veut dire que si vous faites une sauvegarde d’une partie déjà sauvegardée, pas de panique, un suffixe .xxx (ex : 741) sera ajouté à chaque sauvegarde (ce programme est spécifiquement fait pour ça).
- **Restaurez n’importe quelle sauvegarde** — Parcourez vos sauvegardes, choisissez-en une, elle est copiée dans le dossier de sauvegarde du jeu.
- **Supprimez les sauvegardes inutiles** — Sélectionnez la sauvegarde à éliminer, cliquez sur supprimer et hop, disparue.
- **Ajoutez des étiquettes** — Oui, c’est ça, elles sont horodatées pour savoir à quel moment de la journée ou de la nuit vous avez joué et où vous en étiez, mais vous pouvez aussi arrêter de deviner et noter en utilisant la fonction de renommage, par défaut elles n’ont pas de nom.
- **Liste des sauvegardes et backups** — Voyez toutes vos sauvegardes et backups d’un coup d’œil, triées par date.
- **Chemins configurables** — La page des réglages vous permet de modifier l’emplacement des sauvegardes de jeu et le dossier de destination des backups comme vous voulez.
- **Multi-plateforme** — Fonctionne sous Windows et Linux directement, vous prenez le bon package selon votre plateforme et vous lancez, sous Windows les apps Tauri compilées s’installent naturellement.

## Installation / Exécution directe (parlant du .appimage Linux)

Rendez-vous sur la page des [Releases](https://github.com/Tr3yWay996/RepoGuardia/releases) et récupérez l’installeur adapté à votre plateforme :

| Plateforme | Formats |
|------------|----------|
| Windows    | `.exe`, `.msi` (les deux sont des installeurs comme mentionné plus haut) |
| Linux      | `.deb`, `.rpm`, `.AppImage` |

Téléchargez, installez, lancez. C’est tout.

### Premier lancement

Quand vous ouvrez RepoGuardia la première fois, allez dans **Réglages** et configurez :

1. **Chemin des sauvegardes** — Où R.E.P.O stocke ses sauvegardes. Par défaut c’est :
   - Windows : `%AppData%\..\LocalLow\semiwork\REPO\saves`
   - Linux : Rien, parce que je ne sais pas où exactement sur votre sous-système proton, et si jamais je découvre, je l’ajouterai.
2. **Emplacement des backups** — C’est là que RepoGuardia va stocker les backups. Le dossier de sauvegarde par défaut du jeu est dans `LocalLow\semiwork\REPO\backups` (varie souvent, j’ai remarqué que Steam remettait régulièrement des backups test supprimés, puis ça s’est stabilisé, **faites attention à ça ! Merci**), mais vous pouvez bien sûr choisir un autre emplacement.
3. **Version du jeu** — Optionnel, ça vous aide à garder une trace de la version des sauvegardes et dans le futur séparera les sauvegardes des différentes versions. (Pas utilisé pour l’instant, utilisez l’étiquette pour écrire la version du jeu si vous voulez, je suis sûr que la compatibilité cross-version est bien gérée dans R.E.P.O, merci Semiwork)

## Comment l’utiliser

Le menu principal vous donne tout ce dont vous avez besoin :

- **Sauvegarder une partie** — Liste vos sauvegardes et vous permet d’en choisir une pour sauvegarder. Une copie est créée dans votre dossier de backup avec un ID unique et un fichier de métadonnées.
- **Gérer les sauvegardes** — Liste vos backups et vous laisse choisir un backup à restaurer, supprimer, et même ajouter une étiquette personnalisée. Exemple : vous êtes au niveau 5, vous jouez avec deux amis, Thomson et Xander, vous pouvez sauvegarder celle qui est en haut de la liste (triée par date de modification) et mettre l’étiquette « Niveau 5 avec Thomson et Xander », cool, non ?

Chaque backup contient un fichier `backup_metadata.json` qui enregistre le nom original de la sauvegarde, le nom du backup, la date de création et l’étiquette.

## Feuille de route

- [x] Sauvegarde des parties
- [x] Restauration des backups
- [x] Liste des sauvegardes et backups avec horodatage
- [x] Chemins de sauvegarde et backup configurables
- [x] Support multi-plateforme (Windows + Linux)
- [x] Métadonnées par backup (JSON)
- [x] Suppression des backups via l’app
- [x] Renommer les backups
- [x] Étiquettes personnalisées et tags sur les backups (via fichier métadonnées)
- [x] Vue unifiée de gestion des backups (restaurer, supprimer, étiqueter)

<sub>Et encore plus à venir...</sub>
---

## Détails techniques

Cette section est pour les curieux qui veulent savoir comment l’app est construite, ou pour ceux qui souhaitent contribuer.

### Tech stack

| Couche    | Technologie |
|-----------|-------------|
| Backend   | **Rust** (via [Tauri v2](https://v2.tauri.app/)) |
| Frontend  | **Vue 3** + **TypeScript** (avec SFC `<script setup>`) |
| Bundler   | **Vite** |
| Gestionnaire de paquets | **pnpm** |
| CI/CD     | **GitHub Actions** (action Tauri pour créer les releases) |

### Structure du projet

```
RepoGuardia/
├── src/                          # Frontend Vue
│   ├── assets/
│   │   ├── fonts/                # Polices personnalisées (IBM Plex Serif)
│   │   └── vue.svg
│   ├── views/
│   │   ├── Menu.vue              # Menu principal / écran d’accueil
│   │   ├── Settings.vue          # Page de configuration
│   │   ├── List_Saves.vue        # Liste des sauvegardes du jeu
│   │   ├── List_Backups.vue      # Liste des backups existants
│   │   ├── Do_Backup.vue         # Faire une sauvegarde
│   │   ├── Do_Restore.vue        # Restaurer un backup
│   │   ├── Do_Delete.vue         # Supprimer un backup
│   │   ├── Do-Naming.vue         # Définir des étiquettes personnalisées
│   │   └── Manage_Backups.vue    # Gestion unifiée des backups (restaurer, supprimer, étiqueter)
│   ├── router/index.ts           # Configuration Vue Router
│   ├── App.vue                   # Composant racine avec navigation
│   ├── main.ts                   # Point d’entrée de l’app
│   └── vite-env.d.ts             # Déclarations de type Vite
├── src-tauri/                    # Backend Rust
│   ├── src/
│   │   ├── main.rs               # Bootstrap app Tauri + enregistrement des commandes
│   │   ├── backup_convertion.rs  # Logique de backup (copie + métadonnées)
│   │   ├── restore.rs            # Logique de restauration (lecture métadonnées + copie retour)
│   │   ├── delete.rs             # Logique suppression backup (suppression répertoire)
│   │   ├── rename.rs             # Logique d’étiquetage/renommage (mise à jour métadonnées)
│   │   ├── some_listing_action.rs# Scan des dossiers pour sauvegardes et backups
│   │   └── some_config_action.rs # Gestion du fichier de config (JSON, par OS)
│   ├── Cargo.toml                # Dépendances Rust
│   └── tauri.conf.json           # Configuration app Tauri
├── package.json                  # Dépendances Node + scripts
├── vite.config.ts                # Configuration Vite
└── LICENSE.md                    # Licence MIT
```

### Fonctionnement en coulisses

Le frontend communique avec le backend Rust via le système `invoke` de Tauri. Chaque bouton de l’interface appelle une commande Tauri, et Rust réalise les opérations sur les fichiers.

**Processus de sauvegarde :**  
1. Le frontend appelle `invoke("do_backup", { saveName })`.  
2. Rust lit le chemin des sauvegardes configuré, génère un nom de dossier unique basé sur un timestamp.  
3. Il crée le dossier de backup, écrit un fichier `backup_metadata.json` avec le nom original et la date de création, puis copie tout le dossier de sauvegarde avec `dircpy`.

**Processus de restauration :**  
1. Le frontend appelle `invoke("do_restore", { saveName })`.  
2. Rust localise le dossier de backup, lit son `backup_metadata.json` pour retrouver le nom original de la sauvegarde.  
3. Il copie le contenu du backup dans le dossier de sauvegarde du jeu.

**Configuration :**  
- Le fichier de config est un JSON stocké dans le dossier approprié selon l’OS (géré par Tauri).  
- Sur Linux : `config-linux.json` — Sur Windows : `config-windows.json`.  
- Le config contient trois valeurs : `default_saves_path`, `destination_base_path` et `game_version`.

### Dépendances Rust clés

| Bibliothèque | Usage |
|--------------|-------|
| `tauri` v2   | Framework app, IPC, gestion fenêtres |
| `serde` / `serde_json` | Sérialisation JSON pour config et métadonnées |
| `dircpy`     | Copie récursive de dossiers (backup & restauration) |
| `walkdir`    | Parcours des dossiers pour lister les sauvegardes/backups |
| `chrono`     | Formatage des timestamps |
| `lazy_static`| État global du chemin de config |

### Compilation depuis les sources

Vous aurez besoin de [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/) et [pnpm](https://pnpm.io/) installés.

```bash
# Cloner le repo
git clone https://github.com/Tr3yWay996/RepoGuardia.git
cd RepoGuardia

# Installer les dépendances frontend
pnpm install

# Lancer en mode développement
pnpm tauri dev

# Compiler pour la production
pnpm tauri build
```

Le build production génère les installeurs spécifiques à la plateforme dans `src-tauri/target/release/bundle/`.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour les détails.

## Auteur

Réalisé par **Tr3yWay** (alias DarkDeception) — [GitHub](https://github.com/Tr3yWay996)
