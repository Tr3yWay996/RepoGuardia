#[allow(unused_imports)]
use crate::some_config_action::{get_config_value};
use std::path::PathBuf;
pub fn do_backup(save_name: String) -> Result<(), String> {
    let id = chrono::Local::now().format("%3f").to_string();
    let full_save_path = PathBuf::from(get_config_value("default_saves_path")).join(&save_name);
    //let default_saves_path = get_config_value("destination_base_path");
    let default_backup_place = get_config_value("destination_base_path");
    let backup_folder = format!("{}.{}", save_name, id);
    let full_backup_path = format!("{}/{}", default_backup_place, backup_folder );
    let backup_root = PathBuf::from(&full_backup_path).join(&save_name);
/*     println!("id value is: {}", id);
    println!("default_saves_path value is: {}", default_saves_path);
    println!("save_name value is: {}", save_name);
    println!("full_save_path value is: {}", full_save_path.display());
    println!("default_backup_place value is: {}", default_backup_place);
    println!("backup_folder value is: {}", backup_folder);
    println!("full_backup_path value is: {}", full_backup_path);
 */    let backup_path = PathBuf::from(&full_backup_path);
/*     println!("backup_path value is: {}", backup_path.display());
 */    std::fs::create_dir_all(&backup_root).map_err(|e| e.to_string())?;
 /*    println!("backup_root value is: {}", backup_root.display()); */

    let metadata = serde_json::json!({
        "original_name": save_name,
        "backup_name": backup_folder,
        "created_at": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "label": ""
    });
    let metadata_path = backup_path.join("backup_metadata.json");
    let metadata_str = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    std::fs::write(&metadata_path, metadata_str).map_err(|e| e.to_string())?;
/*     println!("Metadata saved to: {}", metadata_path.display());
 */    dircpy::copy_dir(full_save_path, &backup_root).map_err(|e| e.to_string())?;
    Ok(())
}
