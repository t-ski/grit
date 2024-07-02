use std::process::{Command, Output};
use std::io::Error;


pub fn exec(bin: &str, args: Vec<&str>) -> Result<Output, Error> {
    return Command::new(bin)
    .args(args)
    .output();
}