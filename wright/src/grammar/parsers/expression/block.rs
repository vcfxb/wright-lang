use crate::grammar::ast::{Block, Expression, eq::ASTEq};
use crate::grammar::model::{Fragment, HasFragment};
use nom::IResult;
use crate::grammar::parsers::expression::ToExpression;

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
    fn create_expr(self) -> Expression<'s> {Expression::Block(self)}
}

impl<'s> ASTEq for Block<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        ASTEq::ast_eq(&fst.result, &snd.result) &&
        ASTEq::ast_eq(&fst.statements, &snd.statements)
    }
}