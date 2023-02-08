use std::{collections::{HashMap, HashSet}};

pub mod input;
pub mod solve;

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

pub fn normalize(s: &str) -> String {
    let mut result = s.to_string().as_bytes().to_vec();
    let mut replacement = b'A';

    for i in 0..result.len() {
        if result[i].is_ascii_uppercase() {
            continue;
        }

        // Replace all instances of the character with the replacement
        result = result.iter().map(|&c| if c == result[i] { replacement } else { c }).collect();
        replacement += 1;
    }
    
    String::from_utf8(result.to_vec()).unwrap()
}
