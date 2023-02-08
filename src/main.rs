use std::{error::Error, fs::read_to_string, thread, sync::mpsc};

use clap::Parser;
use anyhow::Result;

use sub_solver::{solve::{prune, Solver}, input::{input_to_words, clean_input}, cli::Args, load_wordlist, apply_solution, loading::Loading};

fn main() {
    let loading = Loading::default();
    
    if let Err(e) = do_main(&loading) {
        loading.fail(e.to_string());
        loading.end();
        std::process::exit(1);
    }
}

fn do_main(loading: &Loading) -> Result<(), Box<dyn Error>> {
    // Parse args
    let args = Args::parse();

    loading.text("Loading wordlist...".to_string());

    let wordlist_content = match args.wordlist {
        Some(path) => {
            loading.info(format!("Loaded wordlist from {:?}", path));
            read_to_string(path)?
        },
        None => {
            loading.info(format!("Loaded built-in english wordlist"));
            include_str!("../wordlist/english.txt").to_string()
        }
    };
    // TODO: cache this to a file and load with md5 hash + add --no-cache flag
    loading.text("Finding patterns in wordlist...".to_string());
    let dictionary = load_wordlist(wordlist_content);
    loading.success(format!("Loaded {} unique patterns", dictionary.len()));

    loading.text(format!("Parsing and mapping input words..."));

    let ciphertext = match args.ciphertext.string {
        Some(ciphertext) => {
            loading.info(format!("Input string: {:?}", ciphertext));
            ciphertext
        },
        None => {
            let path = args.ciphertext.file.unwrap();
            loading.info(format!("Input file: {:?}", path));
            read_to_string(path)?
        }
    };

    // Parse input
    let clean = clean_input(&ciphertext);
    
    let mut cipher_words = input_to_words(&clean, dictionary.clone())?;

    loading.success(format!("Parsed {} input words", cipher_words.len()));
    
    loading.text(format!("Pruning..."));
    // Order by length, longest first
    cipher_words.sort_by(|a, b| b.word.len().cmp(&a.word.len()));
    // Remove impossible words
    prune(&mut cipher_words);
    loading.success(format!("Pruned impossible words"));
    
    loading.end();
    loading.info(format!("Starting to find solutions..."));

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // TODO: allow partial key as argument in here
        let mut solver = Solver::new(cipher_words);
        solver.solve(&tx);
    });
    
    let mut solutions = 0;
    for solution in rx {  // Print solutions as they are found
        let plaintext = apply_solution(&clean, &solution);
        let key = "abcdefghijklmnopqrstuvwxyz".chars()
            .map(|c| *solution.get(&c).unwrap_or(&c))
            .collect::<String>();
        println!("{} -> {}", key, plaintext);
        solutions += 1;
    }

    if solutions == 0 {
        loading.fail("No solutions found.".to_string());
    } else {
        loading.success(format!("Finished! ({} solutions)", solutions));
    }

    Ok(())
}
