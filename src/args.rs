fn get_index(name: &str, shorthand: Option<&str>) -> Option<usize> {
    let args: Vec<String> = std::env::args().collect();
    let index = args.iter().position(|s| *s.to_lowercase() == format!("--{}", name.to_lowercase()));
    let shorthand_index = args
        .iter()
        .position(|s| *s.to_lowercase() == format!("-{}", shorthand.unwrap_or("").to_lowercase()));
    return match index != None || shorthand_index != None {
        true => Some(std::cmp::min(index.unwrap_or(args.len()), shorthand_index.unwrap_or(args.len()))),
        false => None
    };
}

pub fn parse_positional(index: usize) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if index >= args.len() - 1 {
        return None;
    }
    return Some(args[index + 1].to_string());
}

pub fn parse_flag(name: &str, shorthand: Option<&str>) -> bool {
    if get_index(name, shorthand) == None {
        return false;
    }
    return true;
}

pub fn parse_option(name: &str, shorthand: Option<&str>) -> Option<String> {
    let index: Option<usize> = get_index(name, shorthand);
    if index == None {
        return None;
    }
    let args: Vec<String> = std::env::args().collect();
    let option_index: usize = index.unwrap() + 1;
    if option_index >= args.len() {
        return None;
    }
    return Some(args[option_index].to_string());
}