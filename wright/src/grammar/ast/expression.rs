use crate::grammar::ast::{BooleanLit, CharLit, Name, NumLit, SelfLit, Statement, StringLit};
use crate::grammar::model::Fragment;

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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Range,
    RangeInclusive,
    LogicalOr,
    LogicalAnd,
    Or,
    Xor,
    And,
    EqEq,
    NotEq,
    Lt,
    Gt,
    Le,
    Ge,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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

/// Type of range expression.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum RangeOperator {
    /// A RangeTo Expression of the form `..n`.
    RangeTo,
    /// A RangeFrom Expression of the form `n..`.
    RangeFrom,
    /// A RangeFromInclusive Expression of the form `..=n`.
    RangeToInclusive
}

/// A RangeTo, RangeFrom, or RangeToInclusive expression.
/// RangeTo: `..100`.
/// RangeFrom: `100..`.
/// RangeToInclusive: `..=100`.
///
/// Full range expressions are handled by the
/// [binary expression parser](struct.BinaryExpression.html).
#[derive(Debug, Clone)]
pub struct RangeExpression<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
    /// The range operator in use.
    pub op: RangeOperator,
    /// The expression being operated on.
    pub expr: Box<Expression<'s>>,
}

/// Unary expression operators.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    Name(Name<'s>),
    Parens(Parens<'s>),
    BinaryExpression(BinaryExpression<'s>),
    UnaryExpression(UnaryExpression<'s>),
    SelfLit(SelfLit<'s>),
    Block(Block<'s>),
    Conditional(Conditional<'s>),
    IndexExpression(IndexExpression<'s>),
    FuncCall(FuncCallExpression<'s>),
}
