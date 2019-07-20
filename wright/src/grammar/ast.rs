#![allow(missing_docs)]
//! Abstract Syntax Tree

use super::Properties;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(u64),
    String(String),
    Boolean(bool),
    Char(char),
    Underscore,
    SelfVar,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub properties: Properties,
    pub variant: ExprVariant,
}

#[derive(Debug, Clone)]
pub enum ExprVariant {
    UnaryExpr(UnaryExpr),
    Literal(Literal),
    BinaryExpr(BinaryExpr),
    Id(Identifier),
    Block(Block),
    Conditional(Conditional),
    Cast(Cast),
}

#[derive(Debug, Clone)]
/// Wrapper struct for unary expressions.
pub struct UnaryExpr {
    pub properties: Properties,
    pub variant: UnaryExprVariant,
    pub expr: Box<Expr>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryExprVariant {
    Not, Neg,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub properties: Properties,
    pub operator: BinaryExprVariant,
    pub left: Box<Expr>,
    pub right: Box<Expr>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryExprVariant {
    /// a\[b\]
    ArrayAccess,
    /// a.b
    MemberAccess,
    /// a(b)
    FunctionCall,
    /// a * b
    Multiply,
    /// a / b
    Divide,
    /// a % b
    Modulo,
    /// a + b
    Add,
    /// a - b
    Subtract,
    /// a << b
    LeftShift,
    /// a >> b
    RightShift,
    /// a >>> b
    SignedRightShift,
    /// a < b
    LT,
    /// a <= b
    LTE,
    /// a > b
    GT,
    /// a >= b
    GTE,
    /// a == b
    Equality,
    /// a != b
    Inequality,
    /// a and b
    AND,
    /// a or b
    OR,
    /// a ^ b
    XOR,
    /// a = b
    Assignment,
    /// a := b
    Gopher,
    /// a += b
    AddAssign,
    /// a -= b
    SubAssign,
    /// a *= b
    MultAssign,
    /// a /= b
    DivAssign,
    /// a %= b
    ModAssign,
    /// a ^= b
    XORAssign,
    /// a <<= b
    LeftShiftAssign,
    /// a >>= b
    RightShiftAssign,
    /// a >>>= b
    SignedRightShiftAssign,
    /// a ; b
    ApplyOperator,
    /// a & b
    Amp,
    /// a @ b
    AtOperator,
    /// a $ b
    Cash,
    /// a | b
    Bar,
    /// a ~ b
    Tilda,
    /// a ~> b
    TildaArrowRight,
    /// a <~ b
    TildaArrowLeft,
}

#[derive(Debug, Clone)]
pub struct Conditional {
    pub initial: IfExpression,
    pub else_ifs: Vec<IfExpression>,
    pub else_expr: Option<Box<Expr>>
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Box<Expr>,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub properties: Properties,
    pub inner: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub tag: Option<Identifier>,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
/// expr => type
pub struct Cast {
    pub prop: Properties,
    pub expr: Box<Expr>,
    pub ty: Type,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Primitive {
    /// u8
    Byte,
    Char,
    Str,
    /// i32
    Int,
    /// u64
    Long
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(Primitive),
    SelfType,
    UserType(Identifier),
    Generic(TypeWithGeneric),
    /// For type-level constants.
    Constant(Box<Expr>),
    Array(ArrayType),
    Function(FunctionType)
}

#[derive(Debug, Clone)]
pub struct TypeWithGeneric {
    pub innner: Box<Type>,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub inner: Box<Type>,
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub arg_types: Vec<Type>,
    pub return_type: Box<Type>,
}

