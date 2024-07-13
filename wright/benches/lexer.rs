//! Lexer benchmarks.

use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use wright::{
    lexer::Lexer,
    source_tracking::{filename::FileName, source::Source},
};

fn make_test_lexer(s: &str) -> Lexer {
    let source = Source::new_from_string(FileName::None, s.to_owned());
    Lexer::new(Arc::new(source))
}

fn bench_symbol_tokens(c: &mut Criterion) {
    // Make a benchmark group.
    let mut group = c.benchmark_group("lexer symbol benchmarks");

    // Function to make a lexer and get a token from it.
    fn make_lexer_and_get_token(b: &mut Bencher, input: &str) {
        b.iter(|| black_box(make_test_lexer(input).next_token()));
    }

    let inputs = ["+", "+=", "*", "@", "?"];

    for i in inputs {
        group.bench_with_input(format!("lexer {i}"), i, make_lexer_and_get_token);
    }
}

fn bench_block_doc_comment(c: &mut Criterion) {
    c.bench_function("lexer block style doc comment", move |b: &mut Bencher| {
        b.iter(move || {
            black_box(make_test_lexer("/*! \n this is a block-style comment \n\n */").next_token())
        });
    });
}

criterion_group!(benches, bench_symbol_tokens, bench_block_doc_comment);
criterion_main!(benches);
