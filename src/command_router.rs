use std::collections::HashMap;

use crate::handlers::command_handler::Command;

pub struct CommandRouter<'a> {
    is_initalized: bool,
    command_registry : HashMap<&'a str,fn(&dyn Command)->Result<(), &str>>
}

impl CommandRouter<'_>{
    // call this function to initiate the router
    fn new(&mut self) {
        self.command_registry = HashMap::new();
        self.is_initalized = true;
    }

    fn register_command<T>(&mut self, command_name:&str, handler_fn:fn(&dyn Command)->Result<(), &str>){
        if self.is_initalized == false {
            Err("No command handler registered yet");
        }
        self.command_registry.insert(command_name, handler_fn);
    }
}