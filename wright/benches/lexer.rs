//! Lexer benchmarks.

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use wright::parser::lexer::Lexer;

fn bench_symbol_tokens(c: &mut Criterion) {
    // Make a benchmark group.
    let mut group = c.benchmark_group("lexer symbol benchmarks");

    // Function to make a lexer and get a token from it.
    fn make_lexer_and_get_token(b: &mut Bencher, input: &str) {
        b.iter(|| Lexer::new(black_box(input)).next_token());
    }

    let inputs = ["+", "+=", "*", "@", "?"];

    for i in inputs {
        group.bench_with_input(format!("lexer {i}"), i, make_lexer_and_get_token);
    }
}

criterion_group!(benches, bench_symbol_tokens);
criterion_main!(benches);
