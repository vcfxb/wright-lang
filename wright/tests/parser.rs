#![cfg(all(feature = "reporting", feature = "parser"))]

use termcolor::Buffer;
use wright::{
    ast::identifier::Identifier,
    lexer::Lexer,
    parser::Parse,
    source_tracking::{filename::FileName, source::Source, SourceMap, SourceRef},
};

#[test]
fn test_parse_fail_identifier_to_diagnostic() -> anyhow::Result<()> {
    let map: SourceMap = SourceMap::new();
    let source_ref: SourceRef = map.add(Source::new_from_static_str(FileName::None, "12345"));
    let mut lexer = Lexer::new(source_ref);
    let parse_error = Identifier::parse(&mut lexer).unwrap_err();
    let mut buffer = Buffer::no_color();

    parse_error
        .as_diagnostic()
        .write(&map, &mut buffer, &Default::default())?;

    assert_eq!(
        std::str::from_utf8(buffer.as_slice())?,
        "\
    error[WPE3]: expected identifier
  ┌─ <NO_NAME>:1:1
  │
1 │ 12345
  │ ^^^^^\n\n"
    );

    Ok(())
}
