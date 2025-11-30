use std::{io, process::exit};
use clearscreen::{self, clear};
use walkdir::WalkDir;
use dircpy::{self, CopyBuilder};
use chrono;
use serde_json;

static CONFIG_PATH: &str = "config.json";

fn load_config() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let config_contents = std::fs::read_to_string(CONFIG_PATH)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;
    Ok(config)
}

fn get_config_value(key: &str) -> String {
    load_config()
        .ok()
        .and_then(|config| config.get(key).and_then(|v| v.as_str()).map(|s| s.to_string()))
        .unwrap_or_else(|| "".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        clear().expect("Failed to clear screen at main menu");
        println!("Available commands:\n1 (lists all backed up saves)\nexit (quit the program)\n2 (copy a save folder from the list)\n3 (restore backup)"); // main menu
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");

        if action.trim() == "1" {
            clear().expect("Failed to clear screen at option 1");
            println!("Those are all the backed up saves.");
            
            // List all save folders
            println!("\nThose are all the save folders available:");
            let default_saves_path = get_config_value("default_saves_path");
            let mut dirs: Vec<String> = Vec::new();
            for entry in WalkDir::new(&default_saves_path)
                .min_depth(1)
                .max_depth(1)
            {
                let entry = entry?;
                if entry.file_type().is_dir() {
                    dirs.push(entry.path().display().to_string());
                }
            }

            // Display numbered list
            for (index, dir) in dirs.iter().enumerate() {
                let metadata = std::fs::metadata(dir)?;
                let modified = metadata.modified()?;
                let datetime: chrono::DateTime<chrono::Local> = modified.into();
                println!("{}: {} last modified: {}", index + 1, dir, datetime.format("%Y-%m-%d %H:%M:%S"));
            }

            println!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("Failed to clear screen at option 1");
        }
        if action.trim() == "2" {
            clear().expect("Failed to clear screen at option 2");

            // Load configuration
            let mut config = load_config().unwrap_or_else(|_| serde_json::json!({}));
            let last_version = get_config_value("last_version");

            // Ask user if they want to use the last version
            println!("Last used version was: {}\nUse it? (y/n)", last_version);
            let mut use_last = String::new();
            io::stdin()
                .read_line(&mut use_last)
                .expect("Failed to read input");
            
            let version = if use_last.trim().eq_ignore_ascii_case("y") || use_last.trim().is_empty() {
                last_version.clone()
            } else {
                println!("What is the version of REPO?");
                let mut version_input = String::new();
                io::stdin()
                    .read_line(&mut version_input)
                    .expect("Failed to read version");
                version_input.trim().to_string()
            };
            

            // Create destination path
            let destination_base_path = get_config_value("destination_base_path");
            let version_path = std::path::Path::new(&destination_base_path).join(&version);
            
            config["last_version"] = serde_json::Value::String(version.clone());
            std::fs::write(CONFIG_PATH, serde_json::to_string_pretty(&config)?)?;

            println!("Using version directory: {}", version_path.display());

            // List all save folders
            println!("\nThose are all the save folders available:");
            let default_saves_path = get_config_value("default_saves_path");
            let mut dirs: Vec<String> = Vec::new();
            for entry in WalkDir::new(&default_saves_path)
                .min_depth(1)
                .max_depth(1)
            {
                let entry = entry?;
                if entry.file_type().is_dir() {
                    dirs.push(entry.path().display().to_string());
                }
            }
            // Display numbered list
            for (index, dir) in dirs.iter().enumerate() {
                let metadata = std::fs::metadata(dir)?;
                let modified = metadata.modified()?;
                let datetime: chrono::DateTime<chrono::Local> = modified.into();
                println!("{}: {} last modified: {}", index + 1, dir, datetime.format("%Y-%m-%d %H:%M:%S"));
            }

            // Get user selection
            println!("\nEnter the number of the directory you want to copy:");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");

            if let Ok(num) = choice.trim().parse::<usize>() {
                if num > 0 && num <= dirs.len() {
                    let selected_path = &dirs[num - 1];
                    println!("You selected: {}", selected_path);

                    // Copy logic
                    let dir_name = std::path::Path::new(selected_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("backup");
                    // let uuid = uuid::Uuid::new_v4(); If i want to use UUIDs instead of timestamps, unlikely tho
                    let timestamp = chrono::Local::now().format("%3f").to_string();
                    let original_name = dir_name;
                    let backup_name = format!("{}.{}", original_name, timestamp);
                    let backup_folder = version_path.join(&backup_name);
                    let full_destination = version_path.join(format!("{}.{}", original_name, timestamp)).join(format!("{}", dir_name));
                    
                    dircpy::copy_dir(selected_path, &full_destination)?;
                    println!("Copied to: {}", full_destination.display());

                    // Create metadata file
                    let metadata = serde_json::json!({
                        "original_name": original_name,
                        "backup_name": backup_name,
                        "created_at": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        "label": ""
                    });
                    let metadata_path = backup_folder.join("backup_metadata.json");
                    std::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;
                    println!("Metadata saved to: {}", metadata_path.display());
                } else {
                    println!("Invalid selection. Please choose a number between 1 and {}", dirs.len());
                }
            } else {
                println!("Invalid input. Please enter a number.");
            }

            println!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("failed to clear screen");
        }
        if action.trim() == "3" {
            clear().expect("Failed to clear screen at option 2");

            // Load configuration
            let mut config = load_config().unwrap_or_else(|_| serde_json::json!({}));
            let last_version = config.get("last_version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| String::from("0.3.0"));

            // Ask user if they want to use the last version
            println!("Last used version was: {}\nUse it? (y/n)", last_version);
            let mut use_last = String::new();
            io::stdin()
                .read_line(&mut use_last)
                .expect("Failed to read input");
            
            let version = if use_last.trim().eq_ignore_ascii_case("y") || use_last.trim().is_empty() {
                last_version.clone()
            } else {
                println!("What is the version of REPO?");
                let mut version_input = String::new();
                io::stdin()
                    .read_line(&mut version_input)
                    .expect("Failed to read version");
                version_input.trim().to_string()
            };
            

            // Create destination path
            let destination_base_path = get_config_value("destination_base_path");
            let version_path = std::path::Path::new(&destination_base_path).join(&version);
            
            config["last_version"] = serde_json::Value::String(version.clone());
            std::fs::write(CONFIG_PATH, serde_json::to_string_pretty(&config)?)?;

            println!("Using version directory: {}", version_path.display());

            // List all backup folders (the timestamped ones)
            println!("\nThose are all the backed up saves available:");
            let default_saves_path = get_config_value("default_saves_path");
            let mut dirs: Vec<String> = Vec::new();
            for entry in WalkDir::new(&destination_base_path)
                .min_depth(2)
                .max_depth(2)
            {
                let entry = entry?;
                if entry.file_type().is_dir() {
                    // Only include folders that have a backup_metadata.json
                    let metadata_check = entry.path().join("backup_metadata.json");
                    if metadata_check.exists() {
                        dirs.push(entry.path().display().to_string());
                    }
                }
            }
            // Display numbered list
            for (index, dir) in dirs.iter().enumerate() {
                // Read metadata to show original name and created date
                let metadata_path = std::path::Path::new(dir).join("backup_metadata.json");
                if let Ok(contents) = std::fs::read_to_string(&metadata_path) {
                    if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&contents) {
                        let original = meta.get("original_name").and_then(|v| v.as_str()).unwrap_or("?");
                        let created = meta.get("created_at").and_then(|v| v.as_str()).unwrap_or("?");
                        println!("{}: {} (created: {})", index + 1, original, created);
                    }
                }
            }

            // Get user selection
            println!("\nEnter the number of the backup to restore:");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");

            if let Ok(num) = choice.trim().parse::<usize>() {
                if num > 0 && num <= dirs.len() {
                    let backup_folder = &dirs[num - 1];
                    println!("You selected to restore: {}", backup_folder);
                    
                    // Read metadata from the backup folder
                    let metadata_path = std::path::Path::new(backup_folder).join("backup_metadata.json");
                    let metadata_contents = std::fs::read_to_string(&metadata_path)?;
                    let metadata: serde_json::Value = serde_json::from_str(&metadata_contents)?;
                    let original_name = metadata.get("original_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    
                    // The save folder is inside the backup folder with the original name
                    let save_folder = std::path::Path::new(backup_folder).join(original_name);
                    let full_destination = std::path::Path::new(&default_saves_path).join(&original_name);
                    
                    if full_destination.exists() {
                        std::fs::remove_dir_all(&full_destination)?;
                        println!("Deleted existing save at: {}", full_destination.display());
                    }
                    // Copy the save folder to destination
                    dircpy::copy_dir(&save_folder, &full_destination)?;
                    println!("Restored to: {}", full_destination.display());
                } else {
                    println!("Invalid selection. Please choose a number between 1 and {}", dirs.len());
                }
            } else {
                println!("Invalid input. Please enter a number.");
            }

            println!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("failed to clear screen");
        }
        if action.trim() == "exit" {
            clear().expect("Failed to clear screen at exit");
            println!("Exiting program. Goodbye!");
            exit(0);
        }
    }
}


    
    //if menu_choise.trim().eq_ignore_ascii_case("dirwalk") {
    //    println!("What you wanna scan");
    //    let mut path = String::new();
    //    io::stdin()
    //        .read_line(&mut path)
    //        .expect("Reading path failed");
    //
    //    for entry in WalkDir::new(path.trim()) 
    //        .min_depth(1)
    //        .max_depth(1)
    //
    //    {
    //        let entry = entry?;
    //        if entry.file_type().is_dir() {
    //            println!("{}", entry.path().display());
    //        }
    //    }
    //}

    //if menu_choise.trim().eq_ignore_ascii_case("test") {
    //    println!("Please enter some text:");
    //
    //    loop {
    //        let mut test = String::new();
    //        io::stdin()
    //            .read_line(&mut test)
    //            .expect("Failed to read line");
    //        println!("You entered: {}", test.trim()); // le return de l'input, tant que c'est pas "exit"
    //        if test.trim().eq_ignore_ascii_case("exit") { // logique pour handle un break si l'input est "exit"
    //            clear().expect("failed to clear screen"); // donc ça c'est pour, bah supr tout sur le term
    //            break;
    //    }   
    //    }
    //}