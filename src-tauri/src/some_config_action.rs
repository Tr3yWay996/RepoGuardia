use lazy_static::lazy_static;
use std::fs;
use std::string::String;
use std::sync::RwLock;
use tauri::Manager;
use crate::PathBuf;

lazy_static! {
    static ref CONFIG_PATH: RwLock<Option<PathBuf>> = RwLock::new(None);
}

pub fn init_config_path(app_handle: &tauri::AppHandle) {
    let config_dir = app_handle
        .path()
        .app_config_dir()
        .expect("Failed to get config dir");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    let mut path = config_dir;
    #[cfg(target_os = "linux")]
    path.push("config-linux.json");
    #[cfg(target_os = "windows")]
    path.push("config-windows.json");
    let mut guard = CONFIG_PATH.write().unwrap();
    *guard = Some(path);
}
pub fn load_config() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let path_guard = CONFIG_PATH.read().unwrap();
    let path = path_guard.as_ref().ok_or("Config not initialized")?;
    let config_contents = std::fs::read_to_string(path)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;
    Ok(config)
}
pub fn save_config(config: serde_json::Value) -> Result<(), String> {
    let config_path = CONFIG_PATH.read().unwrap();
    let path = config_path.as_ref().ok_or("Config path not initialized")?;
    std::fs::write(path, serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
pub fn get_config_value(key: &str) -> String {
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
            config
                .get(key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default()
        }
        Err(e) => {
            eprintln!("CONFIG DEBUG: Failed to load config: {}", e);
            String::new()
        }
    }
}
pub fn get_custom_config_value(config: PathBuf, key: &str) -> String {
    eprintln!("CONFIG DEBUG: Looking for config at: {:?}", config);

    if !config.exists() {
        eprintln!("CONFIG DEBUG: File doesn't exist!");
        return String::new();
    }

    match std::fs::read_to_string(&config) {
        Ok(contents) => match serde_json::from_str::<serde_json::Value>(&contents) {
            Ok(json) => {
                eprintln!("CONFIG DEBUG: Config loaded: {:?}", json);
                json.get(key)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            }
            Err(e) => {
                eprintln!("CONFIG DEBUG: Failed to parse JSON: {}", e);
                String::new()
            }
        },
        Err(e) => {
            eprintln!("CONFIG DEBUG: Failed to read file: {}", e);
            String::new()
        }
    }
}

pub fn ensure_config_defaults() {
    let path_guard = CONFIG_PATH.read().unwrap();
    let path = match path_guard.as_ref() {
        Some(p) => p.clone(),
        None => return,
    };
    drop(path_guard);

    // Load existing config, or start with empty object if file doesn't exist yet
    let mut config: serde_json::Value = if path.exists() {
        let contents = std::fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&contents).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let mut dirty = false;

    // Each key: check if missing or empty, then set platform-appropriate default
    if config["default_saves_path"].as_str().unwrap_or("").is_empty() {
        #[cfg(target_os = "linux")]
        let default = "";  // hard to guess on linux, leaving empty
        #[cfg(target_os = "windows")]
        let default = format!(
            "C:\\Users\\{}\\AppData\\LocalLow\\semiwork\\Repo\\saves",
            std::env::var("USERNAME").unwrap_or("User".into())
        );
        config["default_saves_path"] = serde_json::Value::String(default.into());
        dirty = true;
    }

    if config["destination_base_path"].as_str().unwrap_or("").is_empty() {
        #[cfg(target_os = "linux")]
        let default = "";
        #[cfg(target_os = "windows")]
        let default = format!(
            "C:\\Users\\blahaj\\AppData\\Local\\com.tr3yway.repoguardia\\backups",
            std::env::var("USERNAME").unwrap_or("User".into())
        );
        config["destination_base_path"] = serde_json::Value::String(default.into());
        dirty = true;
    }

    if config["game_version"].as_str().unwrap_or("").is_empty() {
        config["game_version"] = serde_json::Value::String("".into());
        dirty = true;
    }

    // Only write back if something was actually missing
    if dirty {
        let _ = std::fs::write(&path, serde_json::to_string_pretty(&config).unwrap());
    }
}
