use std::{fs::File, io::{self, BufRead, IsTerminal, Read}};
use clap::{Parser, ValueEnum};
use rand::{self, Rng};

#[derive(Debug, Clone, ValueEnum)]
enum Language {
    Pt,
    Br,
    En
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    text: Option<String>,
    
    #[arg(short, long)]
    file: Option<String>,
    
    #[arg(short, long, visible_alias("c"))]
    word_count: Option<u32>,
    
    #[arg(short, long, visible_alias("t"))]
    time: Option<u32>,
    
    #[arg(short, long)]
    language: Option<Language>
}

pub enum Mode {
    Free,
    Timer(u32),
    Word(u32)
}

pub struct Config {
    pub text: String,
    pub mode: Mode
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        let mut mode = Mode::Free;
        
        if args.time.is_some() {
            mode = Mode::Timer(args.time.unwrap());
        } else if args.word_count.is_some() {
            mode = Mode::Word(args.word_count.unwrap());
        }
        
        return Config {
            text: Config::get_default_text().trim().to_string(),
            mode: mode
        };
    }
    
    fn get_default_text() -> String {
        let stdin_text = Config::get_stdin_text();

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
        let args = Args::parse();
        let words_count = args.word_count.unwrap_or(200);
        
        let words = Config::get_random_lines(words_count, args.language.unwrap_or(Language::En));
        
        return words.join(" ");
    }
    
    fn get_random_lines(total: u32, language: Language) -> Vec<String> {
        let file = match language {
            Language::Br | Language::Pt => include_str!("./static/portuguese.txt"),
            Language::En => include_str!("./static/english.txt")
        };
        
        let words: Vec<&str> = file.split("\n").collect();
        
        let mut choosen_words: Vec<String> = Vec::new();
        
        let mut rng = rand::rng();
        
        while choosen_words.len() < total as usize {
            let random_index = rng.random_range(0..words.len());
            let random_word = words.get(random_index).copied().unwrap_or("");
            
            let last_word = choosen_words.last();
            
            if random_word.len() <= 3 && last_word.is_some() && last_word.unwrap().len() <= 3 {
                continue;
            }
            
            choosen_words.push(random_word.to_string());
        }
        
        return choosen_words;
    }
}
