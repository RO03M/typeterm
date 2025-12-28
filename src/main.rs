use std::{io::{Error, stdout}, time::{Duration, Instant}};

use crossterm::{cursor::{MoveRight, MoveTo, MoveToRow}, event::{Event, KeyCode, KeyModifiers, poll, read}, execute, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}};

use crate::colors::{red, red_hi, yellow};

mod colors;
mod config;

fn print_parts(input: &str, target_text: &str) {
    let target_parts: Vec<&str> = target_text.split(" ").collect();
    let input_parts: Vec<&str> = input.split(" ").collect();
    
    for (index, target_part) in target_parts.iter().enumerate() {
        let input_part = input_parts.get(index).copied().unwrap_or("");
        
        print_word_match(input_part, target_part);
        print!(" ");
    }
    
    print!("\n\r");
}

fn render() -> Result<(), Error> {
    let mut stdout = stdout();
    let config = config::Config::new();
    
    let mut input = String::new();
    let full_text_parts: Vec<&str> = config.text.split(" ").collect();
    let target_word_count = full_text_parts.len();
    let last_word = full_text_parts.last().copied().unwrap_or("");
    
    let mut start: Option<Instant> = None;
    
    loop  {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        
        println!("\r{}", (Instant::now() - start.unwrap_or(Instant::now())).as_secs());
        
        print!("\r");
        print_parts(input.as_str(), config.text.as_str());
        
        if input.len() > 0 && start.is_none() {
            start = Some(Instant::now());
        }
        
        let input_parts: Vec<&str> = input.split(" ").collect();
        let input_word_count = input_parts.len();
        let input_last_word = input_parts.last().copied().unwrap_or("");
        
        #[cfg(debug_assertions)]
        {
            println!("\n\rinput_word_count: {}", input_word_count);
            println!("\rinput_last_word: {}", input_last_word);
            println!("\rinput_length: {}", input.chars().count());
            println!("\rtarget_word_count: {}", target_word_count);
            println!("\rtarget_last_word: {}", last_word);
            println!("\rtarget_text_length: {}", config.text.chars().count());
        }
        
        if input_word_count == target_word_count && last_word == input_last_word {
            print!("\n");
            break;
        }
        
        if input.len() > 0 {
            let _ = execute!(stdout, MoveTo(0, 1), MoveRight(input.chars().count().try_into().unwrap_or(0)));
        } else {
            let _ = execute!(stdout, MoveTo(0, 1));
        }
        
        if poll(Duration::from_millis(500)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match (event.code, event.modifiers) {
                    (KeyCode::Char('w'), m) | (KeyCode::Backspace, m) if m.contains(KeyModifiers::CONTROL) => {
                        input.pop();
                        while let Some(c) = input.chars().last() {
                            if c.is_whitespace() {
                                break;
                            }
                            
                            input.pop();
                        }                    
                    },
                    (KeyCode::Backspace, _) => {
                        input.pop();
                    },
                    (KeyCode::Esc, _) => {
                        break;
                    },
                    (KeyCode::Char(' '), _) => {
                        if input.chars().last().unwrap_or(' ') != ' ' {
                            input.push(' ');
                        }
                    },
                    (KeyCode::Char(c), _) => {
                        input.push(c);
                    },
                    _ => {}
                }
                _ => {}
            }
        }
        
    }
    
    if start.is_some() {
        let end = Instant::now() - start.unwrap();
        
        let wpm = calculate_wpm(input.as_str(), config.text.as_str(), end);
        
        println!("\rWPM: {}", wpm);
    }
    
    Ok(())
}

fn print_word_match(input: &str, target: &str) {
    for (i, char) in target.chars().enumerate() {
        let input_char = input.chars().nth(i).unwrap_or('\0');
        
        if input_char == '\0' {
            print!("{}", char);
            continue;
        }
        
        if input_char == char {
            print!("{}", yellow(input_char.to_string().as_str()));
        }
        
        if input_char != char {
            print!("{}", red(input_char.to_string().as_str()));
        }
    }
    
    if input.len() > target.len() {
        let input_rest = &input[target.len()..];
        print!("{}", red_hi(input_rest));
    }
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
    
    Ok(())
}
