pub mod cmd;


pub fn exists() -> bool {
    let cmd_v = cmd::exec("git", vec![ "-v" ]);
    if cmd_v.is_err() == true {
        panic!("{}", cmd_v.unwrap_err());
    }
    return cmd_v.unwrap().stdout.len() > 0;
}