use crate::grammar::ast::Expression;
use crate::grammar::model::Fragment;
use nom::IResult;

impl<'s> Expression<'s> {
    /// Parse an expression
    pub fn parse(_input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        unimplemented!()
    }
}
