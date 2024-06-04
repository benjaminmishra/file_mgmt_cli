pub trait ParsableCommand {
    fn parse_from_str(str_vec : Vec<String>)->Self;
}