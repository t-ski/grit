pub mod exit;
pub mod help;
pub mod args;
pub mod git;
pub mod interfaces;


fn main() {
    if git::is_installed() == false {
        exit::exit_with_error("git not found (requires global binary access)");
    }

    let command_arg: Option<String> = args::parse_positional(0);
    if command_arg == None {
        exit::exit_with_error("Missing command");
    }
    let command: &str = &*command_arg.unwrap();
    if command == "help" {
        exit::exit_with_grace(help::TEXT);
    }
    
    match command {
        "patch" | "minor" | "major" => interfaces::dev_start::run(command),
        "complete" => interfaces::dev_complete::run(),
        "abort" => interfaces::dev_abort::run(),
        "redeclare" => interfaces::dev_redeclare::run(),

        "status" => interfaces::util_status::run(),

        _ => {
            exit::exit_with_error("Unknown command");
        }
    }
}