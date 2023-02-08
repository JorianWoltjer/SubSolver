use std::collections::{HashMap, HashSet};

use unidecode::unidecode;

use crate::{Word, normalize};

/// Clean the input string into a consistent format
/// - Remove all non-alphabetic characters (only keep spaces)
/// - Convert all characters to lowercase
/// - Trim leading and trailing whitespace
/// - Remove duplicate spaces
/// - Normalize unicode characters
pub fn clean_input(input: &str) -> String {
    unidecode(input).chars()
        .filter(|c| c.is_ascii_alphabetic() || c == &' ')
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
        .split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Parse the input string into a vector of `Word`s. 
/// Returns `None` if the input string contains a word that is not possible in the dictionary
pub fn input_to_words(input: &str, dictionary: HashMap<String, HashSet<String>>) -> Result<Vec<Word>, String> {
    let mut result = Vec::new();

    for word in input.split_whitespace() {
        if let Some(candidates) = dictionary.get(&normalize(word)) {
            result.push(Word::new(word, candidates));
        } else {
            return Err(format!("Word {:?} is not possible in the dictionary", word));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_input_tests() {
        assert_eq!(clean_input("Hello, world!"), "hello world");
        assert_eq!(clean_input("Hello, world! 123"), "hello world");
        assert_eq!(clean_input("  some   spaces   "), "some spaces");
        assert_eq!(clean_input("Oké Måns"), "oke mans");
        assert_eq!(clean_input("Æneid"), "aeneid");
    }
}

