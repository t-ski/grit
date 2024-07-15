use std::{process::{Command, Output}, str::Lines};
use regex::Regex;

use crate::exit;


fn exec(args: Vec<&str>) -> Output {
    return Command::new("git")
    .args(args)
    .output()
    .unwrap();
}

fn run(args: Vec<&str>) -> String {
    let result: Output = exec(args);
    if result.status.success() == false {
        let mut error_message: String = String::from_utf8(result.stderr)
        .unwrap_or(String::from("Unknown error"));
        if error_message.len() == 0 {
            error_message = String::from("Unknown error");
        }
        exit::exit_with_error(&*error_message);
    }
    return String::from(&*String::from_utf8(result.stdout).unwrap().trim());
}


pub fn is_installed() -> bool {
    return exec(vec![ "-v" ]).status.success();
}

pub fn get_main_branch_name() -> String {
    let branch_list: &str = &*run(vec![ "branch" ]);
    let mut branches: Lines = branch_list.lines();
    return String::from(str::replace(branches.next().unwrap(), "* ", ""));
}

pub fn get_current_branch_name() -> String {
    return run(vec![ "rev-parse", "--abbrev-ref", "HEAD" ]);
}

pub fn create_branch(branch_name: &str) -> String {
    return run(vec![ "branch", branch_name ]);
}

pub fn checkout_branch(branch_name: &str) -> String {
    return run(vec![ "checkout", branch_name ]);
}

pub fn delete_branch(branch_name: &str) -> String {
    return run(vec![ "branch", "-d", branch_name ]);
}

pub fn rename_branch(old_branch_name: &str, new_branch_name: &str) -> String {
    let current_branch_name: &str = &get_current_branch_name();
    let temp_branch_name: &str = &*format!("_{}", old_branch_name);
    
    create_branch(temp_branch_name);
    checkout_branch(temp_branch_name);
    
    let git_output: String = run(vec![ "branch", "-m", old_branch_name, new_branch_name ]);
    
    if current_branch_name == old_branch_name {
        checkout_branch(new_branch_name);
    } else {
        checkout_branch(current_branch_name);
    }
    delete_branch(temp_branch_name);
    
    return git_output;
}

pub fn stage_files(files_path: &str) -> String {
    return run(vec![ "add", files_path ]);
}

pub fn commit(commit_message: &str) -> String {
    return run(vec![ "commit", "-m", &*format!("\"{}\"", commit_message) ]);
}

pub fn merge(source_branch_name: &str, target_branch_name: &str) -> String {
    checkout_branch(target_branch_name);
    return run(vec![ "merge", source_branch_name ]);
}

pub fn get_latest_tag() -> String {
    let tags_list: &str = &*run(vec![ "tag" ]);
    let version_tag_regex: Regex = Regex::new("^v[0-9]+(\\.[0-9]+){2}$").unwrap();
    let mut tags: Lines = tags_list.lines();
    let mut version: &str;
    loop {
        let next_tag: Option<&str> = tags.next_back();
        if next_tag == None {
            version = "v0.0.0";
            break;
        }
        version = next_tag.unwrap();
        if version_tag_regex.is_match(version) {
            break;
        }
    }
    return String::from(version);
}

pub fn create_tag(version: &str, tag_name: &str) -> String {
    return run(vec![ "tag", "-a", &*format!("v{}", version), "-m", tag_name ]);
}

pub fn get_status() -> String {
    return run(vec![ "status" ]);
}