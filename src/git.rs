use std::{process::{Command, Output}, str::Lines};
use regex::Regex;


fn exec(args: Vec<&str>) -> Output {
    return Command::new("git")
    .args(args)
    .output()
    .unwrap();
}

fn run(args: Vec<&str>) -> Result<String, String> {
    let result: Output = exec(args);
    if result.status.success() == false {
        return Err(String::from("git command failed"));
    }
    let stdout = String::from_utf8(result.stdout);
    match stdout {
        Ok(msg) => Ok(msg),
        Err(_) => Err(String::from("Command result unreadable"))
    }
}


pub fn exists() -> bool {
    return run(vec![ "-v" ]).is_ok();
}

pub fn create_and_witch_branch(branch_name: &str) {
    run(vec![ "switch", "-c", branch_name ])
    .expect("Could not create and switch branch");
}

pub fn add_and_commit(add_files_path: &str, commit_message: &str) {
    run(vec![ "add", add_files_path ])
    .expect("Could not add files");
    run(vec![ "commit", "-m", commit_message ])
    .expect("Could not commit");
}

pub fn merge(source_branch_name: &str, target_branch_name: &str) {
    run(vec![ "checkout", target_branch_name ])
    .expect("Could not checkout target branch");
    run(vec![ "merge", source_branch_name ])
    .expect("Could not merge source branch");
}

pub fn delete_branch(branch_name: &str) {
    run(vec![ "branch", "-d", branch_name ])
    .expect("Could not delete branch");
}

pub fn latest_tag() -> String {
    let tags_list: &str = &*run(vec![ "tag" ]).expect("Could not read tags");
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

pub fn create_tag(version: &str, tag_name: &str) {
    run(vec![ "tag", "-a", &*format!("v{}", version), "-m", tag_name ])
    .expect("Could not create tag");
}