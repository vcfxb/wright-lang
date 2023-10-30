//! Structures used for representing expressions in wright source code.

use self::{literal::{integer::IntegerLiteral, boolean::BooleanLiteral}, parentheses::ParenthesesExpression};
use super::{identifier::Identifier, path::Path};

pub mod block;
pub mod literal;
pub mod parentheses;

/// Enumeration of all the different kinds of expression in wright.
#[derive(Debug)]
pub enum Expression<'src> {
    /// A literal in source code. 
    Literal(Literal<'src>),
    // Block(block::Block<'src>),
}

/// A primary expression is a special type of low-level expression that can appear in places where other expressions 
/// (such as blocks or conditionals) are not allowed. 
pub enum Primary<'src> {
    /// A literal in source code.
    Literal(Literal<'src>),
    /// An identifier refering to a variable/item/symbol in scope. 
    Identifier(Identifier<'src>),
    /// A path to an item/symbol/constant value. 
    Path(Path<'src>),
    /// An expression in parentheses. 
    Parentheses(ParenthesesExpression<'src>)
}

#[derive(Debug)]
pub enum Literal<'src> {
    /// An integer literal in source code.
    Integer(IntegerLiteral<'src>),
    /// A boolean literal in source code. 
    Boolean(BooleanLiteral<'src>),
}

// macro_rules! unary_expr {
//     (
//         $(#[$meta:meta])*
//         $name:ident
//     ) => {
//         $(#[$meta])*
//         #[derive(Clone, Debug)]
//         pub struct $name {
//             pub expr: Box<Expression>,
//         }

//         impl $name {
//             paste::paste! {
//                 #[doc = "Construct a new [`" $name "`] around a given [`Expression`]."]
//                 #[allow(unused)]
//                 #[inline]
//                 pub fn new(expr: Expression) -> Self {
//                     Self {
//                         expr: Box::new(expr)
//                     }
//                 }
//             }
//         }
//     };
// }

// macro_rules! unary_exprs {
//     (
//         $(
//             $(#[$meta:meta])*
//             $name:ident,
//         )*
//     ) => {
//         $(
//             unary_expr! {
//                 $(#[$meta])*
//                 $name
//             }
//         )*
//     };
// }

// /// An expression in wright source code.
// #[derive(Clone, Debug, From)]
// pub enum Expression {
//     Block(Block),
//     Parens(Parens),
//     Bang(Bang),
//     Tilde(Tilde),
// }

// unary_exprs! {
//     /// Bang (`!expr`) is the logical not operator.
//     Bang,

//     /// Tilde (`~expr`) is used as the bitwise not operator.
//     Tilde,

//     /// Parens (`(expr)`) is used for expression grouping.
//     Parens,

//     /// Block (`{expr}`) is used for scope.
//     Block,
// }
