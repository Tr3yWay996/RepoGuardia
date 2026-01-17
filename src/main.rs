// Alright so i added some comments, for 1: Myself and two: for you to understand hopefully my thought process on how i built this lil program
#![allow(unused_macros)]
#![allow(unused_imports)]

use chrono;
use clearscreen::{self, clear};
use colored_text::Colorize;
use ctrlc;
use dircpy;
use lazy_static::lazy_static;
use serde_json;
use std::sync::RwLock;
use std::{io, process::exit};
use tinterm::{Color, Gradient};
use walkdir::WalkDir;

lazy_static! {
    static ref CONFIG_PATH: RwLock<String> = RwLock::new(String::from("config-togo.json"));
}
// Color preset for error messages, warnings, informations, and sucess / confirmation messages
macro_rules! print_error {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).red())
    };
}
macro_rules! print_info {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).blue())
    };
}
macro_rules! print_warn {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).yellow())
    };
}
macro_rules! print_success {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).green())
    };
}
// Color macro for primary, secondary, tertiary message type
macro_rules! print_primary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 194, 159))
    };
}
macro_rules! print_secondary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 172, 125))
    };
}
macro_rules! print_tertiary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 149, 90))
    };
}
// Color macro do just about any RGB or HEX color possible ever to be used in the final executable
macro_rules! print_cyan {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).cyan())
    };
}
macro_rules! print_purple {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(208, 0, 255))
    };
}

fn string_parse_test(_primary: &str, _cyan: &str, _warn: &str, _error: &str) {
    clear().expect("yo phone is linging");
    print_primary!("{}", _primary);
    print_cyan!("{}", _cyan);
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).ok();
}

fn config() {
    //clear().expect("Emotional damage");
    let togo = "togo";
    print_info!(
        "What da conf doin\nType '{}' for the on-drive test config and press enter for the default main one!",
        togo
    );
    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("Faaaaa");

    let mut path = CONFIG_PATH.write().unwrap();
    if action.trim() == "togo" {
        *path = String::from("config-togo.json");
    } else {
        *path = String::from("config.json");
    }
}

// Premade TUI menu text with tinterm gradient color
fn print_main_menu() {
    let multiline = "Available commands:\n1 (lists all backed up saves)\n2 (copy a save folder from the list)\n3 (restore backup)\n4 (Delete backup)\nexit (quit the program)";
    println!(
        "{}",
        multiline.gradient(Color::CYAN, Color::MAGENTA, Some(true))
    );
}

fn load_config() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let path_guard = CONFIG_PATH.read().unwrap();
    let config_contents = std::fs::read_to_string(&*path_guard)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;
    Ok(config)
}

