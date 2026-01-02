use std::io::Stdout;

use crossterm::{
    cursor::{MoveRight, MoveTo},
    queue,
    style::Print,
};

use crate::{
    config::Mode,
    display::page_by_words,
    page::get_page_from_input,
    session::Session,
    text::{compare_strings, wc},
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
            cell_y: if matches!(session.mode, Mode::Timer(_) | Mode::Word(_)) {
                1
            } else {
                0
            },
        };
    }

    pub fn build(&self, stdout: &mut Stdout) {
        let match_text = compare_strings(self.paged_input.as_str(), self.paged_phrase.as_str());
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
