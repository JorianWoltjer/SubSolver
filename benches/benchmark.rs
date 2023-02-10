use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sub_solver::{
    input::{clean_input, input_to_words},
    load_wordlist, normalize,
    solve::{Solution, Solver},
};

fn load_wordlist_bench(c: &mut Criterion) {
    let wordlist = [
        "many", "words", "here", "to", "test", "the", "solver", "also", "few", "a", "ok", "now",
        "all", "words", "should", "be", "good",
    ]
    .join("\n");
    c.bench_function("load_wordlist", |b| {
        b.iter(|| load_wordlist(black_box(&wordlist)))
    });
}

fn normalize_bench(c: &mut Criterion) {
    let word = "correctly";
    c.bench_function("normalize", |b| b.iter(|| normalize(black_box(word))));
}

fn clean_input_bench(c: &mut Criterion) {
    let word = "An   examplé sentence.";
    c.bench_function("clean_input", |b| b.iter(|| clean_input(black_box(word))));
}

fn input_to_words_bench(c: &mut Criterion) {
    let input = "an example sentence";
    let dictionary = load_wordlist(include_str!("../wordlist/english.txt"));
    c.bench_function("input_to_words", |b| {
        b.iter(|| input_to_words(black_box(input), black_box(&dictionary)))
    });
}

fn solve_bench(c: &mut Criterion) {
    let ciphertext = "x cbt tloap";
    let wordlist = [
        "many", "words", "here", "to", "test", "the", "solver", "also", "few", "a", "ok", "now",
        "all", "words", "should", "be", "good",
    ]
    .join("\n");
    let dictionary = load_wordlist(&wordlist);
    let cipher_words = input_to_words(ciphertext, &dictionary).unwrap();

    c.bench_function("solve", |b| {
        b.iter(|| Solver::new(&cipher_words).solve(HashMap::new(), None))
    });
}

fn apply_solution_bench(c: &mut Criterion) {
    let ciphertext = "x cbt tloap";
    let solution = Solution {
        key: [
            ('x', 'a'),
            ('c', 'f'),
            ('b', 'e'),
            ('t', 'w'),
            ('l', 'o'),
            ('o', 'r'),
            ('a', 'd'),
            ('p', 's'),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    c.bench_function("apply_solution", |b| {
        b.iter(|| solution.apply(black_box(ciphertext)))
    });
}

criterion_group!(
    benches,
    load_wordlist_bench,
    normalize_bench,
    clean_input_bench,
    input_to_words_bench,
    solve_bench,
    apply_solution_bench
);
criterion_main!(benches);
