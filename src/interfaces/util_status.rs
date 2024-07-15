use crate::git;


pub fn run() {
    let current_branch: String = git::get_current_branch_name();
    
    println!("You are on {}", current_branch);
}