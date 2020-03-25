use crate::grammar::ast::{eq::ASTEq, Conditional};
use crate::grammar::model::{Fragment, HasFragment};
use nom::IResult;

impl<'s> Conditional<'s> {
    /// Parse a conditional expression in source code.
    pub fn parse(_input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        unimplemented!()
    }
}

impl<'s> HasFragment<'s> for Conditional<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ASTEq for Conditional<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        ASTEq::ast_eq(&fst.default, &snd.default);
        todo!()
    }
}
