use std::fmt;

/// Binary operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Modulo.
    Mod,
    /// Logical OR. (`||`)
    LOR,
    /// Bitwise OR. (`|`)
    OR,
    /// Logical AND. (`&&`)
    LAND,
    /// Bitwise AND. (`&`)
    AND,
    /// XOR. (`^`)
    XOR,
    /// Equal.
    EQ,
    /// Not Equal.
    NE,
    /// Less than.
    LT,
    /// Greater than.
    GT,
    /// Greater than or equal.
    GTE,
    /// Less than or equal.
    LTE,
    /// Left bit shift.
    LShift,
    /// Signed (Arithmetic) right shift.
    RShift,
    /// Unsigned (Logical) right shift.
    URShift,
    Assign(Box<BinaryOp>)
}

/// Unary operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOp {
    /// Negate.
    Neg,
    /// Logical NOT. (!)
    LNOT,
    /// Bitwise NOT. (~)
    NOT,
}

// todo
#[derive(Debug, Clone)]
pub struct Type<'a> {
    pub identifier: &'a str,
    pub polymorphic_parameters: Vec<Type<'a>>
}

// todo
#[derive(Debug, Clone)]
pub enum Expression<'e> {
    Identifier(&'e str),
    LitInt(u64),
    LitFloat(f64),
    LitStr(&'e str),
    BinaryExpr(BinaryOp, Box<Expression<'e>>, Box<Expression<'e>>),
    UnaryExpr(UnaryOp, Box<Expression<'e>>),
    Conditional {
        /// Length or conditions must be at least 1. (if ... { ... })
        /// Can be equal to length of content. (if ... { ... } else if ... { ... })
        /// Or can be equal to one less than the length of content (if ... { ... } else { ... })
        conditions: Vec<Expression<'e>>,
        content: Vec<Vec<Expression<'e>>>,
    },
    Cast(Box<Expression<'e>>, Type<'e>),
    ForEachLoop {
        tag: &'e str,
        // todo
    },
    WhileLoop {
        tag: &'e str,
        condition: Box<Expression<'e>>,
        content: Vec<Statement<'e>>
    },
    Loop {
        tag: &'e str,
        content: Vec<Statement<'e>>
    },
    Return(Box<Expression<'e>>),
    Statement(Box<Expression<'e>>),
}
// todo
#[derive(Debug, Clone)]
pub enum Statement<'s> {
    EnumDeclaration,
    StructDeclaration,
    ClassDeclaration,
    FunctionDeclaration,
    VariableDeclaration,
    Documentation,
    Impl,
    FunctionAlias,
    TypeAlias(Type<'s>, Type<'s>),
}