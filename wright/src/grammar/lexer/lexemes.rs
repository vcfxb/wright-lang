use codespan::Span;
use crate::grammar::lexer::symbols::{SymTy};
use crate::grammar::lexer::tokens::Token;
use codespan_reporting::diagnostic::Diagnostic;

/// Numerical id identifying the pair number of a pair of parentheses or braces.
pub type PairingId = u128;

#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum LexemeTy {
    // keywords
    Class, Struct, Enum, Union, Pub, Fn, Trait, Impl, SelfType, SelfVar, Import,
    Mod, Type, As, Is, Match, If, Then, Else, Let, Mut,
    BoolLit(bool),
    DocComment(String),
    ModComment(String),
    StringLit(String),
    CharLit(char),
    NumLit(u128),
    Identifier(String),
    StartParen(PairingId),
    EndParen(PairingId),
    StartBracket(PairingId),
    EndBracket(PairingId),
    StartCurly(PairingId),
    EndCurly(PairingId),
    StartAngle(PairingId),
    EndAngle(PairingId),
    Sym(SymTy)
}

/// A lexeme in source code.
#[derive(Debug, Clone)]
pub struct Lexeme {
    span: Span,
    ty: LexemeTy
}

impl Lexeme {
    fn new(span: Span, ty: LexemeTy) -> Self {
        Self {span, ty}
    }

    /// Parse lexemes from tokens.
    pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Self>, Diagnostic> {
        unimplemented!()
    }
}