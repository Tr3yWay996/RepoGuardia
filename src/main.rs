use std::io;
use clearscreen::{self, clear};
use walkdir::WalkDir;
use dircpy;
use chrono;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        clear().expect("failed to clear screen");
        println!("Available commands:\n1 (lists all save folders in default game save files path)\nexit (quit the program)\n2 (copy a save folder from the list)");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read menu choice");
        
        if action.trim().eq_ignore_ascii_case("1") {
            clear().expect("failed to clear screen");
            println!("Those are all the save folders available.");
            // Collect directories into a vector
            let mut dirs: Vec<String> = Vec::new();
            let default_saves_path = "C:/Users/Admin/AppData/LocalLow/semiwork/REPO/saves";
            for entry in WalkDir::new(default_saves_path)
                .min_depth(1)
                .max_depth(1)
            {
                let entry = entry?;
                if entry.file_type().is_dir(){
                    dirs.push(entry.path().display().to_string());
                }
            }

            // Display the directories
            for dir in &dirs {
                println!("{}", dir);
            }

            println!("\nPress Enter to return to main menu...");
            let mut _pause = String::new();
            io::stdin().read_line(&mut _pause).ok();
            clear().expect("failed to clear screen");
        }
        if action.trim().eq_ignore_ascii_case("2") {
            clear().expect("failed to clear screen");

            // Load or create configuration file
            let config_path = "config.json";
            let mut last_version = String::from("0.3.0");
            
            // Try to load existing config
            if let Ok(config_contents) = std::fs::read_to_string(config_path) {
                if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_contents) {
                    if let Some(version) = config.get("last_version").and_then(|v| v.as_str()) {
                        last_version = version.to_string();
                    }
                }
            }

            // Ask user if they want to use the last version
            println!("Last used version was: {}\nUse it? (y/n)", last_version);
            let mut use_last = String::new();
            io::stdin()
                .read_line(&mut use_last)
                .expect("Failed to read input");
            
            let version = if use_last.trim().eq_ignore_ascii_case("y") {
                last_version.clone()
            } else {
                println!("What is the version of REPO?");
                let mut version_input = String::new();
                io::stdin()
                    .read_line(&mut version_input)
                    .expect("Failed to read version");
                version_input.trim().to_string()
            };

            // Save the version to config
            let config = serde_json::json!({
                "last_version": version
            });
            std::fs::write(config_path, serde_json::to_string_pretty(&config)?)?;

            // Create destination path
            let destination_base_path = "C:/Users/Admin/GameBackups/rust";
            let version_path = std::path::Path::new(destination_base_path).join(&version);
            
            if !version_path.exists() {
                std::fs::create_dir_all(&version_path)?;
                println!("Created new version directory: {}", version_path.display());
            }
            
            println!("Using version directory: {}", version_path.display());

            // List all save folders
            println!("\nThose are all the save folders available:");
            let mut dirs: Vec<String> = Vec::new();
            let default_saves_path = "C:/Users/Admin/AppData/LocalLow/semiwork/REPO/saves";
            for entry in WalkDir::new(default_saves_path)
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
                println!("{}: {}", index + 1, dir);
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
                    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S%.3f").to_string();
                    let full_destination = version_path.join(format!("{}.{}", dir_name, timestamp));
                    
                    dircpy::copy_dir(selected_path, &full_destination)?;
                    println!("Copied to: {}", full_destination.display());
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
        //if action.trim().eq_ignore_ascii_case("json-test") {
        //    let config = serde_json::json!({
        //        "last_version": version
        //    });
        //}
        else if action.trim().eq_ignore_ascii_case("exit") {
            clear().expect("failed to clear screen");
            println!("Exiting program. Goodbye!");
            break;
            }
        }
    Ok(())
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