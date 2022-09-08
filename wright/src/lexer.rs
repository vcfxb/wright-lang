//! The wright lexer. This module is responsible for lexical analysis and initial processing of source code. 

/// Token of Wright source code. 
#[derive(Clone, Copy)]
pub struct Token {
    /// What type of token is it?
    variant: TokenTy,
    /// Where is it? (byte index into source file useful via a [crate::codemap::FileMap])
    index: usize,
    /// How many bytes of source code long is it? Note this doesn't necessarily mean how many characters long it is.
    length: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenTy {
    LeftParen,      // (
    RightParen,     // )
    Bang,           // !
    BangEq,         // !=
    Tilde,          // ~
    At,             // @
    Mod,            // %
    ModEq,          // %=
    Xor,            // ^
    XorEq,          // ^=
    And,            // &
    AndEq,          // &=
    AndAnd,         // &&
    Star,           // *
    StarEq,         // *=
    Plus,           // +
    PlusEq,         // +=
    Minus,          // -
    MinusEq,        // -=
    Gt,             // >
    GtEq,           // >=
    Lt,             // <
    LtEq,           // <=
    Eq,             // =
    EqEq,           // ==
    Div,            // /
    DivEq,          // /=
    Semi,           // ;
    Colon,          // :
    Question,       // ?
    Dot,            // .
    Comma,          // ,
    LeftSquare,     // [
    RightSquare,    // ]
    LeftBracket,    // {
    RightBracket,   // }

    /// Integer literal. This is a literal integer in source code.
    IntegerLit,

    /// A string literal in source code. 
    StringLit,

    /// A character literal in source code.
    CharLit,

    /// A non-keyword identifier in source code (such as a variable name). 
    Identifier, 

    /// A keyword (such as 'class' 'struct' or 'enum')
    Keyword(Keyword)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Let,
    Mut,
    Fn,
    Class,
    Struct,
    Impl,
    If,
    Else,
}

pub enum LexerError {
    /// Unfinished string literal. 
    UnfinishedStringLit { 
        /// Byte location in source file of the first quote. 
        start: usize 
    }
}

/// Read a source file and produce a series of tokens (aka lexemes) representing the source code for transformation into 
/// an AST. Ignore comments (lines starting with #, anythign between #* and *#). Return error instead of series of tokens
/// if there is an unfinished sting or character literal. 
pub fn lex()
