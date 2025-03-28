#![cfg(all(feature = "reporting", feature = "parser"))]

use termcolor::Buffer;
use wright::{
    ast::identifier::Identifier,
    lexer::Lexer,
    parser::Parser,
    source_tracking::{SourceMap, SourceRef, filename::FileName, source::Source},
};

#[test]
fn test_parse_fail_identifier_to_diagnostic() {
    let map: SourceMap = SourceMap::new();
    let source_ref: SourceRef = map.add(Source::new_from_static_str(FileName::None, "12345"));
    let mut parser = Parser::new(Lexer::new(source_ref));
    let parse_error = Identifier::parse(&mut parser).unwrap_err();
    let mut buffer = Buffer::no_color();

    parse_error
        .as_diagnostic()
        .write(&map, &mut buffer, &Default::default())
        .unwrap();

    assert_eq!(
        std::str::from_utf8(buffer.as_slice()).unwrap(),
        "\
    error: expected identifier
  ┌─ <NO_NAME>:1:1
  │
1 │ 12345
  │ ^^^^^\n\n"
    );
}
