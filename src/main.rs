use std::io;
use clearscreen;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Choose what you wanna run\nREPO!");
    let mut action = String::new();
    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read menu choice");
    if action.trim().eq_ignore_ascii_case("list_saves") {
        clearscreen::clear().expect("failed to clear screen");
        let default_saves_path = "C:/Users/Admin/AppData/LocalLow/semiwork/REPO/saves";
        for entry in WalkDir::new(default_saves_path.trim())
            .min_depth(1)
            .max_depth(1)
        {
            let entry = entry?;
            if entry.file_type().is_dir(){
                println!("{}", entry.path().display());
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
    //            clearscreen::clear().expect("failed to clear screen"); // donc ça c'est pour, bah supr tout sur le term
    //            break;
    //    }   
    //    }
    //}
    
    Ok(())
}


