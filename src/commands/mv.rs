use std::{
    fs::{self, remove_dir, remove_file},
    os::unix::fs::MetadataExt,
    path::Path,
};

pub fn move_items(source_path: &str, destination_path: &str) -> Result<(), String> {
    let from = Path::new(source_path);
    let to = Path::new(destination_path);

    if from == to {
        return Err("The source and destination cannot be the same".to_owned());
    }

    let from_metadata_result = fs::metadata(from).ok();
    let mut file_size: u64 = 0;

    match from_metadata_result {
        Some(metadata) => {
            file_size = metadata.size();
        }
        None => {}
    }

    match fs::copy(from, to) {
        Ok(copied_file_size) => {
            if copied_file_size != file_size {
                return Err("Moved file size inconsistent".to_owned());
            } else {
                if from.is_dir() {
                    let _ = remove_dir(from);
                } else {
                    let _ = remove_file(from);
                }
                return Ok(());
            }
        }
        Err(_) => {
            return Err("Failed to move file".to_owned());
        }
    }
}
