use std::process::exit;

pub mod help;
pub mod args;
pub mod git;
pub mod interfaces;


fn main() {
    if git::is_installed() == false {
        panic!("git not found (requires global binary access)");
    }

    let command: &str = &*args::parse_positional(0).expect("Missing command");
    if command == "help" {
        println!("{}", help::TEXT);
        exit(0);
    }
    
    match command {
        "patch" | "minor" | "major" => interfaces::dev_start::run(command),
        "complete" => interfaces::dev_complete::run(),
        "redeclare" => interfaces::dev_redeclare::run(),

        "status" => interfaces::util_status::run(),

        _ => panic!("Unknown command")
    }
}