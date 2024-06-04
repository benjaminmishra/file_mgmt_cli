use crate::commands::parsable_command::ParsableCommand;

pub fn parse_args<T: ParsableCommand>(arguments: Vec<String>) -> Result<(String, T), String> {
    let mut command_name: String;
    let mut command: T;
    let args_length = arguments.len();

    // assume first item is command
    if args_length < 2 {
        panic!("Unable to parse as no command passed");
    }

    if arguments[0].is_empty() {
        return Err("Empty Command passed".to_owned());
    }

    command_name = arguments[0].to_string();

    // rest
    command = T::parse_from_str(arguments[1..].to_vec());

    return Ok((command_name, command));
}
