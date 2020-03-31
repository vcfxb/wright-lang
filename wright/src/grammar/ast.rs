/// Module to test equality between two AST Nodes.
pub mod eq;

#[cfg(test)]
mod eq_tests;

use crate::grammar::model::Fragment;

/// Numerical literal in wright source code.
/// i.e. `10`, `0xCa1a0`, `0b0101_0101`, `100_000`
#[derive(Copy, Clone, Debug)]
pub struct NumLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented value.
    pub inner: u128,
}

/// Character literal in wright source code.
/// i.e `'a', '\n', '\u{01f441}', '\x00', '♦'`
/// see [this page](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes) for escape
/// information.
#[derive(Copy, Clone, Debug)]
pub struct CharLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented Value.
    pub inner: char,
}

/// String literal in wright source code.
/// i.e. `"hello world", "with \n newline \n escapes"`
/// The parser for string literals also includes all the escape characters
/// [here](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes).
/// Raw-strings and Byte-strings (like those in rust) are not currently
/// supported but may be added in the future.
#[derive(Clone, Debug)]
pub struct StringLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented string value. (not a reference into source code because
    /// source code may contain escaped characters.)
    pub inner: String,
}

/// Boolean literal in wright source code.
/// i.e. `true`, `false`.
#[derive(Copy, Clone, Debug)]
pub struct BooleanLit<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
    /// Represented value.
    pub inner: bool,
}

/// `self` literal in wright source code.
#[derive(Copy, Clone, Debug)]
pub struct SelfLit<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
}

/// An identifier in Wright source code.
/// There is only one field here, the fragment of source code being referenced.
/// This is because the identifier itself will be the same as the fragment's
/// source.
#[derive(Copy, Clone, Debug)]
pub struct Identifier<'s> {
    /// Fragment in wright source code.
    pub frag: Fragment<'s>,
}

/// A scoped, or qualified, name.
#[derive(Clone, Debug)]
pub struct ScopedName<'s> {
    /// The source code fragment.
    pub frag: Fragment<'s>,
    /// The sequence of simple identifiers.
    /// Example: foo::bar::baz -> [ foo, bar, baz ]
    pub names: Vec<Identifier<'s>>,
}

/// An expression in parentheses in wright source code.
#[derive(Clone, Debug)]
pub struct Parens<'s> {
    /// Fragment in source code.
    pub frag: Fragment<'s>,
    /// The expression between these parentheses.
    pub inner: Box<Expression<'s>>,
}

/// The type of binary operation being done.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, IntoEnumIterator)]
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
    Xor,
    // removed Dot, Walrus, for reconsideration.
    DotDot,
}

/// A binary expression in source code.
#[derive(Clone, Debug)]
pub struct BinaryExpression<'s> {
    /// Fragment in source code.
    pub frag: Fragment<'s>,
    /// Operation being done.
    pub op: BinaryOp,
    /// Left side of the expression.
    pub left: Box<Expression<'s>>,
    /// Right side of the expression.
    pub right: Box<Expression<'s>>,
}

/// Unary expression operators.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum UnaryOp {
    LogicalNot,
    BitwiseNot,
    Neg,
}

/// A unary expression in source code.
#[derive(Clone, Debug)]
pub struct UnaryExpression<'s> {
    /// Fragment in source code.
    pub frag: Fragment<'s>,
    /// The operation being done.
    pub op: UnaryOp,
    /// The expression being operated on.
    pub inner: Box<Expression<'s>>,
}

/// A block in source code.
#[derive(Clone, Debug)]
pub struct Block<'s> {
    /// The fragment in source code.
    pub frag: Fragment<'s>,
    /// The statements in this block.
    pub statements: Vec<Statement<'s>>,
    /// The optional return/result expression.
    pub result: Option<Box<Expression<'s>>>,
}

/// A conditional expression in wright source code.
#[derive(Clone, Debug)]
pub struct Conditional<'s> {
    /// The associated fragment of source code.
    pub frag: Fragment<'s>,
    /// The primary condition.
    pub primary: (Box<Expression<'s>>, Block<'s>),
    /// All of the secondary conditions (note that the field is called `elifs`
    /// but in wright code they should use `else if`).
    pub elifs: Vec<(Expression<'s>, Block<'s>)>,
    /// The optional else block at the end of the conditional.
    pub default: Option<Block<'s>>,
}

/// Indexing expressions such as `array[1]` in wright source code.
#[derive(Clone, Debug)]
pub struct IndexExpression<'s> {
    /// The fragment in source code.
    pub frag: Fragment<'s>,
    /// The thing being indexed into. Usually a list, string, or map.
    pub subject: Box<Expression<'s>>,
    /// The indexing key. The thing that goes in the brackets.
    pub object: Box<Expression<'s>>,
}

/// An expression in wright source code.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Expression<'s> {
    NumLit(NumLit<'s>),
    CharLit(CharLit<'s>),
    StringLit(StringLit<'s>),
    BooleanLit(BooleanLit<'s>),
    Identifier(Identifier<'s>),
    ScopedName(ScopedName<'s>),
    Parens(Parens<'s>),
    BinaryExpression(BinaryExpression<'s>),
    UnaryExpression(UnaryExpression<'s>),
    SelfLit(SelfLit<'s>),
    Block(Block<'s>),
    Conditional(Conditional<'s>),
    IndexExpression(IndexExpression<'s>),
}

/// An expression whose results aren't used.
#[derive(Clone, Debug)]
pub struct ExpressionStatement<'s> {
    /// Associated Fragment in source code.
    pub frag: Fragment<'s>,
    /// The expression.
    pub inner: Box<Expression<'s>>,
}

/// A statement in wright source code.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Statement<'s> {
    ExpressionStatement(ExpressionStatement<'s>),
}

/// A type in source code.
#[derive(Clone, Debug)]
pub struct Type<'s> {
    /// Associated Fragment in source code.
    pub frag: Fragment<'s>,
    // todo: type parsing and ast
}

/// A Pattern used in pattern matching.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Pattern<'s> {
    UnderscorePattern(UnderscorePattern<'s>),
}

/// An underscore pattern in source code.
#[derive(Copy, Clone, Debug)]
pub struct UnderscorePattern<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
}
