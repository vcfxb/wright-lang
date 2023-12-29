//! AST node representation and parsing implementation for string literals. 

use std::rc::Rc;

use crate::parser::{ast::{metadata::AstNodeMeta, expression::literal::escapes::unescape}, state::ParserState, util::NodeParserResult, lexer::{tokens::{TokenTy, Token}, IndexedToken}, error::{ParserError, ParserErrorVariant}};

/// The value of a string literal in source code. 
#[derive(Debug, Clone)]
pub enum StringLiteralValue<'src> {
    /// A string literal in source code without any escapes can be represented directly
    /// using a reference into the source code. This will refer to the string literal without the 
    /// opening and closing quotatation marks. 
    WithoutEscapes(&'src str),

    /// A string literal in source code with escapes must be represented using an owned string, as
    /// we have to do some processing to resolve all the escapes into the actual unescaped unicaode string. 
    /// We store this in an [`Rc`] to make cloning less expensive, as we will not need to mutate this string
    /// while it's in the AST. 
    WithEscapes(Rc<str>)
}

impl<'src> StringLiteralValue<'src> {
    pub fn as_str(&self) -> &str {
        match self {
            StringLiteralValue::WithoutEscapes(s) => s,
            StringLiteralValue::WithEscapes(rc) => rc.as_ref(),
        }
    }
}

/// A string literal in source code. 
#[derive(Debug)]
pub struct StringLit<'src> {
    /// The metadata about this node. 
    pub meta: AstNodeMeta<'src>,
    /// A reference counted owned string representing the parsed value. 
    pub value: StringLiteralValue<'src>,
    /// Format strings are denoted using '`' instead of '"'. Treat these similarly to string literals. 
    pub is_format_string: bool,
}

impl<'src> StringLit<'src> {
    /// Parse a string literal from source code. If there is not a [`TokenTy::StringLit`]
    /// available from the parser state's lexer, then this will not mutate the parser state. 
    pub fn parse(parser_state: &mut ParserState<'src>) -> NodeParserResult<Self> {
        // Peek the type of the next token or error out if there is not one. 
        let peeked_token_ty = parser_state
            .peek_token_ty()
            // Dereferencing map here to prevent complaining about ref after mut borrow. 
            .map(|token_ty: &TokenTy| *token_ty)
            // If there is not a next token, error out. 
            .ok_or(ParserError { byte_range: parser_state.peek_byte_range(), ty: ParserErrorVariant::Expected("string literal") })?;

        // Mathc on the next token type available from the lexer. 
        match peeked_token_ty {
            // Unterminated string literals produce an error.
            TokenTy::StringLit { is_terminated: false, .. } => Err(parser_state.peek_byte_range_into_error(ParserErrorVariant::UnterminatedStringLiteral)),

            // Terminated string literals produce a value. 
            TokenTy::StringLit { is_format, .. } => {
                // Peek the important parts of the token. 
                let IndexedToken { index, token: Token { length, .. } } = *parser_state.peek_token().unwrap();
                // Get the associated part of source code, making an immutable reference into the parser state. 
                let full_matching_source: &str = &parser_state.source[index..index+length];
                // Get a reference to the body of the string literal itself (without the quotes or backticks for format
                // strings).
                let string_lit_body: &str = &full_matching_source[1..(full_matching_source.len()-1)];

                // Try to unescape the string literal. 
                match unescape(string_lit_body) {
                    Ok(str_lit_value) => {},
                    Err(str_lit_errors) => {},
                }
                
                

                unimplemented!()
            }

            // All other token types produce an error.
            _ => Err(parser_state.peek_byte_range_into_error(ParserErrorVariant::Expected("string literal"))),
        }
    }
}
