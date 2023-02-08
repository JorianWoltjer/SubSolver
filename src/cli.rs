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
