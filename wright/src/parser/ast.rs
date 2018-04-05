// todo: module level docs

use std::fmt::Debug;

/// Binary operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Or,
    And,
    Xor,
    Eq,
    Ne,
    Lt,
    Gt,
    Ge,
    Le,
    //Assign(BinaryOp)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOp {Neg, Not, Increment, Decrement}

#[derive(Debug, Clone)]
enum Expression<'e> {
    LitInt(u64),
    LitStr(&'e str),
    BinaryExpr(BinaryOp, Box<Expression<'e>>, Box<Expression<'e>>),
    UnaryExpr(UnaryOp, Box<Expression<'e>>)
}

#[derive(Debug, Clone)]
enum Statement<'s> {
    Expr(Expression<'s>)
}
