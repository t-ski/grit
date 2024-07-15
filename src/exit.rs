use std::process::exit;


pub fn exit_with_grace(message: &str) {
    println!("{}", message);
    exit(0);
}

pub fn exit_with_error(message: &str) {
    eprintln!("{}", message);
    exit(1);
}