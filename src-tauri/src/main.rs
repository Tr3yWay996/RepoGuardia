// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono;
use clearscreen::{self, clear};
use dircpy;
use lazy_static::lazy_static;
use serde_json::{self, to_string};
use std::fs::read;
use std::path::{self, Path};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::{io, process::exit};
use tauri_plugin_shell::ShellExt;
use walkdir::WalkDir;
use chrono::Local;
use chrono::DateTime;
use std::path::PathBuf;
use tauri::Manager;
use std::fs;

// Static config state
lazy_static! {
    static ref CONFIG_PATH: RwLock<Option<PathBuf>> = RwLock::new(None);
}

fn init_config_path(app_handle: &tauri::AppHandle) {
    let config_dir = app_handle.path().app_config_dir().expect("Failed to get config dir");
    
    // Create the directory if it doesn't exist
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    
    let mut path = config_dir;
    path.push("config-linux.json");  // or config-linux.json if you prefer
    let mut guard = CONFIG_PATH.write().unwrap();
    *guard = Some(path);
}


fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {  // <-- ADD THIS BLOCK
            init_config_path(&app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            test,
            open_konsole,
            exit_app,
            list_saves
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn greet(name: &str) -> String {
    if name.trim().is_empty() {
        return "Hello! You've been greeted from Rust!".into();
    }
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test(test: &str) -> String {
    format!("{}", test)
}
#[tauri::command]
async fn open_konsole(app: tauri::AppHandle) -> Result<(), String> {
    app.shell()
        .command("konsole")
        .args(&["-e", "bash", "-c", "echo 'Hello from Tauri my good Sir !' && TCP=$(curl -s ipv4.wtfismyip.com | sed 's/^\\([0-9]*\\)\\.[0-9]*\\.[0-9]*\\.\\([0-9]*\\)$/\\1.***.***.\\2/') && echo \"Hello you and your $TCP IP\" && RAM=$(fastfetch | grep Memory: | grep -oP '/ \\K[0-9.]+ [A-Za-z]+') && echo \"Wow, hold on here, you got a whapping $RAM of ram! Verify your doors are well locked!\" && read -p 'Press Enter to close...'"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn exit_app(handle: tauri::AppHandle) {
    handle.exit(0); // 0 = normal exit code
}

fn load_config() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let path_guard = CONFIG_PATH.read().unwrap();
    let path = path_guard.as_ref().ok_or("Config not initialized")?;  // <-- Unwrap the Option
    let config_contents = std::fs::read_to_string(path)?;  // <-- PathBuf auto-derefs to Path
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;
    Ok(config)
}
fn get_config_value(key: &str) -> String {
    let path_guard = CONFIG_PATH.read().unwrap();
    let config_path = match path_guard.as_ref() {
        Some(p) => p,
        None => {
            eprintln!("CONFIG DEBUG: CONFIG_PATH is None - init_config_path wasn't called");
            return String::new();
        }
    };
    
    eprintln!("CONFIG DEBUG: Looking for config at: {:?}", config_path);
    
    if !config_path.exists() {
        eprintln!("CONFIG DEBUG: File doesn't exist!");
        return String::new();
    }
    
    match load_config() {
        Ok(config) => {
            eprintln!("CONFIG DEBUG: Config loaded: {:?}", config);
            config.get(key).and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default()
        }
        Err(e) => {
            eprintln!("CONFIG DEBUG: Failed to load config: {}", e);
            String::new()
        }
    }
}

fn __settings() {

}

// 1 List saves to backup (Raw)
#[tauri::command]
fn list_saves() -> Result<Vec<(String, String)>, String> {
    let default_saves_path = get_config_value("default_saves_path");
    let mut dirs: Vec<(String, std::time::SystemTime)> = Vec::new();

    for entry in WalkDir::new(&default_saves_path).min_depth(1).max_depth(1) {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().is_dir() {
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            if let Ok(modified) = metadata.modified() {
                dirs.push((entry.path().display().to_string(), modified));
            }
        }
    }

    dirs.sort_by(|a, b| b.1.cmp(&a.1));

    let result: Vec<(String, String)> = dirs
        .into_iter()
        .map(|(full_path, modified)| {
            // Extract just the folder name (works on Windows AND Linux)
            let path = Path::new(&full_path);
            let folder_name = path
                .file_name()                           // Gets "REPO_SAVE_2026_02_01_17_21_38"
                .and_then(|n| n.to_str())              // Convert to &str
                .unwrap_or("unknown")                  // Fallback
                .to_string();
            
            let datetime: DateTime<Local> = modified.into();
            let formatted = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
            (folder_name, formatted)                    // Return ("REPO_SAVE_...", "03-02-2026...")
        })
        .collect();

    Ok(result)
}
// 2 Backup save (Raw)
// 3 Restore backup (Raw)
// 4 Restoring backup logic (Raw)
