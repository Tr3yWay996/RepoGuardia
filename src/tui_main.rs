use std::{error::Error, io, path::PathBuf};

use chrono::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use walkdir::WalkDir;

// --- Configuration & Logic Helpers (Adapted from main.rs) ---

static CONFIG_PATH: &str = "config.json";

fn load_config() -> Result<serde_json::Value, Box<dyn Error>> {
    let config_contents = std::fs::read_to_string(CONFIG_PATH)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;
    Ok(config)
}

fn get_config_value(key: &str) -> String {
    load_config()
        .ok()
        .and_then(|config| {
            config
                .get(key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "".to_string())
}

// --- App State ---

enum CurrentScreen {
    MainMenu,
    BackupList,    // Option 1: List backed up saves (and Restore logic could go here)
    SaveList,      // Option 2: Copy a save folder
    RestoreList,   // Option 3: Restore backup
    DeleteBackup,  // Option 4: Delete backed-up saves
    ChooseVersion, // Option 5: Choose backup version
}

struct App {
    current_screen: CurrentScreen,
    items: Vec<String>, // Generic list of items to display
    state: ListState,   // State for the list widget
    status_message: String,
    input_buffer: String,
}

impl App {
    fn new() -> App {
        App {
            current_screen: CurrentScreen::MainMenu,
            items: vec![
                "1. List backed up saves".to_string(),
                "2. Backup a save folder".to_string(),
                "3. Restore backup".to_string(),
                "4. Choose game Version".to_string(),
                "5. Delete save backup".to_string(),
                "Exit".to_string(),
            ],
            state: ListState::default(),
            status_message: String::from("Welcome to RepoGuardian TUI!"),
            input_buffer: String::new(),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

// --- Main Entry Point ---

pub fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    app.state.select(Some(0)); // Select first item by default

    // Run app loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.current_screen {
                    CurrentScreen::ChooseVersion => match key.code {
                        KeyCode::Char(c) => {
                            app.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            save_version(app);
                            app.current_screen = CurrentScreen::MainMenu;
                            app.items = vec![
                                "1. List backed up saves".to_string(),
                                "2. Backup a save folder".to_string(),
                                "3. Restore backup".to_string(),
                                "4. Choose game Version".to_string(),
                                "5. Delete save backup".to_string(),
                                "Exit".to_string(),
                            ];
                            app.state.select(Some(0));
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::MainMenu;
                            app.items = vec![
                                "1. List backed up saves".to_string(),
                                "2. Backup a save folder".to_string(),
                                "3. Restore backup".to_string(),
                                "4. Choose game Version".to_string(),
                                "5. Delete save backup".to_string(),
                                "Exit".to_string(),
                            ];
                            app.state.select(Some(0));
                            app.status_message = "Cancelled version selection".to_string();
                        }
                        _ => {}
                    },
                    _ => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Down => app.next(),
                            KeyCode::Up => app.previous(),
                            KeyCode::Esc | KeyCode::Backspace => {
                                // Go back to main menu
                                app.current_screen = CurrentScreen::MainMenu;
                                app.items = vec![
                                    "1. List backed up saves".to_string(),
                                    "2. Backup a save folder".to_string(),
                                    "3. Restore backup".to_string(),
                                    "4. Choose game version".to_string(),
                                    "5. Delete save backup".to_string(),
                                    "Exit".to_string(),
                                ];
                                app.state.select(Some(0));
                                app.status_message = "Main menu".to_string();
                            }
                            KeyCode::Enter => {
                                match app.current_screen {
                                    CurrentScreen::MainMenu => {
                                        if let Some(selected) = app.state.selected() {
                                            match selected {
                                                0 => {
                                                    // List Backed Up Saves
                                                    app.current_screen = CurrentScreen::BackupList;
                                                    load_backups(app);
                                                }
                                                1 => {
                                                    // Backup a Save Folder
                                                    app.current_screen = CurrentScreen::SaveList;
                                                    load_save_folders(app);
                                                }
                                                2 => {
                                                    // Restore Backup
                                                    app.current_screen = CurrentScreen::RestoreList;
                                                    load_backups(app); // Reuse load_backups for now
                                                }
                                                3 => {
                                                    app.current_screen =
                                                        CurrentScreen::ChooseVersion;
                                                    app.input_buffer =
                                                        get_config_value("last_version");
                                                }
                                                4 => {
                                                    app.current_screen =
                                                        CurrentScreen::DeleteBackup;
                                                    delete_backups(app);
                                                }
                                                5 => return Ok(()), // Exit
                                                _ => {}
                                            }
                                        }
                                    }
                                    CurrentScreen::SaveList => {
                                        // Handle selection in Save List (Perform Backup)
                                        if let Some(selected) = app.state.selected() {
                                            if selected < app.items.len() {
                                                // let selected_folder = app.items[selected].clone();
                                                // Parse the path from the display string or store paths separately
                                                // For simplicity, let's assume we re-fetch or parse.
                                                // Ideally, App should store (DisplayString, ActualPath) tuples.
                                                perform_backup(app, selected);
                                            }
                                        }
                                    }
                                    CurrentScreen::RestoreList => {
                                        // Handle selection in Restore List
                                        if let Some(selected) = app.state.selected() {
                                            perform_restore(app, selected);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

// --- Logic Functions ---

fn load_save_folders(app: &mut App) {
    app.items.clear();
    app.status_message = "Select a folder to BACKUP (Press Enter)".to_string();

    let default_saves_path = get_config_value("default_saves_path");
    if default_saves_path.is_empty() {
        app.items
            .push("Error: default_saves_path not set in config".to_string());
        return;
    }

    for entry in WalkDir::new(&default_saves_path).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry
            && entry.file_type().is_dir()
        {
            app.items.push(entry.path().display().to_string());
        }
    }
    app.state.select(Some(0));
}

fn save_version(app: &mut App) {
    app.items.clear();
    app.status_message = "Type version and press Enter".to_string();
    let mut config = load_config().unwrap_or_else(|_| serde_json::json!({}));
    let version = app.input_buffer.trim().to_string();
    config["last_version"] = serde_json::Value::String(version.clone());
    if let Err(e) = std::fs::write(
        CONFIG_PATH,
        serde_json::to_string_pretty(&config).unwrap_or_default(),
    ) {
        app.status_message = format!("Failed to save config: {}", e);
    } else {
        app.status_message = format!("Version saved: {}", version);
    }
}

fn delete_backups(app: &mut App) {
    app.items.clear();
    app.status_message = "Select the backed-up save to delete and press enter".to_string();
    let destination_base_path = get_config_value("destination_base_path");
    for entry in WalkDir::new(&destination_base_path)
        .min_depth(2)
        .max_depth(2)
    {
        app.items.push(entry?);
    }
}

fn load_backups(app: &mut App) {
    app.items.clear();
    app.status_message =
        "List of Backups (Press Enter to Restore if in Restore Mode, else its read only)"
            .to_string();

    let destination_base_path = get_config_value("destination_base_path");
    if destination_base_path.is_empty() {
        app.items
            .push("Error: destination_base_path not set in config".to_string());
        return;
    }

    // Note: Logic adapted from main.rs option 3
    for entry in WalkDir::new(&destination_base_path)
        .min_depth(2)
        .max_depth(2)
    {
        if let Ok(entry) = entry {
            if entry.file_type().is_dir() {
                let metadata_check = entry.path().join("backup_metadata.json");
                if metadata_check.exists() {
                    // Read metadata to show nice name
                    let display_name =
                        if let Ok(contents) = std::fs::read_to_string(&metadata_check) {
                            if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&contents) {
                                let original = meta
                                    .get("original_name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("?");
                                let created = meta
                                    .get("created_at")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("?");
                                format!("{} (created: {})", original, created)
                            } else {
                                entry.path().display().to_string()
                            }
                        } else {
                            entry.path().display().to_string()
                        };
                    app.items.push(display_name);
                }
            }
        }
    }
    app.state.select(Some(0));
}

fn perform_backup(app: &mut App, selected_index: usize) {
    // Re-fetch paths to get the actual path (since app.items might be display strings)
    // In a real app, store struct { path, display_name }
    let default_saves_path = get_config_value("default_saves_path");
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(&default_saves_path).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            if entry.file_type().is_dir() {
                dirs.push(entry.path().to_path_buf());
            }
        }
    }

    if selected_index < dirs.len() {
        let selected_path = &dirs[selected_index];

        // Logic from main.rs Option 2
        let mut config = load_config().unwrap_or_else(|_| serde_json::json!({}));
        let last_version = get_config_value("last_version");
        // For TUI simplicity, we'll just use last_version for now or default to "0.0.0"
        // A real TUI would pop up an input box here.
        let version = if last_version.is_empty() {
            "0.0.0".to_string()
        } else {
            last_version
        };

        // Update config with last used version (even if we just read it, this mimics the original behavior of saving it)
        config["last_version"] = serde_json::Value::String(version.clone());
        let _ = std::fs::write(
            CONFIG_PATH,
            serde_json::to_string_pretty(&config).unwrap_or_default(),
        );

        let destination_base_path = get_config_value("destination_base_path");
        let version_path = std::path::Path::new(&destination_base_path).join(&version);

        let dir_name = selected_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("backup");
        let timestamp = Local::now().format("%4f").to_string();
        let original_name = dir_name;
        let backup_name = format!("{}.{}", original_name, timestamp);
        let backup_folder = version_path.join(&backup_name);
        let full_destination = version_path
            .join(format!("{}.{}", original_name, timestamp))
            .join(format!("{}", dir_name));

        match dircpy::copy_dir(selected_path, &full_destination) {
            Ok(_) => {
                // Create metadata file
                let metadata = serde_json::json!({
                    "original_name": original_name,
                    "backup_name": backup_name,
                    "created_at": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    "label": ""
                });
                let metadata_path = backup_folder.join("backup_metadata.json");
                let _ = std::fs::write(
                    &metadata_path,
                    serde_json::to_string_pretty(&metadata).unwrap_or_default(),
                );

                app.status_message =
                    format!("Success! Backed up to {}", full_destination.display());
            }
            Err(e) => {
                app.status_message = format!("Error copying: {}", e);
            }
        }
    }
}

fn perform_restore(app: &mut App, selected_index: usize) {
    let destination_base_path = get_config_value("destination_base_path");
    let mut backup_dirs: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(&destination_base_path)
        .min_depth(2)
        .max_depth(2)
    {
        if let Ok(entry) = entry {
            if entry.file_type().is_dir() {
                let metadata_check = entry.path().join("backup_metadata.json");
                if metadata_check.exists() {
                    backup_dirs.push(entry.path().to_path_buf());
                }
            }
        }
    }

    if selected_index < backup_dirs.len() {
        let backup_folder = &backup_dirs[selected_index];

        // Read metadata
        let metadata_path = backup_folder.join("backup_metadata.json");
        if let Ok(metadata_contents) = std::fs::read_to_string(&metadata_path) {
            if let Ok(metadata) = serde_json::from_str::<serde_json::Value>(&metadata_contents) {
                let original_name = metadata
                    .get("original_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                let save_folder = backup_folder.join(original_name);
                let default_saves_path = get_config_value("default_saves_path");
                let full_destination =
                    std::path::Path::new(&default_saves_path).join(original_name);

                if full_destination.exists() {
                    let _ = std::fs::remove_dir_all(&full_destination);
                }

                match dircpy::copy_dir(&save_folder, &full_destination) {
                    Ok(_) => {
                        app.status_message = format!("Restored {} successfully!", original_name)
                    }
                    Err(e) => app.status_message = format!("Error restoring: {}", e),
                }
            }
        }
    }
}

// --- UI Rendering ---

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Min(0),    // List
                Constraint::Length(3), // Status
            ]
            .as_ref(),
        )
        .split(f.area());

    let title = Paragraph::new("RepoGuardian TUI")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    match app.current_screen {
        CurrentScreen::ChooseVersion => {
            let input = Paragraph::new(app.input_buffer.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Enter Game Version"),
                );
            f.render_widget(input, chunks[1]);
        }
        _ => {
            let items: Vec<ListItem> = app
                .items
                .iter()
                .map(|i| {
                    ListItem::new(Line::from(Span::raw(i))).style(Style::default().fg(Color::White))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Items"))
                .highlight_style(
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[1], &mut app.state.clone());
        }
    }

    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status, chunks[2]);
}
