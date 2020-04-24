use crate::grammar::model::Fragment;
use crate::grammar::ast::{Statement, NumLit, CharLit, StringLit, BooleanLit, SelfLit};

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
    /// Example: foo::bar::baz -> [ foo, bar ]
    pub path: Vec<Identifier<'s>>,
    /// The final simple identifier
    /// Example: foo::bar::baz -> baz
    pub name: Identifier<'s>,
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

/// Function call expressions such as `foo(bar, baz)` in wright source code.
#[derive(Clone, Debug)]
pub struct FuncCallExpression<'s> {
    /// The fragment in source code.
    pub frag: Fragment<'s>,
    /// The function being called
    pub func: Box<Expression<'s>>,
    /// The arguments passed to the function
    pub args: Vec<Expression<'s>>,
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
    FuncCall(FuncCallExpression<'s>),
}