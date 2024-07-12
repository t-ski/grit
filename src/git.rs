use std::process::{Command, Output};


fn exec(args: Vec<&str>) -> Output {
    return Command::new("git")
    .args(args)
    .output()
    .unwrap();
}


pub fn exists() -> bool {
    return exec(vec![ "-v" ]).status.success();
}

pub fn run(args: Vec<&str>) -> Result<String, String> {
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