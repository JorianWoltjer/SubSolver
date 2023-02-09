use std::path::PathBuf;

use clap::{Parser, ArgGroup};

/// Substitution Cipher Solver
#[derive(Parser, Debug)]
#[command(name = "sub-solver")]
pub struct Args {
    #[clap(flatten)]
    pub ciphertext: Ciphertext,

    /// Path to the wordlist file (default: built-in english.txt)
    #[arg(short, long)]
    pub wordlist: Option<String>,

    /// Starting key, letter mapping (default: empty, example: "a:b,c:d,e:f", "ab,cd,ef", "b?d?f?????????????????????")
    #[arg(short, long)]
    pub key: Option<String>,

    /// Fill in unknowns in solution with random unused letters (default: false)
    #[arg(short='F', long)]
    pub fill_key: bool,

    /// Disable dictionary cache (default: false)
    #[arg(short, long)]
    pub no_cache: bool,
}

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("ciphertext").required(true).multiple(false))]
pub struct Ciphertext {
    /// Ciphertext string to solve
    #[clap(group = "ciphertext", short, long)]
    pub string: Option<String>,

    /// Path to the ciphertext file
    #[clap(group = "ciphertext", short, long)]
    pub file: Option<PathBuf>,
}
