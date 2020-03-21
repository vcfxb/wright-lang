use crate::grammar::ast::{Block, Expression, ASTEq};
use crate::grammar::model::{Fragment, HasFragment};
use nom::IResult;
use crate::grammar::parsers::expression::ToExpression;
use nom::bits::complete::take;

impl<'s> Block<'s> {
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
        let len = usize::max(fst.statements.len(), snd.statements.len());
        let res = match (fst.result, snd.result) {
            (Some(a), Some(b)) => ASTEq::ast_eq(&*a, &*b),
            (None, None) => true,
            _ => false
        };
        res && fst.statements.iter()
            .map(|a| Some(*a))
            .chain(
                (0..)
                    .iter()
                    .map(|_| None)
            )
            .zip(
                snd.statements.iter()
                    .map(|a| Some(*a))
                    .chain(
                        (0..)
                            .iter()
                            .map(|_| None)
                    )
            )
            .take(len)
            .all(|(a,b)|
                match (a, b) {
                    (Some(a), Some(b)) => ASTEq::ast_eq(&a, &b),
                    (None, None) => true,
                    _ => false
                }
            )
    }
}