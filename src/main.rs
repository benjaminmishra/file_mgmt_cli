// rm - remove
// ls - list (done)
// mv - move
// cp - copy

use std::env;

use commands::{ls, mv, rm};

mod commands;
mod args_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("You need to supply altleast one command");
        return;
    }

    let first_arg: &str = args[1].as_ref();

    if first_arg.is_empty() {
        println!("First argument cannot be empty");
        return;
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

            let _ = mv::move_items(second_arg, second_arg);
        }
        _ => println!("Invalid command supplied"),
    }
}
