//! Primary expressions in Qright source code. 

use crate::parser::ast::path::Path;
use super::{literal::Literal, parentheses::ParenthesesExpression};

/// A primary expression is a special type of low-level expression that can appear in places where other expressions 
/// (such as blocks or conditionals) are not allowed. 
#[derive(Debug)]
pub enum Primary<'src> {
    /// A literal in source code.
    Literal(Literal<'src>),
    /// A path to an item/symbol/constant value. 
    /// 
    /// This includes identifiers as single element paths. 
    Path(Path<'src>),
    /// An expression in parentheses. 
    Parentheses(ParenthesesExpression<'src>)
}

