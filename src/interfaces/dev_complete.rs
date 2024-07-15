use crate::{args, exit, git};


pub fn run() {
    if !super::dev_utils::is_on_development_branch() {
        exit::exit_with_error("You are currently not on a grit development branch");
    }

    git::stage_files("./");
    git::commit("Complete development cycle");  // TODO: Version as commit message

    let target_branch_name_name: &str = &*git::get_current_branch_name();
    let target_branch: &str = &*args::parse_option("target", Some("t"))
    .unwrap_or(String::from("main"));
    let merge_output: String = git::merge(target_branch_name_name, target_branch);
    print!("{}", merge_output);
    if !args::parse_flag("keep-branch", None) {
        git::delete_branch(target_branch_name_name);
    }
    
    let development_declaration: &str = target_branch_name_name.split("/").collect::<Vec<&str>>()[0];
    let version_increment_index: usize = match development_declaration {
        "major" => 0,
        "minor" => 1,
        "patch" => 2,
        _  => panic!("Broken development cycle")
    };
    let latest_version_tag: String = git::get_latest_tag();
    let mut latest_version_parts: Vec<&str> = latest_version_tag[1..].split(".").collect();
    let version_increment_part: i32 = latest_version_parts[version_increment_index].parse::<i32>().unwrap() + 1;
    let version_increment: &str = &*version_increment_part.to_string();
    latest_version_parts[version_increment_index] = version_increment;
    for i in (version_increment_index + 1)..(2 + 1) {
        latest_version_parts[i as usize] = "0";
    }
    git::create_tag(&*latest_version_parts.join("."), development_declaration);

    println!("Development of {} completed.", target_branch_name_name);
}