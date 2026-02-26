#[allow(unused_imports)]
use crate::some_config_action::{get_config_value, get_custom_config_value};
use std::path::PathBuf;

pub fn do_rename(save_name: String, new_label: String) -> Result<(), String> {
    let destination_base_path = get_config_value("destination_base_path");
    let backup_root = PathBuf::from(&destination_base_path).join(&save_name);
    let backup_parent = backup_root.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from(&destination_base_path));
    let metadata_file = backup_parent.join("backup_metadata.json");

    if !metadata_file.exists() {
        return Err(format!("Metadata file not found: {}", metadata_file.display()));
    }

    let contents = std::fs::read_to_string(&metadata_file).map_err(|e| e.to_string())?;
    let mut metadata: serde_json::Value = serde_json::from_str(&contents).map_err(|e| e.to_string())?;

    metadata["label"] = serde_json::Value::String(new_label.clone());

    let updated = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    std::fs::write(&metadata_file, updated).map_err(|e| e.to_string())?;

    Ok(())
}