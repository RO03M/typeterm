use crate::colors::{background_red, red_bold, yellow};
use rand::{self, Rng};

// don't like the name of both these functions, and I think they shouldn't be in text... but I am not sure yet where to place those fuckers

pub fn compare_strings(input: &str, target: &str) -> String {
    let input_parts: Vec<&str> = input.split(" ").collect();
    let target_parts: Vec<&str> = target.split(" ").collect();

    let mut final_string = String::new();
    
    for (index, target_part) in target_parts.iter().enumerate() {
        let input_part = input_parts.get(index).copied().unwrap_or("");
        
        final_string.push_str(compare_words(input_part, target_part).as_str());
        final_string.push(' ');
    }
    
    return final_string;
}

pub fn compare_words(input: &str, target: &str) -> String {
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

pub fn wc(text: &str) -> usize {
    return text.split_whitespace().count();
}

// future new function
pub fn generate_text(words: Vec<&str>, word_count: usize) -> String {
    let mut choosen_words: Vec<String> = Vec::new();
    
    let mut rng = rand::rng();
    
    while choosen_words.len() < word_count {
        let random_index = rng.random_range(0..words.len());
        let random_word = words.get(random_index).copied().unwrap_or("");
        
        let last_word = choosen_words.last();
        
        // have to check if the last string is equal to the current,
        // but at the same time this might cause a loop,
        // so maybe a try no repeat counter to avoid loops
        
        if random_word.len() <= 3 && last_word.is_some() && last_word.unwrap().len() <= 3 {
            continue;
        }
        
        choosen_words.push(random_word.to_string());
    }
    
    return choosen_words.join(" ");
}

#[cfg(test)]
mod tests {

}
