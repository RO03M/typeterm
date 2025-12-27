pub fn yellow(string: &str) -> String {
    return format!("\x1B[33;1m{}\x1B[0m", string);
}

pub fn red(string: &str) -> String {
    let color = "\x1B[0;31m";
    return format!("{}{}\x1B[0m", color, string);
}

pub fn red_hi(string: &str) -> String {
    let color = "\x1B[0;91m";
    return format!("{}{}\x1B[0m", color, string);
}

pub fn red_bold(string: &str) -> String {
    let color = "\x1B[1;31m";
    return format!("{}{}\x1B[0m", color, string);
}