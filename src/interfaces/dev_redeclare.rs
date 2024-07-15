use crate::{args, exit, git};


pub fn run() {
    if !super::dev_utils::is_on_development_branch() {
        exit::exit_with_error("You are currently not on a grit development branch");
    }

    let new_declaration: &str = &*args::parse_positional(1).expect("");
    match new_declaration {
        "major" | "minor" | "patch" => {},
        _ => exit::exit_with_error(&*format!("Not a Semver declaration {}", new_declaration))
    }

    let current_branch_name: &str = &*git::get_current_branch_name();
    let current_branch_name_parts: Vec<&str> = current_branch_name.split("/").collect::<Vec<&str>>();
    let current_declaration: &str = current_branch_name_parts[0];
    let current_name: &str = current_branch_name_parts[1];

    if new_declaration == current_declaration {
        exit::exit_with_grace("Keeping same development declaration");
    }

    let new_branch_name: &str = &*format!("{}/{}", new_declaration, current_name);

    git::rename_branch(current_branch_name, new_branch_name);

    println!("Development of {} redeclared (from {}).", new_branch_name, current_branch_name);
}