use std::{
    fs::{self, remove_dir, remove_file},
    os::unix::fs::MetadataExt,
    path::Path,
};

use super::parsable_command::ParsableCommand;


pub struct MoveItemArgs<'a>{
    source_path : &'a str,
    destination_path : &'a str
}

impl<'a> ParsableCommand for MoveItemArgs<'a> {
    fn parse_from_str(str_vec : Vec<String>) -> MoveItemArgs<'a> {
        return MoveItemArgs {
            source_path : str_vec[0].as_ref(),
            destination_path : str_vec[1].as_ref()
        }
    }
}

pub fn move_items(args: MoveItemArgs) -> Result<(), String> {
    let from = Path::new(args.source_path);
    let to = Path::new(args.destination_path);

    if from == to {
        return Err("The source and destination cannot be the same".to_owned());
    }

    let from_metadata_result = fs::metadata(from).ok();
    let file_size: u64;

    match from_metadata_result {
        Some(metadata) => {
            file_size = metadata.size();
        },
        None => {
            return Err("Failed to obtain metadata of the source item".to_owned());
        }
    }

    match fs::copy(from, to) {
        Ok(copied_file_size) => {
            if copied_file_size != file_size {
                return Err("Moved file size inconsistent".to_owned());
            } else {
                let _ = if from.is_dir() { remove_dir(from) } else { remove_file(from) };
                return Ok(());
            }
        },
        Err(_) => {
            return Err("Failed to move file".to_owned());
        }
    }
}
