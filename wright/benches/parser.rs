use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use wright::{
    ast::identifier::Identifier, lexer::Lexer, parser::Parser, source_tracking::{filename::FileName, source::Source, SourceMap}
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
