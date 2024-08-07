//! Token models.

use crate::source_tracking::fragment::Fragment;
use derive_more::Display;

/// A token in wright source code.
#[derive(Debug, Display)]
#[display(fmt = "\"{}\" ({:?})", "fragment.as_str()", variant)]
pub struct Token {
    /// What type of token this is.
    pub variant: TokenTy,
    /// The matching fragment of source code -- this contains the location and length data for the token.
    pub fragment: Fragment,
}

/// The different types of tokens in wright source.
#[rustfmt::skip] // Turn off auto reformat. 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Allow missing docs (most of these should be self-evident). 
#[allow(missing_docs)]
pub enum TokenTy {
    LeftCurly, RightCurly,
    LeftBracket, RightBracket,
    LeftParen, RightParen,

    Plus, PlusEq,
    Star, StarEq,
    Div, DivEq,
    Xor, XorEq,
    Mod, ModEq,
    Bang, BangEq,

    Minus, MinusEq, SingleArrow,
    Eq, EqEq, DoubleArrow,

    Lt, LtEq, LtLt,
    Gt, GtEq, GtGt,
    And, AndEq, AndAnd,
    Or, OrEq, OrOr,
    Colon, ColonEq, ColonColon,

    At,
    Tilde,
    Semi,
    Dot,
    Comma,
    Hash,
    Question,
    Dollar,
    
    // Not in the same group as the other ones there since it can be used at the start of identifiers.
    Underscore,

    Identifier,

    OuterDocComment, OuterBlockDocComment,
    InnerDocComment, InnerBlockDocComment,
    
    /// Indicates a block style comment without termination. 
    /// Separate from [TokenTy::InnerDocComment] and [TokenTy::OuterDocComment] to indicate that 
    /// unterminated comments will be handled differently (produce errors eventually). 
    UnterminatedBlockComment,

    KwRecord,
    KwType,
    KwEnum,
    KwUnion,
    KwFunc,
    KwRepr,
    KwImpl,
    KwConstraint,
    KwReferences,
    KwTrait,
    KwUse,
    KwAs,
    KwConst,
    KwMod,
    KwIf,
    KwElse,
    KwMatch,
    KwFor,
    KwIn,
    KwWhile,
    KwTrue,
    KwFalse,
    KwLoop,
    KwWhere,

    IntegerLiteral,
    StringLiteral { terminated: bool }, 
    FormatStringLiteral { terminated: bool },
    CharLiteral { terminated: bool },

    /// Whitespace counts as a token.
    Whitespace,

    /// Unknown character in lexer fragment. 
    Unknown
}
