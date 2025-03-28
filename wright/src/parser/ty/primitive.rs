//! Parsing for primitive type signatures.

use crate::{ast::ty::{AtomicTy, AtomicTyVariant}, lexer::token::TokenTy, parser::{error::{ParserError, ParserErrorKind}, Parser}, source_tracking::fragment::Fragment};

impl AtomicTy {
    /// Parse an atomic primitive type from souce or error with [ParserErrorKind::ExpectedAtomicTypeSignature]
    /// and no progress made on the given [Parser].
    #[rustfmt::skip]
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {

        // Local function reused by the match block below to shorten unwrapping a fragment and initializing the new 
        // value once a match has been found.
        fn accept(variant: AtomicTyVariant, parser: &mut Parser) -> Result<AtomicTy, ParserError> {
            Ok(AtomicTy { variant, matching_source: parser.next_token().unwrap().unwrap().fragment })
        }
        
        match parser.peek_variant() {
            Some(TokenTy::KwBool) => accept(AtomicTyVariant::Bool, parser),
            Some(TokenTy::KwChar) => accept(AtomicTyVariant::Char, parser),
            Some(TokenTy::KwU8  ) => accept(AtomicTyVariant::U8,   parser),
            Some(TokenTy::KwI8  ) => accept(AtomicTyVariant::I8,   parser),
            Some(TokenTy::KwU16 ) => accept(AtomicTyVariant::U16,  parser),
            Some(TokenTy::KwI16 ) => accept(AtomicTyVariant::I16,  parser),
            Some(TokenTy::KwU32 ) => accept(AtomicTyVariant::U32,  parser),
            Some(TokenTy::KwI32 ) => accept(AtomicTyVariant::I32,  parser),
            Some(TokenTy::KwF32 ) => accept(AtomicTyVariant::F32,  parser),
            Some(TokenTy::KwU64 ) => accept(AtomicTyVariant::U64,  parser),
            Some(TokenTy::KwI64 ) => accept(AtomicTyVariant::I64,  parser),
            Some(TokenTy::KwF64 ) => accept(AtomicTyVariant::F64,  parser),
            _ => Err(ParserErrorKind::ExpectedAtomicTypeSignature.at(parser.peek_fragment_or_rest_cloned())),
        }
    }
}
