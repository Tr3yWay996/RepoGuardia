// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backup_convertion;
mod some_config_action;
mod some_listing_action;
mod restore;
mod delete;
mod rename;
use lazy_static::lazy_static;
use serde_json;
use some_config_action::*;
use std::path::PathBuf;
use std::path::{Path};
use std::sync::{RwLock};

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
            init_config_path(&app.handle());
            ensure_config_defaults();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            exit_app,
            list_saves,
            list_backup,
            do_backup,
            get_config_value,
            save_config,
            load_config,
            do_restore,
            do_delete,
            do_rename,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
    handle.exit(0);
}

fn __settings() {}

// 1 List saves to backup (Revamp)
#[tauri::command]
fn list_backup() -> Result<Vec<(String, String, String)>, String> {
    some_listing_action::list_backup()
}

#[tauri::command]
fn list_saves() -> Result<Vec<(String, String)>, String> {
    some_listing_action::list_saves()
}

// 2 Backup save (Revamp)
#[tauri::command]
fn do_backup(save_name: String) -> Result<(), String> {
    backup_convertion::do_backup(save_name)
}
// 3 Restore backup (Revamp)
#[tauri::command]
fn do_restore(save_name: String) -> Result<(), String> {
    restore::do_restore(save_name)
}
// 4 Delete backup (New)
#[tauri::command]
fn do_delete(save_name: String) -> Result<(), String> {
    delete::do_delete(save_name)
}
// 5 Rename backup (New)
#[tauri::command]
fn do_rename(save_name: String, new_label: String) -> Result<(), String> {
    rename::do_rename(save_name, new_label)
}
