//! An expression in parentheses in Wright source code.

use super::Expression;
use crate::parser::{ast::metadata::AstNodeMeta, state::ParserState, util::NodeParserResult};

#[derive(Debug)]
pub struct ParenthesesExpression<'src> {
    /// The AST node metadata.
    pub meta: AstNodeMeta<'src>,
    /// The body of this block as an expression.
    pub body: Box<Expression<'src>>,
}

/// Parse an expression in parentheses from source code.
pub fn parse_parentheses_expr<'src>(
    parser_state: &mut ParserState<'src>,
) -> NodeParserResult<ParenthesesExpression<'src>> {
    unimplemented!()
}
