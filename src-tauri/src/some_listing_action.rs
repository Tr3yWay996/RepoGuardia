use crate::some_config_action::get_config_value;
use crate::Path;
use chrono::{DateTime, Local};
use walkdir::WalkDir;

pub fn list_saves() -> Result<Vec<(String, String)>, String> {
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
            let path = Path::new(&full_path);
            let folder_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let datetime: DateTime<Local> = modified.into();
            let formatted = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
            (folder_name, formatted)
        })
        .collect();

    Ok(result)
}

pub fn list_backup() -> Result<Vec<(String, String)>, String> {
    let destination_base_path = get_config_value("destination_base_path");
    let base_path = Path::new(&destination_base_path);
    let mut dirs: Vec<(String, std::time::SystemTime)> = Vec::new();

    for entry in WalkDir::new(&destination_base_path)
        .min_depth(2)
        .max_depth(2)
    {
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
                .strip_prefix(base_path)
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|_| {
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string()
                });

            let datetime: DateTime<Local> = modified.into();
            let formatted = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
            (folder_name, formatted) // Return ("REPO_SAVE_...", "03-02-2026...")
        })
        .collect();

    Ok(result)
}
