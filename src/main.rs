use std::{error::Error, fs::read_to_string};

use clap::Parser;

use sub_solver::{solve::{prune, Solver}, input::{input_to_words, clean_input}, cli::Args, load_wordlist};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // TODO: colored output with functions
    let wordlist_content = match args.wordlist {
        Some(path) => {
            println!("[~] Loading wordlist from {}...", path);
            read_to_string(path)?
        },
        None => {
            println!("[~] Loading built-in english wordlist...");
            include_str!("../wordlist/english.txt").to_string()
        }
    };
    // TODO: cache this to a file and load with md5 hash + add --no-cache flag
    let dictionary = load_wordlist(wordlist_content);

    println!("[~] Parsing and mapping input words...");

    let ciphertext = match args.ciphertext.string {
        Some(ciphertext) => {
            println!("[*] Input: {:?}", ciphertext);
            ciphertext
        },
        None => {
            let path = args.ciphertext.file.unwrap();
            println!("[*] Input file: {:?}", path);
            read_to_string(path)?
        }
    };

    let clean = clean_input(&ciphertext);
    let mut cipher_words = input_to_words(&clean, dictionary.clone())
        .expect("Input contained a word that was not possible in the dictionary");
    
    println!("[~] Pruning...");

    // Order by length, longest first. Also remember original order
    let original_cipher_words = cipher_words.clone();
    cipher_words.sort_by(|a, b| b.word.len().cmp(&a.word.len()));
    
    prune(&mut cipher_words);

    println!("[*] Cracking...");
    println!("");

    let mut solver = Solver::new(cipher_words);
    solver.solve(&original_cipher_words);

    println!("[+] Finished!");

    Ok(())
}
