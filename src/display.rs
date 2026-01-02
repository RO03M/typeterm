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
