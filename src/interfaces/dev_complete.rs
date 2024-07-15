use crate::{args, git};


pub fn run() {
    if !super::dev_utils::is_on_development_branch() {
        panic!("You are currently not on a grit development branch");
    }

    // TODO: Add incremental tag
    // let latest_tag: String = git::get_latest_tag();
    
    git::stage_files("./");
    git::commit("Complete development cycle");  // TODO: Version as commit message
    
    /* let source_branch: &str = &*git::get_current_branch_name();
    let target_branch: &str = &*args::parse_option("target", Some("t"))
    .unwrap_or(String::from("main"));
    
    let merge_output: String = git::merge(source_branch, target_branch);
    print!("{}", merge_output);

    if !args::parse_flag("keep", None) {
        git::delete_branch(source_branch);
    }
    
    println!("Development of {} completed.", source_branch); */
}