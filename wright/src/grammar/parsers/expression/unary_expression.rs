use crate::grammar::ast::{eq::ASTEq, Expression, UnaryExpression, UnaryOp};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;

impl UnaryOp {}

impl<'s> UnaryExpression<'s> {
    /// Parse a unary expression in source code.
    pub fn parse(_input: Fragment<'s>) -> Self {
        todo!()
    }
}

impl<'s> ToExpression<'s> for UnaryExpression<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::UnaryExpression(self)
    }
}

impl<'s> HasFragment<'s> for UnaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ASTEq for UnaryExpression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.op == snd.op && ASTEq::ast_eq(&*fst.inner, &*snd.inner)
    }
}
