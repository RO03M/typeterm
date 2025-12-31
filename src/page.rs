pub fn get_page_from_input(input: &str, words_per_page: usize) -> usize {
    if words_per_page == 0 {
        return 0;
    }
    
    // na última palavra tenho que verificar se o cara já deu o espaço
    let mut word_count = input.split_whitespace().count();
    
    if input.chars().last() == Some(' ') {
        word_count += 1;
    }
    
    if word_count < words_per_page {
        return 0;
    }
    
    return (word_count - 1) / words_per_page;
}

#[cfg(test)]
mod tests {
    use crate::page::get_page_from_input;

    #[test]
    fn test_get_page() {
        assert_eq!(get_page_from_input("small school run could becom enation however develop from people", 10), 0);
        assert_eq!(get_page_from_input("small school run could becom enation however develop from people ", 10), 1);
        assert_eq!(get_page_from_input("with still can word may call up mean say keep year one however the both do small long stand if", 10), 1);
        assert_eq!(get_page_from_input("with still can word may call up mean say keep year one however the both do small long stand if ", 10), 2);
    }
}
