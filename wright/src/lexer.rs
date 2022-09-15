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
pub fn lex(source: &str) -> Vec<Token> {
    // Return no tokens if there is no source.
    if source.is_empty() { return Vec::new(); }
    // Create output vec.
    let mut output: Vec<Token> = Vec::new();
    // For now (until I care enough to go back and optimize this, we'll just iterate through the characters of the string).
    // We used char_indices here though because I want to know my byte offset into the string.
    let mut iterator = source.char_indices();

    // Work our way through the iterator using a `while let` loop to destructructure the items as we work through and 
    // make it slightly clearer that we mutate the iterator during the loop if we find the start of a string.  
    while let Some((byte_index, character)) = iterator.next() {
        // Single character tokens are so common that I simplify the function to add them to the output vector here. 
        let mut emit_single_char_token = |variant: TokenTy| { 
            output.push(Token { variant, index: byte_index, length: 1}); 
        };

        // Figure out what type of token to generate here. This may consume an aditional item from the iterator if possible.
        match character {
            '(' => emit_single_char_token(TokenTy::LeftParen),
            ')' => emit_single_char_token(TokenTy::RightParen),
            '[' => emit_single_char_token(TokenTy::LeftSquare),
            ']' => emit_single_char_token(TokenTy::RightSquare),
            '{' => emit_single_char_token(TokenTy::LeftBracket),
            '}' => emit_single_char_token(TokenTy::RightBracket),
            _ => unimplemented!()
        }
    }

    unimplemented!()
}
