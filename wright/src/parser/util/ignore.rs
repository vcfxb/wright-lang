//! Utility features to ignore whitespace and comments in source code. 

use crate::parser::{state::ParserState, lexer::tokens::{TokenTy, CommentTy}, error::{ParserError, ParserErrorVariant}};
use super::NodeParserResult;

/// Parse through any/all whitespace and comments from the lexer. Return an error if any unterminated 
/// comment is encountered. Does not parse through doc-comments. 
pub fn ignore_whitespace_and_comments<'src>(parser_state: &mut ParserState<'src>) -> NodeParserResult<()> {
    // Use an infinite loop and only break out when we cannot parse either another whitespace or another comment. 
    while let Some(peeked_token_ty) = parser_state.peek_token_ty() {
        match peeked_token_ty {
            // Any of the following can be safely ignored. 
            | TokenTy::Whitespace 
            | TokenTy::SingleLineComment { comment_type: CommentTy::Normal } 
            | TokenTy::MultilineComment { comment_type: CommentTy::Normal, is_terminated: true } 
            => {
                // Discard the next token.
                let _ = parser_state.next_token();
            },

            // Any unterminated multiline comment will cause an error. 
            TokenTy::MultilineComment { is_terminated: false, .. } => {
                return Err(ParserError {
                    byte_range: parser_state.peek_byte_range(),
                    ty: ParserErrorVariant::UnterminatedMultilineComment,
                })
            },

            // Any token peeked that is not handled above is important -- return Ok
            // and let caller handle. 
            _ => { return Ok(()); }
        }
    }

    // If the lexer hits its end, return Ok. 
    Ok(())
}
