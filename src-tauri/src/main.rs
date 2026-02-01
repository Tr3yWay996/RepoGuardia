// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_shell::ShellExt;

fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, test, open_konsole])
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
        .args(&["-e", "bash -c 'echo Hello from Tauri; exec fish'"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}