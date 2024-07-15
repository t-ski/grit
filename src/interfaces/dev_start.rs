use std::time::{UNIX_EPOCH, SystemTime};

use crate::{args, exit, git};


pub fn run(branch_prefix: &str) {
    if super::dev_utils::is_on_development_branch() {
        exit::exit_with_error("You are already on a grit development branch");
    }

    let name: String = args::parse_option("name", Some("n"))
    .unwrap_or(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
    let branch_name: &str = &*format!("{}/{}", branch_prefix, name);
    
    git::create_branch(branch_name);
    git::checkout_branch(branch_name);
    
    println!("Development of {} started.", branch_name);
}