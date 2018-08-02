//! Module for parsing Wright Source code to an AST (Abstract Syntax Tree).

pub mod ast;

//use self::ast::*;
//use ::grammar::*;
use ::codespan::*;
use std::fmt::Debug;
use std::ops::Add;

/// For representing AST nodes with their source spans.
#[derive(Clone, Debug)]
pub struct Spanned<T: Debug + Clone> {
    /// The AST node.
    pub inner: T,
    /// It's ByteSpan in the source code.
    pub span: ByteSpan,
}

impl<T: Debug + Clone> Spanned<T> {
    /// Construct a new spanned Element.
    pub fn new(node: T, start: usize, end: usize) -> Spanned<T> {
        Spanned{inner: node, span:Span::new(ByteIndex(start as u32), ByteIndex(end as u32))}
    }
}