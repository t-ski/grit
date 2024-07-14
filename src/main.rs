use std::process::exit;

pub mod help;
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
        println!("{}", help::TEXT);
        exit(0);
    }
    
    match &*command_key {
        "patch" => interfaces::patch::run(),
        "minor" => interfaces::minor::run(),
        "major" => interfaces::major::run(),

        "complete" => interfaces::complete::run(),

        "status" => interfaces::status::run(),

        _ => panic!("Unknown command")
    }
}