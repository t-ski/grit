use std::process::{Command, Output};


fn exec(args: Vec<&str>) -> Output {
    return Command::new("git")
    .args(args)
    .output()
    .unwrap();
}


fn run(args: Vec<&str>) -> Result<String, String> {
    let result = exec(args);
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