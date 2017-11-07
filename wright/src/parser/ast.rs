#[derive(Clone, Debug)]
/// Expression enum.
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall(Call),
    SingleCondition(Condition),
    Conditional(Conditional),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Unary Operators
pub enum UnaryOperator {
    Not,
    Negative,
}

#[derive(Debug, Clone)]
/// Unary Expression.
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub right: Box<Expr>,
}

#[derive(Copy, Clone, Debug)]
/// Enum representing binary operators.
pub enum BinaryOperator {
    Arithmetic(ArithmeticOperator),
    Logical(LogicalOperator),
    Relational(RelationalOperator),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Arithmetic Operators
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Logical Operators
pub enum LogicalOperator {
    And,
    Or,
    Xor,
    ShortAnd,
    ShortOr,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Relational Operators
pub enum RelationalOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}

#[derive(Debug, Clone)]
/// Binary Expression.
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
/// Literal struct.
pub struct Literal {
    pub literal: String,
}

#[derive(Debug, Clone)]
/// Identifier struct.
pub struct Identifier {
    pub id: String,
    pub declared_type: Option<String>,
}

#[derive(Debug, Clone)]
/// Function call.
pub struct Call {
    pub callee: Identifier,
    pub args: Vec<Box<Expr>>,
}

#[derive(Debug, Clone)]
/// Union for statements.
pub enum Statement {
    // no conditionals; they are in Expr
    Block(Block),
    Expression(Expr),
    Assignment(Assignment),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    FnDef(FunctionDefinition),
    ClassDef(ClassDeclaration),
    ConstDef(Constant),
    Return(Return),
    Break(Break),
    Continue(Continue),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Different types of assignments, one for mutability, the other for immutability.
pub enum Assigner {
    Let,
    Var,
}

#[derive(Debug, Clone)]
/// Assignment statement.
pub struct Assignment {
    pub left: Identifier,
    pub assign_type: Assigner,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
/// Block of statements
pub struct Block {
    pub statements: Vec<Box<Statement>>,
}

#[derive(Debug, Clone)]
/// Single conditional statement.
pub struct Condition {
    pub condition: Box<Expr>,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Conditional block
pub struct Conditional {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
/// While loop struct.
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// For loop struct.
pub struct ForLoop {
    pub assignment: Box<Expr>,
    pub source_var: Identifier,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Function defining struct.
pub struct FunctionDefinition {
    pub id: Identifier,
    pub args: Vec<Box<Expr>>,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Class declaration struct.
pub struct ClassDeclaration {
    pub id: Identifier,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Constant struct.
pub struct Constant {
    pub id: Identifier,
    pub val: Box<Expr>,
}

#[derive(Debug, Clone)]
/// Return statement struct.
pub struct Return {
    pub val: Box<Expr>
}

#[derive(Debug, Clone)]
/// Break statement struct.
pub struct Break {
    pub identifier: Identifier,
    pub val: Box<Expr>,
}

#[derive(Debug, Copy, Clone)]
/// Continue statement struct.
//  (empty)
pub struct Continue {}

#[derive(Debug)]
/// Module struct
/// Module is not in `Statement` because
/// users should not be able to define modules in a file.
/// Each Wright file should be it's own independent Module.
pub struct Module {
    pub id: Identifier,
    pub content: Block,
}

impl Module {
    pub fn new(name: String) -> Self {
        Module {
            id: Identifier{id: name, declared_type: None},
            content: Block{statements: vec![]}
        }
    }
}
