use std::sync::Arc;

use criterion::{Bencher, Criterion, black_box, criterion_group, criterion_main};
use wright::{
    ast::identifier::Identifier,
    lexer::Lexer,
    parser::Parser,
    source_tracking::{SourceMap, filename::FileName, source::Source},
};

fn bench_parse_identifier(c: &mut Criterion) {
    c.bench_function("parse identifier", move |b: &mut Bencher| {
        let map = SourceMap::new();
        let source_ref = map.add(Source::new_from_static_str(FileName::None, "test_ident"));
        b.iter(|| {
            let parser = Parser::new(Lexer::new(Arc::clone(&source_ref)));
            Identifier::parse(&mut black_box(parser)).unwrap()
        });
    });
}

criterion_group!(benches, bench_parse_identifier);
criterion_main!(benches);