fn get_config_value(key: &str) -> String {
    load_config()
        .ok()
        .and_then(|config| {
            config
                .get(key)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let interrupted = Arc::new(AtomicBool::new(false));
    //let flag = interrupted.clone();

    //ctrlc::set_handler(move || {
    //    flag.store(true, Ordering::SeqCst);
    //    print_warn!("Ctrl+C detected, returning to main menu...");
    //})?;
    //config();
    loop {
        clear().expect("Failed to clear screen at main menu");
        print_main_menu(); // main menu
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");

        if action.trim() == "1" {
            clear().expect("Failed to clear screen at option 1");
            print_info!("Those are all saves that you have in backup.");

            // List all save folders
            let destination_base_path = get_config_value("destination_base_path");
            let mut dirs: Vec<String> = Vec::new();
            for entry in WalkDir::new(&destination_base_path)
                .min_depth(2)
                .max_depth(2)
            {
                let entry = entry?;
                if entry.file_type().is_dir() {
                    dirs.push(entry.path().display().to_string());
                }
            }

            // Display numbered list, was a lil bit harder but i got it with some stackoverflow help
            for (index, dir) in dirs.iter().enumerate() {
                let metadata = std::fs::metadata(dir)?;
                let modified = metadata.modified()?;
                let datetime: chrono::DateTime<chrono::Local> = modified.into();
                println!(
                    "{}: {} last modified: {}",
                    index + 1,
                    dir,
                    datetime.format("%Y-%m-%d %H:%M:%S")
                );
            }
            print_secondary!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("Failed to clear screen at option 1");
        }
        if action.trim() == "2" {
            clear().expect("Failed to clear screen at option 2");

            // Load configuration, logic that i'll reuse a lot of time, most likelly :3
            let mut config = load_config().unwrap_or_else(|_| serde_json::json!({}));
            let last_version = get_config_value("last_version");

            // Ask user if they want to use the last version, cuz what if they got an update or smsh
            let version = if last_version.is_empty() {
                println!("No last version found, please enter the curent game version bollow:");
                let mut version_input = String::new();
                io::stdin()
                    .read_line(&mut version_input)
                    .expect("Failed to read version");
                version_input.trim().to_string()
            } else {
                print_warn!("Last used version was: {}\nUse it? (y/n)", last_version);
                let mut use_last = String::new();
                io::stdin()
                    .read_line(&mut use_last)
                    .expect("Failed to read input");
                if use_last.trim().eq_ignore_ascii_case("y") || use_last.trim().is_empty() {
                    last_version.clone()
                } else {
                    print_info!("What is the version of REPO?");
                    let mut version_input = String::new();
                    io::stdin()
                        .read_line(&mut version_input)
                        .expect("Failed to read version");
                    version_input.trim().to_string()
                }
            };

            // Create destination path
            let destination_base_path = get_config_value("destination_base_path");
            let version_path = std::path::Path::new(&destination_base_path).join(&version);

            config["last_version"] = serde_json::Value::String(version.clone());
            let config_path = CONFIG_PATH.read().unwrap();
            std::fs::write(&*config_path, serde_json::to_string_pretty(&config)?)?;

            print_success!("Using version directory: {}", version_path.display());

            // List all save folders
            print_info!("\nThose are all the save folders available:");
            let default_saves_path = get_config_value("default_saves_path");
            let mut dirs: Vec<String> = Vec::new();
            for entry in WalkDir::new(&default_saves_path).min_depth(1).max_depth(1) {
                let entry = entry?;
                if entry.file_type().is_dir() {
                    dirs.push(entry.path().display().to_string());
                }
            }
            // Display numbered list, again
            for (index, dir) in dirs.iter().enumerate() {
                let metadata = std::fs::metadata(dir)?;
                let modified = metadata.modified()?;
                let datetime: chrono::DateTime<chrono::Local> = modified.into();
                println!(
                    "{}: {} last modified: {}",
                    index + 1,
                    dir,
                    datetime.format("%Y-%m-%d %H:%M:%S")
                );
            }

            // Get user selection
            print_info!("\nEnter the number of the directory you want to copy:");
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");
            if let Ok(num) = choice.trim().parse::<usize>() {
                if num > 0 && num <= dirs.len() {
                    let selected_path = &dirs[num - 1];
                    print_success!("You selected: {}", selected_path);

                    // Copy logic, nice
                    let dir_name = std::path::Path::new(selected_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("backup");
                    // let uuid = uuid::Uuid::new_v4(); If i want to use UUIDs instead of timestamps, unlikely tho, but who knows what my future self have in mind ¯\_(ツ)_/¯
                    let timestamp = chrono::Local::now().format("%3f").to_string();
                    let uuid = uuid::Uuid::new_v4();
                    let full_message = format!("UUID is: {}", uuid);
                    let colored = full_message.gradient(
                        Color::new(255, 15, 123), // Start color
                        Color::new(248, 155, 41), // End color
                        None,                     // No block mode (continuous gradient)
                    );
                    println!("{}", colored);
                    let original_name = dir_name;
                    let backup_name = format!("{}.{}", original_name, timestamp);
                    let backup_folder = version_path.join(&backup_name);
                    let full_destination = version_path
                        .join(format!("{}.{}", original_name, timestamp))
                        .join(format!("{}", dir_name));
                    print_info!(
                        "Trying to copy {} to {}",
                        selected_path.to_string(),
                        full_destination.display()
                    );
                    dircpy::copy_dir(selected_path, &full_destination)?;
                    print_success!("Copied to: {}", full_destination.display());

                    // Create metadata file, hey that's a new one, i didnt implemented the damn label thingy yet lol, but fine for now
                    let metadata = serde_json::json!({
                        "original_name": original_name,
                        "backup_name": backup_name,
                        "created_at": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        "label": ""
                    });
                    let metadata_path = backup_folder.join("backup_metadata.json");
                    std::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;
                    print_primary!("Metadata saved to: {}", metadata_path.display());
                } else {
                    println!(
                        "Invalid selection. Please choose a number between 1 and {}",
                        dirs.len()
                    );
                }
            } else {
                println!("Invalid input. Please enter a number.");
            }

            print_secondary!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("failed to clear screen");
        }
        if action.trim() == "3" {
            clear().expect("Failed to clear screen at option 2");

            // Load configuration
            let mut config = load_config().unwrap_or_else(|_| serde_json::json!({}));
            let last_version = config
                .get("last_version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| String::from("0.3.0"));

            // Ask user if they want to use the last version
            print_warn!("Last used version was: {}\nUse it? (y/n)", last_version);
            let mut use_last = String::new();
            io::stdin()
                .read_line(&mut use_last)
                .expect("Failed to read input");

            let version = if use_last.trim().eq_ignore_ascii_case("y") || use_last.trim().is_empty()
            {
                last_version.clone()
            } else {
                println!("What is the version of REPO?");
                let mut version_input = String::new();
                io::stdin()
                    .read_line(&mut version_input)
                    .expect("Failed to read version");
                version_input.trim().to_string()
            };

            // Create destination path, because who doenst love reusing code am i right
            let destination_base_path = get_config_value("destination_base_path");
            let version_path = std::path::Path::new(&destination_base_path).join(&version);

            config["last_version"] = serde_json::Value::String(version.clone());
            let config_path = CONFIG_PATH.read().unwrap();
            std::fs::write(&*config_path, serde_json::to_string_pretty(&config)?)?;

            print_info!("Using version directory: {}", version_path.display());

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
            // Display numbered list, again
            for (index, dir) in dirs.iter().enumerate() {
                // Read metadata to show original name and created date
                let metadata_path = std::path::Path::new(dir).join("backup_metadata.json");
                if let Ok(contents) = std::fs::read_to_string(&metadata_path) {
                    if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&contents) {
                        let original = meta
                            .get("original_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        let created = meta
                            .get("created_at")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        println!("{}: {} (created: {})", index + 1, original, created);
                    }
                }
            }

            // Get user selection
            print_info!("\nEnter the number of the backup to restore:");
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");

            if let Ok(num) = choice.trim().parse::<usize>() {
                if num > 0 && num <= dirs.len() {
                    let backup_folder = &dirs[num - 1];
                    print_success!("You selected to restore: {}", backup_folder);
                    // Read metadata from the backup folder
                    let metadata_path =
                        std::path::Path::new(backup_folder).join("backup_metadata.json");
                    let metadata_contents = std::fs::read_to_string(&metadata_path)?;
                    let metadata: serde_json::Value = serde_json::from_str(&metadata_contents)?;
                    let original_name = metadata
                        .get("original_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    // The save folder is inside the backup folder with the original name, checked
                    let backup_save_folder =
                        std::path::Path::new(backup_folder).join(original_name);
                    let full_destination =
                        std::path::Path::new(&default_saves_path).join(&original_name);
                    print_warn!(
                        "This is going to delete the existing save at: {}\nPlease confirm with 'y'",
                        full_destination.display()
                    );
                    let mut deletion_confirmation = String::new();
                    io::stdin()
                        .read_line(&mut deletion_confirmation)
                        .expect("Failed to retrieve user deletion confirmation");
                    if deletion_confirmation.trim() == "y" {
                        print_purple!(
                            "Debug info, full_dstination {}, deletion_confirmation: {}, backup_save_folder: {}, backup_folder: {}, original_name: {}",
                            full_destination.display(),
                            deletion_confirmation,
                            backup_save_folder.display(),
                            backup_folder,
                            original_name
                        );
                        print_tertiary!("Arived at std::fs::remove_dir_all(&full_destination)?");
                        if full_destination.exists() {
                            std::fs::remove_dir_all(&full_destination)?;
                        }
                        print_tertiary!("passed std::fs::remove_dir_all(&full_destination)?");
                        print_warn!("Deleted existing save at: {}", full_destination.display());
                        // Copy the save folder to destination, checked
                        print_tertiary!(
                            "Arived at dircpy::copy_dir(&backup_save_folder, &backup_save_folder)?"
                        );
                        dircpy::copy_dir(&backup_save_folder, &full_destination)?;
                        print_tertiary!(
                            "passed dircpy::copy_dir(&backup_save_folder, &backup_save_folder)?"
                        );
                        print_success!("Restored to: {}", full_destination.display());
                    } else {
                        print_error!("Operation canceled")
                    }
                } else {
                    print_error!(
                        "Invalid selection. Please choose a number between 1 and {}",
                        dirs.len()
                    );
                }
            } else {
                print_error!("Invalid input. Please enter a number.");
            }

            print_secondary!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("failed to clear screen");
        }
        if action.trim() == "4" {
            clear().expect("Failed to clear screen at option 4");

            // Load configuration
            let mut config = load_config().unwrap_or_else(|_| serde_json::json!({}));
            let last_version = config
                .get("last_version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| String::from("0.3.0"));

            // Ask user if they want to use the last version
            print_warn!("Last used version was: {}\nUse it? (y/n)", last_version);
            let mut use_last = String::new();
            io::stdin()
                .read_line(&mut use_last)
                .expect("Failed to read input");

            let version = if use_last.trim().eq_ignore_ascii_case("y") || use_last.trim().is_empty()
            {
                last_version.clone()
            } else {
                println!("What is the version of REPO?");
                let mut version_input = String::new();
                io::stdin()
                    .read_line(&mut version_input)
                    .expect("Failed to read version");
                version_input.trim().to_string()
            };

            // Create destination path, because who doenst love reusing code am i right
            let destination_base_path = get_config_value("destination_base_path");
            let version_path = std::path::Path::new(&destination_base_path).join(&version);

            config["last_version"] = serde_json::Value::String(version.clone());
            let config_path = CONFIG_PATH.read().unwrap();
            std::fs::write(&*config_path, serde_json::to_string_pretty(&config)?)?;

            print_info!("Using version directory: {}", version_path.display());

            // List all backup folders (the timestamped ones)
            println!("\nThose are all the backed up saves available:");
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
            // Display numbered list, again
            for (index, dir) in dirs.iter().enumerate() {
                // Read metadata to show original name and created date
                let metadata_path = std::path::Path::new(dir).join("backup_metadata.json");
                if let Ok(contents) = std::fs::read_to_string(&metadata_path) {
                    if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&contents) {
                        let original = meta
                            .get("original_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        let created = meta
                            .get("created_at")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        println!("{}: {} (created: {})", index + 1, original, created);
                    }
                }
            }

            // Get user selection
            print_info!("\nEnter the number of the backup to delete:");
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");

            if let Ok(num) = choice.trim().parse::<usize>() {
                if num > 0 && num <= dirs.len() {
                    let backup_folder = &dirs[num - 1];
                    print_success!("You selected to delete: {}", backup_folder);

                    // Read metadata from the backup folder
                    let metadata_path =
                        std::path::Path::new(backup_folder).join("backup_metadata.json");
                    let metadata_contents = std::fs::read_to_string(&metadata_path)?;
                    let metadata: serde_json::Value = serde_json::from_str(&metadata_contents)?;
                    let original_name = metadata
                        .get("original_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    // The save folder is inside the backup folder with the original name, checked
                    let backup_save_folder =
                        std::path::Path::new(backup_folder).join(original_name);
                    print_purple!(
                        "save folder is {} and is made using the backup folder {} combined with its original name: {}",
                        backup_save_folder.display(),
                        backup_folder,
                        original_name
                    );
                    print_warn!(
                        "This is going to delete the existing save backup at: {}\nPlease confirm with 'y'",
                        backup_folder
                    );
                    let mut deletion_confirmation = String::new();
                    io::stdin()
                        .read_line(&mut deletion_confirmation)
                        .expect("Failed to retrieve user deletion confirmation");
                    if deletion_confirmation.trim() == "y" {
                        std::fs::remove_dir_all(&backup_folder)?;
                        print_warn!("Deleted existing save backup at: {}", backup_folder);
                    } else {
                        print_error!("Backup deletion canceled")
                    }
                } else {
                    print_error!(
                        "Invalid selection. Please choose a number between 1 and {}",
                        dirs.len()
                    );
                }
            } else {
                print_error!("Invalid input. Please enter a number.");
            }

            print_secondary!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("failed to clear screen");
        }
        if action.trim() == "exit" {
            clear().expect("Failed to clear screen at exit");
            print_primary!("Exiting program. Goodbye!");
            exit(0);
        }
        if action.trim() == "change config" {
            config();
        }
        if action.trim() == "test" {
            string_parse_test("your bed", " is nice", "f", "f");
        }
    }
}

// Don't mind that commented out code bellow too much, it was my first times with dirwalk and stuff like serde_json, keept it for reference, and also as a history of my discoveries :3
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
