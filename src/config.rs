use std::{fs::File, io::{self, BufRead, IsTerminal, Read}};
use clap::Parser;
use rand::{self, Rng};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    text: Option<String>,
    
    #[arg(short, long)]
    file: Option<String>
}

pub struct Config {
    pub text: String
}

impl Config {
    pub fn new() -> Config {
        return Config {
            text: Config::get_default_text()
        };
    }
    
    fn get_default_text() -> String {
        let stdin_text = Config::get_stdin_text();
        println!("{}", stdin_text.is_some());
        if stdin_text.is_some() {
            return stdin_text.unwrap();
        }
        
        let file_text = Config::get_file_text();
        
        if file_text.is_some() {
            return file_text.unwrap();
        }
        
        let args = Args::parse();
        
        if args.text.is_some() {
            return args.text.unwrap();
        }
        
        return Config::generate_text();
    }
    
    fn get_stdin_text() -> Option<String> {
        let stdin_input: Option<String> = if !io::stdin().is_terminal() {
            let mut buffer = String::new();
            
            io::stdin().read_to_string(&mut buffer).ok();
            
            if buffer.is_empty() {
                None
            } else {
                Some(buffer)
            }
        } else {
            None
        };
        
        return stdin_input;
    }
    
    fn get_file_text() -> Option<String> {
        let args = Args::parse();
        
        if args.file == None {
            return None;
        }
        
        return std::fs::read_to_string(args.file.unwrap()).ok();
    }
    
    fn generate_text() -> String {
        let words_count = 15; // fixed for now
        
        let words = Config::get_random_lines(words_count);
        
        return words.join(" ");
    }
    
    fn get_random_lines(total: u32) -> Vec<String> {
        let file = include_str!("./static/english.txt");
        let words: Vec<&str> = file.split("\n").collect();
        
        let mut choosen_words: Vec<String> = Vec::new();
        
        let mut rng = rand::rng();
        
        while choosen_words.len() < total as usize {
            let random_index = rng.random_range(0..words.len());
            let random_word = words.get(random_index).copied().unwrap_or("");
            
            choosen_words.push(random_word.to_string());
        }
        
        return choosen_words;
    }
}