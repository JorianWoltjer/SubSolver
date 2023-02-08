use std::collections::{HashMap, HashSet};

use unidecode::unidecode;

use crate::{Word, normalize};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

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

pub fn parse_key(key: &str) -> Result<HashMap<char, char>, String> {
    if key.contains('?') {
        // Key is in wildcard format (example: "b?d?f?????????????????????")
        let mut result = HashMap::new();
        for (c, k) in ALPHABET.chars().zip(key.chars()) {
            if k != '?' {
                if !ALPHABET.contains(k) {
                    return Err(format!("Invalid key character: {:?} (should be in lowercase alphabet)", k));
                }
                // Error if duplicate key
                if let Some(_) = result.insert(c, k) {
                    return Err(format!("Duplicate key character: {:?}", key));
                }
            }
        }
        return Ok(result);
    } else {
        // Key is in delimiter format (example: "a:b,c:d,e:f" or "ab,cd,ef")
        let mut result = HashMap::new();
        for pair in key.split(',') {
            let pair = pair.chars().collect::<Vec<char>>();
            let (a, b) = (pair.first().ok_or(format!("No first character in key: {:?}", key))?, 
                                        pair.last().ok_or(format!("No last character in key: {:?}", key))?);

            if !ALPHABET.contains(*a) || !ALPHABET.contains(*b) {
                return Err(format!("Invalid key character: {:?} (should be in lowercase alphabet)", pair));
            }
            // Error if duplicate key
            if let Some(_) = result.insert(*a, *b) {
                return Err(format!("Duplicate key character: {:?}", key));
            }
        }
        return Ok(result);
    }
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

    #[test]
    fn parse_key_tests() {
        assert_eq!(parse_key("a:b,c:d,e:f").unwrap(), [('a', 'b'), ('c', 'd'), ('e', 'f')].iter().cloned().collect());
        assert_eq!(parse_key("ab,cd,ef").unwrap(), [('a', 'b'), ('c', 'd'), ('e', 'f')].iter().cloned().collect());
        assert_eq!(parse_key("b?d?f?????????????????????????").unwrap(), [('a', 'b'), ('c', 'd'), ('e', 'f')].iter().cloned().collect());
    }
}
