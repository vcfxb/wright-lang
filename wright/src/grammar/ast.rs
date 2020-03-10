use crate::grammar::model::Fragment;

use multi_eq::*;
use wright_derive::*;

multi_eq_make_trait!(pub, AstEq, ast_eq);

/// Numerical literal in wright source code.
/// i.e. `10`, `0xCa1a0`, `0b0101_0101`, `100_000`
#[derive(Copy, Clone, Debug, AstEq)]
pub struct NumLit<'s> {
    /// Associated fragment of source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
    /// Represented value.
    #[ast_eq(cmp = "eq")]
    pub inner: u128,
}

/// Character literal in wright source code.
/// i.e `'a', '\n', '\u{01f441}', '\x00', '♦'`
/// see [this page](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes) for escape
/// information.
#[derive(Copy, Clone, Debug, AstEq)]
pub struct CharLit<'s> {
    /// Associated fragment of source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
    /// Represented Value.
    #[ast_eq(cmp = "eq")]
    pub inner: char,
}

/// String literal in wright source code.
/// i.e. `"hello world", "with \n newline \n escapes"`
/// The parser for string literals also includes all the escape characters
/// [here](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes).
/// Raw-strings and Byte-strings (like those in rust) are not currently
/// supported but may be added in the future.
#[derive(Clone, Debug, AstEq)]
pub struct StringLit<'s> {
    /// Associated fragment of source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
    /// Represented string value. (not a reference into source code because
    /// source code may contain escaped characters.)
    #[ast_eq(cmp = "eq")]
    pub inner: String,
}

/// Boolean literal in wright source code.
/// i.e. `true`, `false`.
#[derive(Copy, Clone, Debug, AstEq)]
pub struct BooleanLit<'s> {
    /// Associated fragment in source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
    /// Represented value.
    #[ast_eq(cmp = "eq")]
    pub inner: bool,
}

/// An identifier in Wright source code.
/// There is only one field here, the fragment of source code being referenced.
/// This is because the identifier itself will be the same as the fragment's
/// source.
#[derive(Copy, Clone, Debug, AstEq)]
pub struct Identifier<'s> {
    /// Fragment in wright source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
}

/// An underscore symbol. Underscores are their own expression in wright.
/// (Similarly to their use in Rust)
#[derive(Copy, Clone, Debug, AstEq)]
pub struct Underscore<'s> {
    /// Fragment in source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
    // todo: finish this struct.
}

/// A type in source code.
#[derive(Clone, Debug, AstEq)]
pub struct Type<'s> {
    /// Associated Fragment in source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
}

/// An expression in parentheses in wright source code.
#[derive(Clone, Debug, AstEq)]
pub struct Parens<'s> {
    /// Fragment in source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
    /// The expression between these parentheses.
    pub inner: Box<Expression<'s>>,
}

/// The type of binary operation being done.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, AstEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    AndAnd,
    Or,
    OrOr,
    Mod,
    Lt,
    Gt,
    Le,
    Ge,
    EqEq,
    NotEq,
    Eq,
    Xor,
    Dot,
    DotDot,
}

/// A binary expression in source code.
#[derive(Clone, Debug, AstEq)]
pub struct BinaryExpression<'s> {
    /// Fragment in source code.
    #[ast_eq(ignore)]
    pub frag: Fragment<'s>,
    /// Operation being done.
    pub op: BinaryOp,
    /// Left side of the expression.
    pub left: Box<Expression<'s>>,
    /// Right side of the expression.
    pub right: Box<Expression<'s>>,
}

/// An expression in wright source code.
#[derive(Clone, Debug, AstEq)]
#[allow(missing_docs)]
pub enum Expression<'s> {
    NumLit(NumLit<'s>),
    CharLit(CharLit<'s>),
    StringLit(StringLit<'s>),
    BooleanLit(BooleanLit<'s>),
    Identifier(Identifier<'s>),
    Underscore(Underscore<'s>),
    Parens(Parens<'s>),
    BinaryExpression(BinaryExpression<'s>),
}
