use regex::Regex;

use crate::git;


pub fn is_on_development_branch() -> bool {
    let app_branch_name_regex: Regex = Regex::new("^(patch|minor|major)/[^ ]+$").unwrap();
    let current_branch: &str = &*git::get_current_branch_name();
    
    return app_branch_name_regex.is_match(current_branch);
}