use crate::grammar::ast::{BooleanLit, CharLit, NumLit, SelfLit, Statement, StringLit, ScopedName};
use crate::grammar::model::Fragment;
use std::fmt::Debug;

/// An expression in parentheses in wright source code.
#[derive(Clone, Debug)]
pub struct Parens<SourceCodeReference: Clone + Debug> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// The expression between these parentheses.
    pub inner: Box<Expression<SourceCodeReference>>,
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
pub struct BinaryExpression<SourceCodeReference: Debug + Clone> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// Operation being done.
    pub op: BinaryOp,
    /// Left side of the expression.
    pub left: Box<Expression<SourceCodeReference>>,
    /// Right side of the expression.
    pub right: Box<Expression<SourceCodeReference>>,
}

/// Type of range expression.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum RangeOperator {
    /// A RangeTo Expression of the form `..n`.
    RangeTo,
    /// A RangeFrom Expression of the form `n..`.
    RangeFrom,
    /// A RangeFromInclusive Expression of the form `..=n`.
    RangeToInclusive,
}

// FIXME: link
/// A RangeTo, RangeFrom, or RangeToInclusive expression.
/// RangeTo: `..100`.
/// RangeFrom: `100..`.
/// RangeToInclusive: `..=100`.
///
/// Full range expressions are handled by the
/// [binary expression parser](struct.BinaryExpression.html).
#[derive(Debug, Clone)]
pub struct RangeExpression<SourceCodeReference: Clone + Debug> {
    /// Associated source code.
    pub frag: SourceCodeReference,
    /// The range operator in use.
    pub op: RangeOperator,
    /// The expression being operated on.
    pub expr: Box<Expression<SourceCodeReference>>,
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
pub struct UnaryExpression<SourceCodeReference: Clone + Debug> {
    /// Associated source code.
    pub frag: SourceCodeReference,
    /// The operation being done.
    pub op: UnaryOp,
    /// The expression being operated on.
    pub inner: Box<Expression<SourceCodeReference>>,
}

/// A block in source code.
#[derive(Clone, Debug)]
pub struct Block<SourceCodeReference: Clone + Debug> {
    /// The associated source code.
    pub frag: SourceCodeReference,
    /// The statements in this block.
    pub statements: Vec<Statement<SourceCodeReference>>,
    /// The optional return/result expression.
    pub result: Option<Box<Expression<SourceCodeReference>>>,
}

/// A conditional expression in wright source code.
#[derive(Clone, Debug)]
pub struct Conditional<SourceCodeReference: Clone + Debug> {
    /// The associated source code.
    pub source: SourceCodeReference,
    /// The primary condition.
    pub primary: (Box<Expression<SourceCodeReference>>, Block<SourceCodeReference>),
    /// All of the secondary conditions (note that the field is called `elifs`
    /// but in wright code they should use `else if`).
    pub elifs: Vec<(Expression<SourceCodeReference>, Block<SourceCodeReference>)>,
    /// The optional else block at the end of the conditional.
    pub default: Option<Block<SourceCodeReference>>,
}

/// Indexing expressions such as `array[1]` in wright source code.
#[derive(Clone, Debug)]
pub struct IndexExpression<SourceCodeReference: Clone + Debug> {
    /// The associated source code.
    pub source: SourceCodeReference,
    /// The thing being indexed into. Usually a list, string, or map.
    pub subject: Box<Expression<SourceCodeReference>>,
    /// The indexing key. The thing that goes in the brackets.
    pub object: Box<Expression<SourceCodeReference>>,
}

/// Function call expressions such as `foo(bar, baz)` in wright source code.
#[derive(Clone, Debug)]
pub struct FuncCall<SourceCodeReference: Clone + Debug> {
    /// The associated source code.
    pub source: SourceCodeReference,
    /// The function being called
    pub func: Box<Expression<SourceCodeReference>>,
    /// The arguments passed to the function
    pub args: Vec<Expression<SourceCodeReference>>,
}

/// An expression in wright source code.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Expression<SourceCodeReference: Clone + Debug> {
    NumLit(NumLit<SourceCodeReference>),
    CharLit(CharLit<SourceCodeReference>),
    StringLit(StringLit<SourceCodeReference>),
    BooleanLit(BooleanLit<SourceCodeReference>),
    ScopedName(ScopedName<SourceCodeReference>),
    Parens(Parens<SourceCodeReference>),
    BinaryExpression(BinaryExpression<SourceCodeReference>),
    UnaryExpression(UnaryExpression<SourceCodeReference>),
    SelfLit(SelfLit<SourceCodeReference>),
    Block(Block<SourceCodeReference>),
    Conditional(Conditional<SourceCodeReference>),
    IndexExpression(IndexExpression<SourceCodeReference>),
    FuncCall(FuncCall<SourceCodeReference>),
}
