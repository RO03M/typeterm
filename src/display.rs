pub fn pager(input: String, max_chars: u16, page: u16) {
    let offset = (page * max_chars) as usize;
    let max_chars = max_chars as usize;
    println!("{}", &input[offset..(offset + max_chars)]);
}

pub fn page_by_words(input: String, words_per_page: usize, page: usize) -> String {
    let words: Vec<&str> = input.split(" ").collect();
    
    if words.len() <= words_per_page {
        return words.join(" ");
    }
    
    let start = page * words_per_page;
    let end = start + words_per_page;
    
    if start >= words.len() {
        return String::new();
    }
    
    if end >= words.len() {
        return words[start..].join(" ");
    }
    
    return words[start..end].join(" ");
}

pub fn get_current_index() {
    
}
