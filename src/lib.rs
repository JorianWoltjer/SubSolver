use std::{collections::{HashMap, HashSet}};

use input::clean_input;

pub mod input;
pub mod solve;
pub mod cli;
pub mod loading;

#[derive(Debug, Clone)]
pub struct Word {
    pub word: String,
    pub candidates: HashSet<String>,
    pub letter_map: HashMap<char, HashSet<char>>,
}
impl Word {
    pub fn new(s: &str, candidates: &HashSet<String>) -> Self {
        let mut letter_map = HashMap::new();

        for word in candidates {
            for (i, j) in s.chars().zip(word.chars()) {
                letter_map.entry(i)
                    .or_insert(HashSet::new()).insert(j);
            }
        }

        Word {
            word: s.to_string(),
            candidates: candidates.clone(),
            letter_map,
        }
    }
}

/// Convert word to uppercase, and substitute all characters to be in alphabetical order.
/// This makes words equivalent if they have the same charactaristics
/// 
/// ```rust
/// use sub_solver::normalize;
/// 
/// assert_eq!(normalize("example"), "ABCDEFA");  // "example" has 2 'a's at the start and end
/// assert_eq!(normalize("example"), normalize("squares"));  // "example" and "squares" have the same repeated character positions
/// assert_eq!(normalize("testing"), "ABCADEF");  // "testing" does not have repeated characters at the start and end
/// ```
pub fn normalize(s: &str) -> String {
    let mut result = s.chars().collect::<Vec<char>>();
    let mut replacement = b'A';

    for i in 0..result.len() {
        if result[i].is_ascii_uppercase() {
            continue;
        }

        // Replace all instances of the character with the replacement
        result = result.iter().map(|&c| if c == result[i] { replacement as char } else { c }).collect();
        replacement += 1;
    }
    
    result.into_iter().collect()
}

/// Load a wordlist from a file into a dictionary with normalized words
pub fn load_wordlist(contents: String) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();

    for word in contents.lines() {
        let word = clean_input(word);
        map.entry(normalize(&word))
                .or_insert(HashSet::new()).insert(word.to_string());
    }

    map
}

pub fn apply_solution(ciphertext: &str, solution: &HashMap<char, char>) -> String {
    let mut result = String::new();

    for c in ciphertext.chars() {
        result.push(*solution.get(&c).unwrap_or(&c));
    }

    result
}
