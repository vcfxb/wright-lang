//! Lexer benchmarks.

// use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
// use wright::parser::lexer::Lexer;

// fn bench_symbol_tokens(c: &mut Criterion) {
//     // Make a benchmark group.
//     let mut group = c.benchmark_group("lexer symbol benchmarks");

//     // Function to make a lexer and get a token from it.
//     fn make_lexer_and_get_token(b: &mut Bencher, input: &str) {
//         b.iter(|| black_box(Lexer::new(input).next_token()));
//     }

//     let inputs = ["+", "+=", "*", "@", "?"];

//     for i in inputs {
//         group.bench_with_input(format!("lexer {i}"), i, make_lexer_and_get_token);
//     }
// }

// fn bench_block_doc_comment(c: &mut Criterion) {
//     c.bench_function("lexer block style doc comment", move |b: &mut Bencher| {
//         b.iter(move || {
//             black_box(Lexer::new("/*! \n this is a block-style comment \n\n */").next_token())
//         });
//     });
// }

// criterion_group!(benches, bench_symbol_tokens, bench_block_doc_comment);
// criterion_main!(benches);

fn main() {}
