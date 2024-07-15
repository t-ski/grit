pub fn run() {
    if !super::dev_utils::is_on_development_branch() {
        panic!("You are currently not on a grit development branch");
    }
    
    // TODO
    // let new_purpose: String = args::parse_positional(1).expect("");
}