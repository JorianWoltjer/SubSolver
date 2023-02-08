use std::collections::{HashMap, HashSet};

use crate::{Word, normalize};

/// Clean the input string into a consistent format
/// - Remove all non-alphabetic characters (only keep spaces)
/// - Convert all characters to lowercase
/// - Trim leading and trailing whitespace
/// - Remove duplicate spaces
pub fn clean_input(input: &str) -> String {
    input.to_string().chars()
        .filter(|c| c.is_ascii_alphabetic() || c == &' ')
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
        .split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Parse the input string into a vector of `Word`s
pub fn input_to_words(input: &str, dictionary: HashMap<String, HashSet<String>>) -> Vec<Word> {
    input.split_whitespace()
        .map(|s| Word::new(s, dictionary.get(&normalize(s)).unwrap())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_test() {
        let word = "aardvark";
        
        assert_eq!(normalize(word), "AABCDABE");
    }
}
