use crate::colors::{background_red, red_bold, yellow};

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

#[cfg(test)]
mod tests {

}
