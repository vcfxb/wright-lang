//! Lexer benchmarks.


use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wright::parser::lexer::Lexer;

fn bench_lex_plus_eq(c: &mut Criterion) {
    c.bench_function("lex +=", |b| b.iter(|| {
        Lexer::new(black_box("+=")).next_token();
    }));
}

criterion_group!(benches, bench_lex_plus_eq);
criterion_main!(benches);
