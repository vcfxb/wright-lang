/// Empty trait to define an expression superclass.
pub trait Expr: Statement {}

#[derive(Debug)]
/// Unary Operators
pub enum UnaryOperator {
    Not,
    Negative,
}
impl Expr for UnaryOperator {}

/// Empty trait to help define BinaryOperator as a superclass.
pub trait BinaryOperator: Expr {}

#[derive(Debug)]
/// Binary Operators
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    Increment,
    Decrement,
}
impl BinaryOperator for ArithmeticOperator {}

#[derive(Debug)]
/// Logical Operators
pub enum LogicalOperator {
    And,
    Or,
    Xor,
    ShortAnd,
    ShortOr,
}
impl BinaryOperator for LogicalOperator {}

#[derive(Debug)]
/// Relational Operators
pub enum RelationalOperators {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}
impl BinaryOperator for RelationalOperators {}

#[derive(Debug)]
/// Unary Expression.
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub left: Expr,
}
impl Expr for UnaryExpr {}

#[derive(Debug)]
/// Binary Expression.
pub struct BinaryExpr{
    pub left: Expr,
    pub operator: BinaryOperator,
    pub right: Expr,
}
impl Expr for BinaryExpr {}

#[derive(Debug)]
/// Literal struct.
pub struct Literal {
    /// For type inference, when and where possible.
    pub literal_type: Option<String>,
    pub literal: String,
}
impl Expr for Literal {}

#[derive(Debug)]
/// Identifier struct.
pub struct Identifier {
    pub id: String,
}
impl Expr for Identifier {}

#[derive(Debug)]
/// Function call.
pub struct Call {
    pub callee: String,
    pub args: Vec<Expr>,
}
impl Expr for Call {}

/// Empty trait for statements.
pub trait Statement {}

#[derive(Debug)]
/// Different types of assignments, one for mutability, the other for immutability.
pub enum Assign {
    Let,
    Var,
}

#[derive(Debug)]
/// Assignment statement.
pub struct Assignment {
    pub left: Identifier,
    pub assign_type: Assign,
    pub right: Expr,
}
impl Statement for Assignment {}

#[derive(Debug)]
/// Block of statements
pub struct Block {
    pub statements: Vec<Statement>,
}
impl Expr for Block {}

#[derive(Debug)]
/// Single conditional statement.
pub struct Condition {
    pub condition: Expr,
    pub block: Block,
}
impl Expr for Condition {}

#[derive(Debug)]
/// Conditional block
pub struct Conditional {
    pub conditions: Vec<Condition>,
}
impl Expr for Conditional {}

#[derive(Debug)]
/// While loop struct.
pub struct WhileLoop {
    pub condition: Expr,
    pub block: Block,
}
impl Statement for WhileLoop {}

#[derive(Debug)]
/// For loop struct.
pub struct ForLoop {
    pub assignment: Assignment,
    pub source_var: Identifier,
    pub block: Block,
}
impl Statement for ForLoop {}

#[derive(Debug)]
/// Function defining struct.
pub struct FunctionDefinition {
    pub id: Identifier,
    pub function_type: Option<String>,
    pub args: Vec<Expr>,
    pub block: Block,
}
impl Statement for FunctionDefinition {}

#[derive(Debug)]
/// Class declaration struct.
pub struct ClassDeclaration {
    pub id: Identifier,
    pub block: Block,
}
impl Statement for ClassDeclaration {}

#[derive(Debug)]
/// Constant struct.
pub struct Constant {
    pub id: Identifier,
    pub val: Expr,
}
impl Statement for Constant {}

#[derive(Debug)]
/// Return statement struct.
pub struct Return {
    pub val: Expr
}
impl Statement for Return {}

#[derive(Debug)]
/// Module struct
/// Module does not implement `Statement` because
/// users should not be able to define modules in a file.
/// Each Wright file should be it's own independent Module.
pub struct Module {
    pub id: Identifier,
    pub content: Block,
}
