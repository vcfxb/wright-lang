//! Parsing implementation for [Path].

use std::sync::Arc;

use super::Parser;
use super::error::ParserError;
use super::error::ParserErrorKind;
use crate::ast::identifier::Identifier;
use crate::ast::path::Path;
use crate::lexer::token::TokenTy;
use crate::source_tracking::fragment::Fragment;

impl Path {
    /// Parse a [Path] from the given [Parser]. This is greedy (as much path as possible will be parsed).
    /// [Path]s of size 1 (just a single identifier) are accepted.
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        let head: Identifier = parse_head(parser)?;
        let mut tail = Vec::new();

        // Parse the tail.
        while let Some(ident) = parse_segment(parser) {
            tail.push(ident);
        }

        // Calculate the fragment containing the whole path.
        let last = tail.last().unwrap_or(&head);
        let matched_source_range = head.fragment.range.start..last.fragment.range.end;

        Ok(Path {
            // Head and tail should all have the same source ref since they came from the same parser.
            full_path: Fragment {
                source: Arc::clone(&head.fragment.source),
                range: matched_source_range,
            },
            head,
            tail,
        })
    }
}

/// Parse the first (and possibly only) [Identifier] in the [Path].
fn parse_head(parser: &mut Parser) -> Result<Identifier, ParserError> {
    Identifier::parse(parser).map_err(|mut err| {
        err.kind = ParserErrorKind::ExpectedPath;
        err
    })
}

/// Attempt to parse a segment of this path indivisbly (never just parse a seperator without another [Identifier]
/// at the end of it).
fn parse_segment(parser: &mut Parser) -> Option<Identifier> {
    if parser.matches_ignore_whitespace(&[TokenTy::ColonColon, TokenTy::Identifier]) {
        // Consume any whitespace
        parser.consume_optional_whitespace();

        // Skip the double colon
        parser.advance(1);

        // Consume any more whitespace
        parser.consume_optional_whitespace();

        // Consume the identifier we know is at the end of the sequence.
        // SAFETY: We just checked/matched that this parser ends with an identifier.
        return Some(unsafe { Identifier::parse(parser).unwrap_unchecked() });
    }

    // If the sequence doesn't match, there's no segment/tail.
    None
}

#[cfg(test)]
mod test_path {
    use crate::{
        ast::path::Path,
        lexer::Lexer,
        parser::Parser,
        source_tracking::{SourceMap, filename::FileName, source::Source},
    };

    /// Test the simple case path.
    #[test]
    fn test_ok_paths() {
        let map = SourceMap::new();
        let sources = &["test::path", "test :: path", "test ::path", "test:: path"];

        for source in sources {
            dbg!(source);
            let source_ref = map.add(Source::new_from_static_str(FileName::None, *source));
            let lexer = Lexer::new(source_ref);
            let mut parser = Parser::new(lexer);
            let path = Path::parse(&mut parser).unwrap();
            assert_eq!(path.head.fragment.as_str(), "test");
            assert_eq!(path.tail[0].fragment.as_str(), "path");
            assert_eq!(path.full_path.len(), source.len());
            assert_eq!(parser.lexer.bytes_remaining(), 0);
        }
    }

    #[test]
    fn test_not_paths() {
        let sources = &["", "0", "_"];

        for source in sources {
            let mut parser = Parser::new(Lexer::new_test(source));
            assert_eq!(Path::parse(&mut parser).unwrap_err().location.as_str(), *source);
        }
    }
}
