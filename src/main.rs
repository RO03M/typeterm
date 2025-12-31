mod colors;
mod config;
mod session;
mod words;
mod display;
mod page;

use std::{io::{Error, stdout}, time::Duration};
use crossterm::{cursor::{MoveRight, MoveTo}, event::{Event, KeyCode, KeyModifiers, poll, read}, execute, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}};
use crate::{config::Mode, display::page_by_words, page::get_page_from_input, session::Session, words::compare_strings};

fn render() -> Result<(), Error> {
    let mut stdout = stdout();
    let config = config::Config::new();
    let mut session = Session::new(config.mode, config.text.clone());
    let cell_y = if matches!(session.mode, Mode::Timer(_)) { 1 } else { 0 };
    println!("celly: {cell_y}");
    loop  {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        
        session.print_mode_header();
        
        let current_page = get_page_from_input(&session.input, 10);
        // println!("current_page: {current_page}");
        let paged_input = page_by_words(session.input.clone(), 10, current_page);
        let paged_phrase = page_by_words(session.phrase(), 10, current_page);
        let next_page = page_by_words(session.phrase(), 10, current_page + 1);
        
        let match_text = compare_strings(paged_input.as_str(), paged_phrase.as_str());
        print!("\r{}\n\r{}", match_text, next_page);
        
        if session.input.len() > 0 && !session.is_started() {
            session.start();
        }

        if session.should_end() {
            print!("\n");
            break;
        }
        
        // #[cfg(debug_assertions)]
        // {
        //     println!("\n\rpage: {current_page}");
        //     println!("\rcursor_x: {}", paged_input.chars().count().try_into().unwrap_or(0));
        //     println!("\rraw_input: {}", session.input);
        //     println!("\rpaged_input: {paged_input}");
        // }
        
        if paged_input.len() > 0 {
            let _ = execute!(stdout, MoveTo(0, cell_y), MoveRight(paged_input.chars().count().try_into().unwrap_or(0)));
        } else {
            let _ = execute!(stdout, MoveTo(0, cell_y));
        }
        
        
        if poll(Duration::from_millis(500)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match (event.code, event.modifiers) {
                    (KeyCode::Char('w'), m) | (KeyCode::Backspace, m) if m.contains(KeyModifiers::CONTROL) => {
                        session.input.pop();
                        while let Some(c) = session.input.chars().last() {
                            if c.is_whitespace() {
                                break;
                            }
                            
                            session.input.pop();
                        }                    
                    },
                    (KeyCode::Backspace, _) => {
                        session.input.pop();
                    },
                    (KeyCode::Esc, _) => {
                        break;
                    },
                    (KeyCode::Char(' '), _) => {
                        if session.input.chars().last().unwrap_or(' ') != ' ' {
                            session.input.push(' ');
                        }
                    },
                    (KeyCode::Char(c), _) => {
                        session.input.push(c);
                    },
                    _ => {}
                }
                _ => {}
            }
        }
        
    }
    
    session.end();

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
    
    return (correct_chars_count as f32 / 5.0) / minutes;
}

fn main() -> Result<(), ()> {
    let _ = enable_raw_mode();
    
    if let Err(e) = render() {
        eprintln!("Error: {e}");
    }
    
    let _ = disable_raw_mode();
 
    println!();
 
    // println!("{}", get_page_from_input("with still can word may call up mean say keep year one however the both do small long stand if ", 10));
    
    // println!("{}", page_by_words("small school run could becom enation however develop from people because around where interest".to_string(), 10, 1));
    
    // let phrase = "since well both great little stuart".to_string();
    // let p1 = page_by_words(phrase.clone(), 3, 0);
    // let p2 = page_by_words(phrase.clone(), 3, 1);
    // let p3 = page_by_words(phrase.clone(), 3, 2);
    
    // println!("{p1}\n{p2}\n{p3}");
    
    Ok(())
}
