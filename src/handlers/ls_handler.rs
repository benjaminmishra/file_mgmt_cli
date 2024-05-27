use std::fs;

pub struct LsHandler;

pub struct LsCommand<'a> {
    path: &'a str
}

impl<'a> LsHandler{

    fn handle<LsCommand>(self: &'a Self, command: &LsCommand) -> Result<(), &str> {
        
        if command.path.is_empty() {
            return Err("Invlaid path");
        }

        let read_dir_result = fs::read_dir(command.path);

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
            Err(err) => println!("Failed to read {} due to error {}", command.path, err.to_string()),
        }

        Ok(())
    }
}
