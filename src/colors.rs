enum Color {
    BoldBlack,
    BoldRed,
    BoldGreen,
    BoldYellow,
    BoldBlue,
    BoldPurple,
    BoldCyan,
    BoldWhite
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::BoldBlack => "\x1B[1;30m",
            Color::BoldRed => "\x1B[1;31m",
            Color::BoldGreen => "\x1B[1;32m",
            Color::BoldYellow => "\x1B[1;33m",
            Color::BoldBlue => "\x1B[1;34m",
            Color::BoldPurple => "\x1B[1;35m",
            Color::BoldCyan => "\x1B[1;36m",
            Color::BoldWhite => "\x1B[1;37m"
        }
    }
}

fn with_color(color: &str, text: &str) -> String {
    return format!("{}{}\x1B[0m", color, text);
}

pub fn yellow(string: &str) -> String {
    return with_color(Color::BoldYellow.as_str(), string);
}

pub fn red(string: &str) -> String {
    return with_color("\x1B[0;31m", string);
}

pub fn background_red(string: &str) -> String {
    return with_color("\x1B[1;41m", string)
}

pub fn red_bold(string: &str) -> String {
    return with_color(Color::BoldRed.as_str(), string)
}
