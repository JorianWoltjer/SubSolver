use std::{collections::{HashMap, HashSet}};

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

pub fn prune(cipher_words: &mut Vec<Word>) {
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

    pub fn solve(&mut self) {
        let mut map: HashMap<char, char> = HashMap::new();
        self.solve_recursive(0, &mut map);
    }

    fn solve_recursive(&mut self, depth: usize, map: &mut HashMap<char, char>) {
        if is_consistent(map) {
            if depth >= self.cipher_words.len() {  // FOUND SOLUTION
                for word in &self.cipher_words {
                    for j in word.word.chars() {
                        print!("{}", map.get(&j).unwrap());
                    }
    
                    print!(" ");
                }
    
                println!("");
            }
            else {
                for i in self.cipher_words[depth].candidates.to_owned().iter() {
                    // println!("{}{} -> {}", "  ".repeat(depth), self.cipher_words[depth].word, i);
                    if &apply_map(&self.cipher_words[depth].word, &i, map) == i {
                        self.solve_recursive(depth + 1, 
                            &mut update_map(&self.cipher_words[depth].word, &i, map));
                    }
                }
            }
        }
    }
}
