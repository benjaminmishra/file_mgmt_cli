// rm - remove
// ls - list (done)
// mv - move
// cp - copy

use std::env;

use commands::{ls, mv::{self, MoveItemArgs}, parsable_command::{self, ParsableCommand}, rm};

mod commands;
mod args_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command_name : String;
    let commad_args : T;
    let parse_result = args_parser::parse_args::<MoveItemArgs>(args);
            match parse_result {
                Ok((cmd_name, comd_args)) => {
                    command_name = cmd_name;
                    
                }
            }

    // all commands
    match first_arg {
        "ls" => {
            if args.len() < 3 {
                println!("Please provide a valid path");
                return;
            }
            let second_arg: &str = args[2].as_ref();

            if second_arg.is_empty() {
                println!("Please provide a valid path");
                return;
            }

            let _ = ls::list_all_dir_contents(second_arg);
        }
        "rm" => {
            if args.len() < 3 {
                println!("Please provide a valid path");
                return;
            }
            let second_arg: &str = args[2].as_ref();
            if second_arg.is_empty() {
                println!("Please provide a valid path");
                return;
            }
            let _ = rm::remove_item(second_arg);
        }
        "mv" => {
            

            if args.len() < 4 {
                println!("Please provide valid source and destination paths.");
                return;
            }

            let second_arg: &str = args[2].as_ref();
            if second_arg.is_empty() {
                println!("Please provide a valid source path");
                return;
            }

            let third_arg: &str = args[3].as_ref();
            if third_arg.is_empty() {
                println!("Please provide a valid destination path");
                return;
            }

            let _ = mv::move_items(command_args);
        }
        _ => println!("Invalid command supplied"),
    }
}
