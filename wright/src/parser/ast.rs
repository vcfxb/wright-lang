#[derive(Clone, Debug)]
/// Expression enum.
pub enum Expression {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall(Call),
    Sub(Sub),
    Cast(Cast),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Unary Operators.
pub enum UnaryOperator {
    Not,
    Negative,
    BitwiseNot,
}

#[derive(Debug, Clone)]
/// Unary Expression.
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub right: Box<Expression>,
}

#[derive(Copy, Clone, Debug)]
/// Enum representing binary operators.
pub enum BinaryOperator {
    Arithmetic(ArithmeticOperator),
    Logical(LogicalOperator),
    Relational(RelationalOperator),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Arithmetic Operators.
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
/// Logical Operators.
pub enum LogicalOperator {
    And,
    Or,
    Xor,
    ShortAnd,
    ShortOr,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Relational Operators.
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
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
/// Subfield access expression. (i.e. "left.right").
pub struct Sub {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
/// Cast expression struct (i.e. "value to type").
pub struct Cast {
    pub value: Box<Expression>,
    pub to_type: Type,
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
}

#[derive(Debug, Clone)]
/// Type struct.
/// Neither an Expression or a Statement.
pub struct Type {
    pub id: String,
    pub type_parameters: Vec<Type>,
}

#[derive(Debug, Clone)]
/// Function call.
pub struct Call {
    pub callee: Identifier,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone)]
/// Enum for statements.
pub enum Statement {
    Block(Block),
    Expression(Expression),
    Assignment(Assignment),
    Reassignment(Reassignment),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    FnDef(FunctionDefinition),
    ClassDef(ClassDeclaration),
    TraitDef(TraitDeclaration),
    EnumDef(EnumDeclaration),
    ConstDef(Constant),
    Impl(ImplBlock),
    Return(Return),
    Break(Break),
    Continue(Continue),
    SingleCondition(Condition),
    Conditional(Conditional),
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
    pub right: Expression,
    pub declared_type: Option<Type>,
}

#[derive(Debug, Clone)]
/// Reassignment statement.
pub struct Reassignment {
    pub left: Identifier,
    pub right: Expression,
}

#[derive(Debug, Clone)]
/// Block of statements.
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
/// Single conditional statement.
pub struct Condition {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Conditional block.
pub struct Conditional {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
/// While loop struct.
pub struct WhileLoop {
    pub id: LoopAnnotation,
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// For loop struct.
pub struct ForLoop {
    pub id: LoopAnnotation,
    pub assignment: Expression,
    pub source_var: Expression,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Function defining struct.
pub struct FunctionDefinition {
    pub declared_type: Option<Type>,
    pub id: Identifier,
    pub args: Vec<Identifier>,
    pub visibility: Visibility,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Class declaration struct.
pub struct ClassDeclaration {
    pub generics: Vec<Type>,
    pub id: Type,
    pub visibility: Visibility,
    pub traits_implemented: Vec<Type>,
    pub fields: Vec<(Identifier, Visibility)>,
}

#[derive(Debug, Clone)]
/// Trait declaration struct.
pub struct TraitDeclaration {
    pub generics: Vec<Type>,
    pub id: Type,
    pub requires: Vec<Identifier>,
    pub visibility: Visibility,
    pub block: Block,
}

#[derive(Debug, Clone)]
/// Enum (tagged union) declaration struct.
pub struct EnumDeclaration {
    pub generics: Vec<Type>,
    pub id: Type,
    pub visibility: Visibility,
    pub variants: Vec<ClassDeclaration>,
}

#[derive(Debug, Clone)]
/// Struct for an impl block.
pub struct ImplBlock {
    pub generics: Vec<Type>,
    pub on_id: Identifier,
    pub for_trait_if_any: Option<Identifier>,
    pub contents: Block,
}

#[derive(Debug, Clone)]
/// Constant struct.
pub struct Constant {
    pub id: Identifier,
    pub visibility: Visibility,
    pub declared_type: Option<Type>,
    pub val: Box<Expression>,
}

#[derive(Debug, Clone)]
/// Return statement struct.
pub struct Return {
    pub val: Box<Expression>
}

#[derive(Debug, Clone)]
/// Loop annotation struct.
pub struct LoopAnnotation {
    pub id: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Enum for visibility.
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, Clone)]
/// Break statement struct.
pub struct Break {
    pub id: LoopAnnotation,
}

#[derive(Debug, Clone)]
/// Continue statement struct.
pub struct Continue {
    pub id: LoopAnnotation,
}

#[derive(Debug, Clone)]
/// Module struct.
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
            id: Identifier{id: name},
            content: Block{statements: vec![]}
        }
    }
}

