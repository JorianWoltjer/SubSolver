#[macro_use]
extern crate lazy_static;

use std::{fs::File, io::{BufReader, BufRead, Write, stdout}, error::Error, collections::{HashMap, HashSet}, str::FromStr};

lazy_static! {
    // TODO: Make this dynamic later
    static ref DICTIONARY: HashMap<String, Vec<String>> = {
        let mut map = HashMap::new();
        let file = File::open("wordlist/dutch-clean.txt").unwrap();

        for word in BufReader::new(file).lines() {
            let word = word.unwrap();
            map.entry(normalize(&word))
                .or_insert(Vec::new()).push(word);
        }

        map
    };
}

fn normalize(s: &str) -> String {
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

fn intersect(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    let mut result = HashSet::new();

    for i in a {
        if b.contains(&i) {
            result.insert(i);
        }
    }

    result
}

fn is_consistent(map: &HashMap<char, char>) -> bool {
    let mut counter: HashMap<char, char> = HashMap::new();
    
    for (&first, &second) in map.iter() {
        counter.insert(second, first);
    }
    
    map.len() == counter.len()
}

fn apply_map(cipher: &str, plain: &str, map: &HashMap<char, char>) -> String {
    let mut result = String::new();

    for (i, c) in cipher.chars().enumerate() {
        let c = *map.get(&c).unwrap_or(&plain.chars().nth(i).unwrap());
        result.push(c);
    }

    result
}
fn update_map<'a>(cipher: &str, plain: &str, map: &HashMap<char, char>) -> HashMap<char, char> {
    let mut map = map.to_owned();
    
    for (i, c) in cipher.chars().enumerate() {
        if !map.contains_key(&c) {
            map.insert(c, plain.chars().nth(i).unwrap());
        }
    }

    map
}

fn unscramble(cipher_words: &mut Vec<Word>, depth: usize, map: &mut HashMap<char, char>) {
    // println!("{}{:?}", "  ".repeat(depth), map);
    
    if is_consistent(map) {
        if depth >= cipher_words.len() {  // FOUND SOLUTION
            for word in cipher_words {
                for j in word.word.chars() {
                    print!("{}", map.get(&j).unwrap());
                }

                print!(" ");
            }

            println!("");
        }
        else {
            for i in cipher_words[depth].candidates.to_owned().iter() {
                // println!("{}{} -> {}", "  ".repeat(depth), cipher_words[depth].word, i);
                if &apply_map(&cipher_words[depth].word, &i, map) == i {
                    unscramble(cipher_words, depth + 1, 
                        &mut update_map(&cipher_words[depth].word, &i, map));
                }
            }
        }
    }
}

struct Word {
    word: String,
    candidates: HashSet<String>,
    letter_map: HashMap<char, HashSet<char>>,
}
impl FromStr for Word {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut candidates = HashSet::new();
        let mut letter_map = HashMap::new();

        for word in DICTIONARY.get(&normalize(s)).unwrap() {
            candidates.insert(word.to_string());
            for (i, j) in s.chars().zip(word.chars()) {
                letter_map.entry(i)
                    .or_insert(HashSet::new()).insert(j);
            }
        }

        Ok(Word {
            word: s.to_string(),
            candidates,
            letter_map,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter ciphertext:");
    print!("> "); stdout().flush()?;

    let mut sentence = String::new();

    std::io::stdin().read_line(&mut sentence)?;
    println!("Preparing...");

    let mut cipher_words: Vec<Word> = sentence.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    
    println!("Pruning...");

    let mut pruner: HashMap<char, HashSet<char>> = HashMap::new();
    for i in 'a'..='z' {
        for j in 'a'..='z' {
            pruner.entry(i)
                .or_insert(HashSet::new()).insert(j);
        }
    }

    for word in cipher_words.iter() {
        for j in word.word.chars() {
            pruner.entry(j)
                .and_modify(|e| *e = intersect(e.clone(), word.letter_map.get(&j).unwrap().clone()));
        }
    }
    
    for word in cipher_words.iter_mut() {
        for j in 0..word.word.len() {
            word.candidates.retain(|k| pruner.get(&word.word.chars().nth(j).unwrap()).unwrap().contains(&k.chars().nth(j).unwrap()));
        }
    }

    println!("Breaking...");
    println!("");

    unscramble(&mut cipher_words, 0, &mut HashMap::new());

    println!("");
    println!("The program has finished.");

    Ok(())
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

