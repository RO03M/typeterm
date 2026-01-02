use rand::{self, Rng};

pub fn wc(text: &str) -> usize {
    return text.split_whitespace().count();
}

pub fn generate_text(words: Vec<&str>, word_count: usize) -> String {
    let mut choosen_words: Vec<String> = Vec::new();
    
    let mut rng = rand::rng();
    
    let mut try_counter: u8 = 0;
    let max_tries: u8 = 3;
    
    while choosen_words.len() < word_count {
        let random_index = rng.random_range(0..words.len());
        let random_word = words.get(random_index).copied().unwrap_or("");
        
        let last_word = choosen_words.last();
        
        let double_short_word = random_word.len() <= 3 && last_word.is_some() && last_word.unwrap().len() <= 3;
        let duplicate_word = random_word == last_word.unwrap_or(&"".to_string());
        
        if (double_short_word || duplicate_word) && try_counter < max_tries {
            try_counter += 1;
            continue;
        }
        
        choosen_words.push(random_word.to_string());
    }
    
    return choosen_words.join(" ");
}

#[cfg(test)]
mod tests {
    use crate::text::{generate_text, wc};

    #[test]
    fn test_generate_text_has_correct_count() {
        let word_list = vec!["first", "second", "third", "anata"];
        
        assert_eq!(wc(generate_text(word_list, 10).as_str()), 10);
    }
    
    #[test]
    fn test_generate_text_with_just_one_word_on_list() {
        let word_list = vec!["one"];
        
        assert_eq!(generate_text(word_list, 5), "one one one one one");
    }
}
