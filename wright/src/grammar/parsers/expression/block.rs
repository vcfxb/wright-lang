use crate::grammar::ast::{eq::AstEq, Block, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use nom::IResult;

impl<'s> Block<'s> {
    /// Parse a block in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment, Self> {
        todo!()
    }
}

impl<'s> HasFragment<'s> for Block<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for Block<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::Block(self)
    }
}

impl<'s> AstEq for Block<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.result, &snd.result) && AstEq::ast_eq(&fst.statements, &snd.statements)
    }
}
