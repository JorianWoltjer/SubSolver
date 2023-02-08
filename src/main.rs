#[macro_use]
extern crate lazy_static;

use std::{io::{Write, stdout}, error::Error, collections::{HashMap, HashSet}};
use std::{fs::File, io::{BufReader, BufRead}};

use sub_solver::{solve::{prune, Solver}, input::{input_to_words, clean_input}, normalize};

lazy_static! {
    // TODO: Make this dynamic later
    static ref DICTIONARY: HashMap<String, HashSet<String>> = {
        let mut map = HashMap::new();
        let file = File::open("wordlist/dutch-clean.txt").unwrap();

        for word in BufReader::new(file).lines() {
            let word = word.unwrap();
            map.entry(normalize(&word))
                .or_insert(HashSet::new()).insert(word);
        }

        map
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("[?] Enter ciphertext:");
    print!(" > "); stdout().flush()?;

    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;
    println!("[~] Parsing and mapping words...");

    let mut cipher_words = input_to_words(&clean_input(&input), DICTIONARY.clone());
    
    println!("[~] Pruning...");

    prune(&mut cipher_words);

    println!("[*] Breaking...");
    println!("");

    let mut solver = Solver::new(cipher_words);
    solver.solve();

    Ok(())
}
