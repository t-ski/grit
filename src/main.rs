pub mod args;
pub mod git;


fn main() {
    if git::exists() == false {
        panic!("git not found (requires global binary access)");
    }

    dbg!(args::parse_positional(0));
}