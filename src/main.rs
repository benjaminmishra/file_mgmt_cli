// rm - remove
// ls - list (done)
// mv - move
// cp - copy

use std::env;

mod handlers;
mod command_router;

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
            let _ = handlers::ls_handler::handle(&args[2]).unwrap();
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