use crate::{exit, git};


pub fn run() {
    if !super::dev_utils::is_on_development_branch() {
        exit::exit_with_error("You are currently not on a grit development branch");
    }
    
    let current_branch_name: &str = &*git::get_current_branch_name();
    git::checkout_branch(&*git::get_main_branch_name());
    git::delete_branch(current_branch_name);
}