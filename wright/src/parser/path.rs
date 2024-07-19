//! [Parse] implementation for [Path].

use super::error::ParserErrorKind;
use super::whitespace::optional_whitespace;
use super::Parser;
use super::{error::ParserError, Parse};
use crate::ast::identifier::Identifier;
use crate::ast::path::Path;
use crate::lexer::{self, Lexer};

impl Parse for Path {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        unimplemented!()
    }
}

/// Parse the first (and possibly only) [Identifier] in the [Path].
fn parse_head(parser: &mut Parser) -> Result<Identifier, ParserError> {
    Identifier::parse(parser)
        .map_err(|mut err| {
            err.kind = ParserErrorKind::ExpectedPath;
            err
        })
}


// /// Parse a path (`head::tail`) in source code.
// pub fn parse_path<'src>(parser_state: &mut ParserState<'src>) -> NodeParserResult<Path<'src>> {
//     // Get the initial index to make metadata at the end.
//     let initial_index = parser_state.index();

//     // Parse the head of the path and destructure the parser success.
//     let head = parse_identifier(parser_state)
//         // Replace the error with a missing path error.
//         .map_err(|mut parser_error| {
//             // Replace the parser error text.
//             parser_error.ty = ParserErrorVariant::Expected(
//                 "fully qualified symbol reference (path) or identifier",
//             );
//             parser_error
//         })?;

//     // Parse the tail of the path. Map through Box::new to create neccesary heap allocation.
//     let tail = parse_path_tail(parser_state).map(Box::new);
//     // Make the metadata for the produced AST node.
//     let meta = parser_state.make_ast_node_meta(initial_index, parser_state.index() - initial_index);
//     // Return Ok.
//     Ok(Path { meta, head, tail })
// }

// /// Parse the tail of a path, ignoring any whitespace encountered and producing an [`Option`] with a [`Path`].
// ///
// /// This will update the parser state's cursor incrementally, avoiding leaving it partially between two tokens in
// /// the tail or past the whitespace at the end of the tail.
// fn parse_path_tail<'src>(parser_state: &mut ParserState<'src>) -> NodeParserOption<Path<'src>> {
//     // Get the initial index of the parser.
//     let initial_index = parser_state.index();
//     // Make a clone of the parser state to parse path parts incrementally on.
//     let mut scoped_state = parser_state.clone();
//     // Allow ignored whitespace/comment between parts of the path.
//     // This will turn into None and return early if we peek a multi-line unterminated comment.
//     ignore_whitespace_and_comments(&mut scoped_state).ok()?;
//     // Parse the double colon.
//     // Returns early if this returns none and the next token is not a double colon.
//     scoped_state.next_token_if_ty_eq(TokenTy::ColonColon)?;
//     // Allow ignored whitespace/comments after the double colon.
//     // This will turn into None and error out if we peek a multi-line unterminated comment.
//     ignore_whitespace_and_comments(&mut scoped_state).ok()?;
//     // Parse the head of the tail. If this errors, return none and do not update parser state.
//     let head = parse_identifier(&mut scoped_state).ok()?;
//     // Update the parser state after parsing the head so that the parent function does not re-parse it.
//     *parser_state = scoped_state.clone();
//     // Parse the rest of the tail. If this returns None we have reached the end of the path.
//     // Map througgh Box::new to create a heap allocation and prevent infinite stack nesting.
//     let tail = parse_path_tail(&mut scoped_state).map(Box::new);
//     // Update the parser state if there was a parsed tail. If there was not, do not update the parser state as
//     // it may have greedily consumed various whitespace, comments, and double colons.
//     if tail.is_some() {
//         *parser_state = scoped_state;
//     }

//     // Make AST node metadata using the initial index and the current index.
//     let meta = parser_state.make_ast_node_meta(initial_index, parser_state.index() - initial_index);
//     // Return the parsed tail combined into a path.
//     Some(Path { meta, head, tail })
// }

// #[cfg(test)]
// mod test_path {
//     use crate::{
//         filemap::{FileMap, FileName},
//         parser::state::ParserState,
//     };

//     use super::parse_path;

//     /// Test the simple case path.
//     #[test]
//     fn test_simple_path() {
//         let source = "test::path";

//         let mut file_map = FileMap::new();
//         let file_id = file_map.add(FileName::Test("test input"), source.to_owned());
//         let mut parser_state = ParserState::new(&file_map, file_id);
//         let path = parse_path(&mut parser_state).expect("parses successfully");

//         assert_eq!(path.head.matching_source(), "test");
//         assert!(path.tail.is_some());
//         assert_eq!(path.tail.unwrap().head.matching_source(), "path");
//     }
// }
