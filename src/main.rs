use std::{io::{Error, stdout}, time::{Duration, Instant}};

use crossterm::{cursor::{MoveRight, MoveTo, MoveToRow}, event::{Event, KeyCode, read}, execute, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}};

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
    
    let start = Instant::now();
    
    loop  {
        let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        
        print_parts(input.as_str(), config.text.as_str());
        
        if input.len() > 0 {
            let _ = execute!(stdout, MoveToRow(0), MoveRight(input.len().try_into().unwrap_or(0)));
        } else {
            let _ = execute!(stdout, MoveTo(0, 0));
        }
        
        let input_parts: Vec<&str> = input.split(" ").collect();
        let input_word_count = input_parts.len();
        let input_last_word = input_parts.last().copied().unwrap_or("");
        
        // println!("\n\rinput_word_count: {}\n\rlast_word: {}\n\rfull_text_word_count: {}\n\rlast_word: {}\n\r", input_word_count, input_last_word, target_word_count, last_word);
        
        if input_word_count == target_word_count && last_word == input_last_word {
            print!("\n");
            break;
        }
        
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Char(char) => {
                    input.push(char);
                },
                KeyCode::Backspace => {
                    input.pop();
                },
                KeyCode::Esc => break,
                _ => {}
            },
            _ => {}
        }
    }
    
    let end = Instant::now() - start;
    
    let wpm = calculate_wpm(input.as_str(), config.text.as_str(), end);
    
    println!("WPM: {}", wpm);
    
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

fn show_results(start: Instant) {
    let end = Instant::now() - start;
    
    println!("\n\rTook {} seconds", end.as_secs());
}

fn main() -> Result<(), ()> {
    let _ = enable_raw_mode();
    
    let start = Instant::now();
    
    if let Err(e) = render() {
        eprintln!("Error: {e}");
    }
    
    show_results(start);
    
    let _ = disable_raw_mode();
 
    println!();

    
    Ok(())
}
