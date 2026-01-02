use std::io::Stdout;

use crossterm::{
    cursor::{MoveRight, MoveTo},
    queue,
    style::Print,
};

use crate::{
    colors::{background_red, red_bold, yellow},
    config::Mode,
    display::page_by_words,
    page::get_page_from_input,
    session::Session,
    text::wc,
};

#[derive(PartialEq)]
pub struct View {
    header: String,
    paged_input: String,
    paged_phrase: String,
    next_paged_phrase: String,
    cell_y: u16,
}

impl View {
    pub fn new(session: &Session) -> View {
        let inline_word_count: usize = 10;

        let current_page = get_page_from_input(&session.input, inline_word_count);

        let paged_input = page_by_words(session.input.clone(), inline_word_count, current_page);
        let paged_phrase = page_by_words(session.phrase(), inline_word_count, current_page);
        let next_page = if wc(paged_phrase.as_str()) < inline_word_count {
            "".to_string()
        } else {
            page_by_words(session.phrase(), inline_word_count, current_page + 1)
        };

        return View {
            header: session.header(),
            paged_input: paged_input.to_string(),
            paged_phrase: paged_phrase.to_string(),
            next_paged_phrase: next_page.to_string(),
            cell_y: if matches!(session.mode, Mode::Timer(_) | Mode::Word) {
                1
            } else {
                0
            },
        };
    }

    pub fn build(&self, stdout: &mut Stdout) {
        let match_text = build_colored_text(self.paged_input.as_str(), self.paged_phrase.as_str());
        let next_phrase = self.next_paged_phrase.clone();

        let _ = queue!(
            stdout,
            Print(self.header.clone()),
            Print("\n\r"),
            Print(match_text),
            Print("\n\r"),
            Print(next_phrase),
        );

        if self.paged_input.len() > 0 {
            let _ = queue!(
                stdout,
                MoveTo(0, self.cell_y),
                MoveRight(self.paged_input.chars().count().try_into().unwrap_or(0))
            );
        } else {
            let _ = queue!(stdout, MoveTo(0, self.cell_y));
        }
    }
}

pub fn build_colored_text(input: &str, target: &str) -> String {
    let input_parts: Vec<&str> = input.split(" ").collect();
    let target_parts: Vec<&str> = target.split(" ").collect();

    let mut final_string = String::new();

    for (index, target_part) in target_parts.iter().enumerate() {
        let input_part = input_parts.get(index).copied().unwrap_or("");

        final_string.push_str(colorize_words(input_part, target_part).as_str());
        final_string.push(' ');
    }

    return final_string;
}

pub fn colorize_words(input: &str, target: &str) -> String {
    let mut compared_word = String::new();

    for (i, char) in target.chars().enumerate() {
        let input_char = input.chars().nth(i).unwrap_or('\0');

        if input_char == '\0' {
            compared_word.push(char);
            continue;
        }

        if input_char == char {
            compared_word.push_str(yellow(input_char.to_string().as_str()).as_str());
        }

        if input_char != char {
            compared_word.push_str(red_bold(input_char.to_string().as_str()).as_str());
        }
    }

    if input.len() > target.len() {
        let rest = &input[target.len()..];
        compared_word.push_str(background_red(rest).as_str());
    }

    return compared_word;
}
