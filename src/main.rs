mod colors;
mod config;
mod display;
mod page;
mod session;
mod text;
mod view;

use crate::{session::Session, view::View};
use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    event::{Event, KeyCode, KeyModifiers, poll, read},
    execute, queue,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::{
    io::{Error, Write, stdout},
    time::Duration,
};

fn render() -> Result<(), Error> {
    let mut stdout = stdout();
    let config = config::Config::new();
    let mut session = Session::new(config.mode, config.text.clone());

    let _ = execute!(stdout, EnterAlternateScreen, SetCursorStyle::BlinkingBlock);

    loop {
        let _ = queue!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        if session.input.len() > 0 && !session.is_started() {
            session.start();
        }

        if session.should_end() {
            print!("\n");
            break;
        }

        let view = View::new(&session);
        view.build(&mut stdout);

        let _ = stdout.flush();

        if poll(Duration::from_millis(500)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match (event.code, event.modifiers) {
                    (KeyCode::Char('w'), m) | (KeyCode::Backspace, m)
                        if m.contains(KeyModifiers::CONTROL) =>
                    {
                        session.input.pop();
                        while let Some(c) = session.input.chars().last() {
                            if c.is_whitespace() {
                                break;
                            }

                            session.input.pop();
                        }
                    }
                    (KeyCode::Backspace, _) => {
                        session.input.pop();
                    }
                    (KeyCode::Esc, _) => {
                        break;
                    }
                    (KeyCode::Char(' '), _) => {
                        if session.input.chars().last().unwrap_or(' ') != ' ' {
                            session.input.push(' ');
                        }
                    }
                    (KeyCode::Char(c), _) => {
                        session.input.push(c);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    session.end();

    let _ = execute!(
        stdout,
        LeaveAlternateScreen,
        SetCursorStyle::DefaultUserShape
    );

    println!("\rWPM: {}", session.wpm());

    Ok(())
}

fn calculate_wpm(input: &str, target: &str, time: Duration) -> f32 {
    let mut correct_chars_count = 0;

    let target_parts: Vec<&str> = target.split(" ").collect();
    let input_parts: Vec<&str> = input.split(" ").collect();

    for (i, target_part) in target_parts.iter().enumerate() {
        let input_part = input_parts.get(i).copied().unwrap_or("");

        for (i, target_char) in target_part.chars().enumerate() {
            let input_char = input_part.chars().nth(i).unwrap_or('\0');
            if target_char == input_char {
                correct_chars_count += 1;
            }
        }
    }

    let minutes = time.as_secs_f32() / 60.0;

    if minutes == 0.0 {
        return 0.0;
    }

    return (correct_chars_count as f32 / 5.0) / minutes;
}

fn main() -> Result<(), ()> {
    let _ = enable_raw_mode();

    if let Err(e) = render() {
        eprintln!("Error: {e}");
    }

    let _ = disable_raw_mode();

    println!();

    Ok(())
}
