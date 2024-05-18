use std::{env, fs};

// rm - remove
// ls - list (done)
// mv - move
// cp - copy


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        print!("You need to supply altleast one command");
        return;
    }

    // all commands
    match args[1].as_ref() {
        "ls" => {
            if args.len() <= 2 {
                println!("Please provide a valid path");
                return;
            }
            let _ = list_all_items_in_dir(&args[2]).unwrap();
        }
        "rm" => {
            if args.len() <= 2 {
                println!("Please provide a valid path");
                return;
            }
        }
        _ => println!("Invalid command supplied"),
    }
}

fn list_all_items_in_dir(path: &str) -> Result<(), &str> {
    if path.is_empty() {
        return Err("Invlaid path");
    }

    let read_dir_result = fs::read_dir(path);

    match read_dir_result {
        Ok(read_dir_instance) => {
            for item in read_dir_instance {
                match item {
                    Ok(dir) => println!("{}", dir.file_name().to_string_lossy()),
                    Err(error) => println!("failed to read dir entry error {}", error.to_string()),
                }
            }
        }
        Err(err) => println!("Failed to read {} due to error {}", path, err.to_string()),
    }

    Ok(())
}

fn remove_items_from_dir(path: &str) -> Result<(), &str>{
    
}