use std::{collections::{HashMap, HashSet}, sync::mpsc, fmt::Display};

use crate::Word;

fn intersect(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    let mut result = HashSet::new();

    for i in a {
        if b.contains(&i) {
            result.insert(i);
        }
    }

    result
}

/// Remove certain words from the candidates that are not possible
pub fn prune(cipher_words: &mut Vec<Word>) {
    // Initialize with all possible letters
    let mut pruner: HashMap<char, HashSet<char>> = HashMap::new();
    for i in 'a'..='z' {
        for j in 'a'..='z' {
            pruner.entry(i)
                .or_insert(HashSet::new()).insert(j);
        }
    }

    // Remove letters that are not possible
    for word in cipher_words.iter() {
        for j in word.word.chars() {
            pruner.entry(j)
                .and_modify(|e| *e = intersect(e.clone(), word.letter_map.get(&j).unwrap().clone()));
        }
    }
    
    // Remove candidates that are not possible
    for word in cipher_words.iter_mut() {
        for j in 0..word.word.len() {
            word.candidates.retain(|k| pruner.get(&word.word.chars().nth(j).unwrap()).unwrap().contains(&k.chars().nth(j).unwrap()));
        }
    }
}

pub fn order_by_possible_words(cipher_words: &mut Vec<Word>) {
    cipher_words.sort_by(|a, b| {
        let a = a.word.len();
        let b = b.word.len();

        b.cmp(&a)
    });
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

pub struct Solver {
    pub cipher_words: Vec<Word>,
}
impl Solver {
    pub fn new(cipher_words: Vec<Word>) -> Self {
        Solver {
            cipher_words,
        }
    }

    pub fn solve(&mut self, tx: &mpsc::Sender<Solution>, mut starting_key: HashMap<char, char>) {
        self.solve_recursive(0, &mut starting_key, &tx);
    }

    fn solve_recursive(&mut self, depth: usize, map: &mut HashMap<char, char>, tx: &mpsc::Sender<Solution>) {
        if is_consistent(map) {
            if depth >= self.cipher_words.len() {  // Solution found
                let solution = Solution::new(map.to_owned());
                tx.send(solution).unwrap();
            }
            else {
                // Explore all candidates
                for i in self.cipher_words[depth].candidates.to_owned().iter() {
                    if &apply_map(&self.cipher_words[depth].word, &i, map) == i {
                        self.solve_recursive(depth + 1, 
                            &mut update_map(&self.cipher_words[depth].word, &i, map), 
                            tx);
                    }
                }
            }
        }
    }
}

pub struct Solution {
    pub key: HashMap<char, char>,
}
impl Solution {
    pub fn new(key: HashMap<char, char>) -> Self {
        Solution {
            key,
        }
    }
    
    /// Fill unknowns in the key with unused letters
    pub fn fill_key(&mut self) {
        let mut unused: Vec<char> = ('a'..='z').rev().collect();
        // Filter used letters
        for value in self.key.values() {
            unused.retain(|x| x != value);
        }

        // Fill unknown letters with unused letters
        for c in 'a'..='z' {
            if !self.key.contains_key(&c) {
                self.key.insert(c, unused.pop().unwrap());
            }
        }
    }

    pub fn apply(&self, ciphertext: &str) -> String {
        let mut result = String::new();
        
        for c in ciphertext.chars() {
            // Show unknown letters as '?', but keep punctuation
            result.push(*self.key.get(&c).unwrap_or_else(|| {
                if c.is_alphabetic() { &'?' } else { &c }
            }));
        }

        result
    }
}
impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ('a'..='z')
            .map(|c| *self.key.get(&c).unwrap_or(&'?'))
            .collect::<String>().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{input::input_to_words, load_wordlist};

    use super::*;

    #[test]
    fn can_solve() {
        let ciphertext = "x cbt tloap";
        let wordlist = ["many", "words", "here", "to", "test", "the", "solver", "also", "few", "a", "ok", "now", "all", "words", "should", "be", "good"]
                                    .join("\n");
        let dictionary = load_wordlist(wordlist);

        let cipher_words = input_to_words(ciphertext, dictionary).unwrap();
        let mut solver = Solver::new(cipher_words);
        
        let (tx, rx) = mpsc::channel();
        solver.solve(&tx, HashMap::new());
        
        let solution = rx.recv().unwrap();
        let plaintext = solution.apply(ciphertext);
        assert_eq!(plaintext, "a few words");
    }
}
