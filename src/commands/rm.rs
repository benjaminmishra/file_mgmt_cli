use std::{fs, path::Path};

pub fn remove_item(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("Invlaid path".to_owned());
    }

    let path_ref = Path::new(path);

    if path_ref.is_dir() {
        match fs::remove_dir(path_ref) {
            Ok(()) => return Ok(()),
            Err(_) => return Err("Failed to delete provided dir due to Error".to_owned()),
        }
    } else {
        match fs::remove_file(path_ref) {
            Ok(()) => return Ok(()),
            Err(_) => return Err("Failed to delete provided file due to Error {}".to_owned()),
        }
    }
}
