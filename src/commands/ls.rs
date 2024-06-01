use std::{fs, path::Path};

pub fn list_all_dir_contents(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("Invlaid path".to_owned());
    }
    let path_obj = Path::new(path);
    if path_obj.is_file() {
        return Err("Cannot list contents of a file".to_owned());
    }

    let read_dir_result = fs::read_dir(path_obj);

    match read_dir_result {
        Ok(read_dir_instance) => {
            for item in read_dir_instance {
                match item {
                    Ok(dir) => println!("{}", dir.file_name().to_string_lossy()),
                    Err(error) => {
                        println!("failed to read dir entry error {}", error.to_string())
                    }
                }
            }
        }
        Err(err) => println!(
            "Failed to read {} due to error {}",
            path,
            err.to_string()
        ),
    }

    Ok(())
}
