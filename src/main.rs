use std::{collections::HashMap, error::Error, fs::read_to_string, sync::mpsc, thread};

use clap::Parser;

use sub_solver::{
    cache::{load_cached_dictionary, save_cached_dictionary},
    cli::Args,
    input::{clean_input, input_to_words, parse_key},
    load_wordlist,
    loading::Loading,
    solve::{prune, Solver},
};

fn main() {
    let args = Args::parse();

    let loading = Loading::default();

    if let Err(e) = do_main(&loading, args) {
        loading.fail(e.to_string());
        loading.end();
        std::process::exit(1);
    }
}

fn do_main(loading: &Loading, args: Args) -> Result<(), Box<dyn Error>> {
    // Parse args
    let starting_key = match args.key {
        Some(key) => {
            loading.info(format!("Using starting key: {key:?}"));
            parse_key(&key)?
        }
        None => {
            loading.info("Using empty starting key".to_string());
            HashMap::new()
        }
    };

    loading.text("Loading wordlist...".to_string());

    let wordlist_content = match args.wordlist {
        Some(path) => {
            loading.info(format!("Using wordlist from {path:?}"));
            read_to_string(path)?
        }
        None => {
            loading.info("Using built-in english wordlist".to_string());
            include_str!("../wordlist/english.txt").to_string()
        }
    };

    // Try loading from cache
    let dictionary = if args.no_cache {
        loading.warn("Dictionary cache disabled".to_string());
        None
    } else {
        loading.text("Loading dictionary cache...".to_string());
        load_cached_dictionary(&wordlist_content)
    };

    let dictionary = if let Some(dictionary) = dictionary {
        // Cache loaded
        loading.success(format!(
            "Loaded {} unique patterns (from cache)",
            dictionary.len()
        ));
        dictionary
    } else {
        // Cache not loaded
        loading.text("Finding patterns in wordlist...".to_string());
        let dictionary = load_wordlist(&wordlist_content);
        loading.success(format!("Loaded {} unique patterns", dictionary.len()));

        if !args.no_cache {
            // Save cache
            save_cached_dictionary(&wordlist_content, &dictionary)?;
            loading.success("Saved dictionary cache".to_string());
        }
        dictionary
    };

    loading.text("Parsing and mapping input words...".to_string());

    let ciphertext = match args.ciphertext.string {
        Some(ciphertext) => {
            loading.info(format!("Input string: {ciphertext:?}"));
            ciphertext
        }
        None => {
            let path = args.ciphertext.file.unwrap();
            loading.info(format!("Input file: {path:?}"));
            read_to_string(path)?
        }
    };

    // Parse input
    let ciphertext_clean = clean_input(&ciphertext);

    let mut cipher_words = input_to_words(&ciphertext_clean, &dictionary)?;

    loading.success(format!("Parsed {} input words", cipher_words.len()));

    loading.text("Pruning...".to_string());
    // Order by length, longest first
    cipher_words.sort_by(|a, b| b.word.len().cmp(&a.word.len()));
    // Remove impossible words
    prune(&mut cipher_words);
    loading.success("Pruned impossible words".to_string());

    loading.end();
    loading.info("Starting to find solutions...".to_string());

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut solver = Solver::new(&cipher_words);
        solver.solve(starting_key, Some(&tx));
    });

    let mut solutions = 0;
    for mut solution in rx {
        // Print solutions as they are found
        let plaintext = solution.apply(&ciphertext_clean);

        if args.fill_key {
            solution.fill_key();
        }

        println!("{solution} -> {plaintext}");
        solutions += 1;
    }

    if solutions == 0 {
        loading.fail("No solutions found.".to_string());
    } else {
        loading.success(format!("Finished! ({solutions} solutions)"));
    }

    Ok(())
}
