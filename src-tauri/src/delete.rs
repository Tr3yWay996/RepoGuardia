#[allow(unused_imports)]
use crate::some_config_action::{get_config_value, get_custom_config_value};
use std::path::PathBuf;
use std::fs;

pub fn do_delete(save_name: String) -> Result<(), String> {
    let default_saves_path = get_config_value("default_saves_path");
    let destination_base_path = get_config_value("destination_base_path");
    let backup_root = PathBuf::from(&destination_base_path).join(&save_name);
    let backup_parent = backup_root.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from(&destination_base_path));
    let metadata_file = backup_parent.join("backup_metadata.json");
    
    println!("\n\n\n\n\n\n\n\n\n");
    println!("\ndefault_saves_path value is: {}", default_saves_path);
    println!("\nsave_name value is: {}", save_name);
    println!("\ndestination_base_path value is: {}", destination_base_path);
    println!("\nbackup_root value is : {}", backup_root.display());
    println!("\nbackup_parent value is : {}", backup_parent.display());
    println!("\nLe 'metadata_file' value is this: {}", metadata_file.display());

    let original_save_name = get_custom_config_value(metadata_file, "original_name");
    std::fs::remove_dir_all(&backup_parent).map_err(|e| e.to_string())?;
    Ok(())
}


