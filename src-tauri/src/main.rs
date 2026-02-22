// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backup_convertion;
mod some_config_action;
mod some_listing_action;
mod restore;
use lazy_static::lazy_static;
use serde_json;
use some_config_action::*;
use std::path::PathBuf;
use std::path::{Path};
use std::sync::{RwLock};
use tauri_plugin_shell::ShellExt;

// Static config state
lazy_static! {
    static ref CONFIG_PATH: RwLock<Option<PathBuf>> = RwLock::new(None);
}

fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // <-- ADD THIS BLOCK
            init_config_path(&app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            test,
            open_konsole,
            exit_app,
            list_saves,
            list_backup,
            do_backup,
            get_config_value,
            save_config,
            load_config,
            do_restore,
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
fn get_config_value(key: &str) -> String {
    some_config_action::get_config_value(key)
}

#[tauri::command]
fn save_config(config: serde_json::Value) -> Result<(), String> {
    some_config_action::save_config(config)
}

#[tauri::command]
fn load_config() -> Result<serde_json::Value, String> {
    some_config_action::load_config().map_err(|e| e.to_string())
}

#[tauri::command]
fn exit_app(handle: tauri::AppHandle) {
    handle.exit(0); // 0 = normal exit code
}

fn __settings() {}

// 1 List saves to backup (Raw)
#[tauri::command]
fn list_backup() -> Result<Vec<(String, String)>, String> {
    some_listing_action::list_backup()
}

#[tauri::command]
fn list_saves() -> Result<Vec<(String, String)>, String> {
    some_listing_action::list_saves()
}

// 2 Backup save (Raw)
#[tauri::command]
fn do_backup(save_name: String) -> Result<(), String> {
    backup_convertion::do_backup(save_name)
}
// 3 Restore backup (Raw)
#[tauri::command]
fn do_restore(save_name: String) -> Result<(), String> {
    restore::do_restore(save_name)
}
// 4 Restoring backup logic (Raw)
