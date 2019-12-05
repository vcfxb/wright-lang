use codespan::{Span, ByteIndex};
use crate::grammar::lexer::symbols::{SymTy, Sym};
use codespan_reporting::diagnostic::Diagnostic;
use either::Either;

#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenTy {
    String(String),
    Char(char),
    Word(String),
    Number(u128),
    Comment(String),
    Sym(SymTy),
}

/// Token in wright source code.
#[derive(Debug, Clone)]
pub struct Token {
    span: Span,
    ty: TokenTy,
}

impl Token {
    fn new(span: Span, ty: TokenTy) -> Self {
        Self {span, ty}
    }

    /// Makes a list of tokens from a list of symbols.
    pub fn parse(syms: &Vec<Sym>) -> Result<Vec<Self>, Diagnostic> {
        // escaped characters first
        unimplemented!()
    }
}