// Mode handles the logic to display the timer if the mode is "time", the word count if the mode is "word", etc.
// Besides that, it should care about if the program may end or not.
// 

use std::time::Instant;

use crate::{calculate_wpm, config::Mode, text::wc};

pub struct Session {
    pub mode: Mode,
    pub input: String,
    phrase_word_count: usize,
    phrase_last_word: String,
    phrase: String,
    started_at: Option<Instant>,
    finished_at: Option<Instant>,
}

impl Session {
    pub fn new(mode: Mode, phrase: String) -> Session {
        let words = phrase.split(" ");
        // let phrase_words: Vec<&str> = words.clone().collect();
        
        let phrase_word_count = words.clone().count();
        let phrase_last_word = words.last().unwrap_or("").to_string();
        
        return Session {
            input: String::new(),
            phrase: phrase,
            phrase_word_count: phrase_word_count,
            phrase_last_word: phrase_last_word,
            mode: mode,
            started_at: None,
            finished_at: None
        };
    }
    
    pub fn header(&self) -> String {
        match self.mode {
            Mode::Timer(t) => {
                if self.started_at.is_none() {
                    return "-".to_string();
                }
                
                let delta = (Instant::now() - self.started_at.unwrap_or(Instant::now())).as_secs();
                if delta as u32 > t {
                    return "\r0".to_string();
                }
                
                let remaining_time = t - delta as u32;
                
                return format!("\r{}", remaining_time);  
            },
            Mode::Word(_) => {
                return format!("\r{}/{}", wc(self.input.as_str()), self.phrase_word_count);
            },
            _ => {
                return "".to_string();
            }
        }
    }
    
    pub fn phrase(&self) -> String {
        return self.phrase.clone();
    }
    
    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
    }
    
    pub fn is_started(&self) -> bool {
        return self.started_at.is_some();
    }
    
    pub fn end(&mut self) {
        self.finished_at = Some(Instant::now());
    }
    
    pub fn should_end(&self) -> bool {
        match self.mode {
            Mode::Timer(t) => {
                let delta = (Instant::now() - self.started_at.unwrap_or(Instant::now())).as_secs();
                
                if delta as u32 > t {
                    return true;
                }
            },
            _ => {}
        }
        
        let input_parts: Vec<&str> = self.input.split(" ").collect();
        let input_word_count = input_parts.len();
        let input_last_word = input_parts.last().copied().unwrap_or("");
        
        return input_word_count == self.phrase_word_count && self.phrase_last_word == input_last_word;
    }
    
    pub fn wpm(&self) -> f32 {    
        let duration = self.finished_at.unwrap_or(Instant::now()) - self.started_at.unwrap_or(Instant::now());
        
        return calculate_wpm(self.input.as_str(), &self.phrase, duration);
    }
}
