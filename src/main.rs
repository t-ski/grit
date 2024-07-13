use std::process::exit;

pub mod _help;
pub mod args;
pub mod git;
pub mod interfaces;


fn main() {
    if git::exists() == false {
        panic!("git not found (requires global binary access)");
    }

    let command: Option<String> = args::parse_positional(0);
    if command == None {
        panic!("Missing command");
    }

    let command_key: String = command.unwrap();
    if command_key == "help" {
        println!("{}", _help::TEXT);
        exit(0);
    }
    
    match &*command_key {
        "example" => interfaces::example::run(),
        _ => panic!("Unknown command")
    }
}