pub mod args;
pub mod git;
pub mod interfaces;


fn main() {
    if git::exists() == false {
        panic!("git not found (requires global binary access)");
    }
    if args::parse_positional(0) == None {
        panic!("Missing command");
    }

    match &*args::parse_positional(0).unwrap() {
        "example" => interfaces::example::run(),
        _ => panic!("Unknown command")
    }
}