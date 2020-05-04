use crate::grammar::ast::{
    eq::AstEq, BinaryExpression, BinaryOp, Expression,
};
use crate::grammar::model::{Fragment, HasFragment};
use nom::IResult;

/// Operator parsing functions.
pub(crate) mod operator;

/// Primary parsing functions used in manual recursive descent parsing.
pub(crate) mod primary;

impl<'s> BinaryExpression<'s> {
    fn new(
        frag: Fragment<'s>,
        left: impl Into<Expression<'s>>,
        op: BinaryOp,
        right: impl Into<Expression<'s>>,
    ) -> Self {
        Self {
            frag,
            left: Box::new(left.into()),
            op,
            right: Box::new(right.into()),
        }
    }

    /// Create a new Binary Expression by merging two subexpressions, and adding
    /// a given operator between them. This assumes that the fragment merging of
    /// the sub expressions will not fail.
    ///
    /// ## Panics:
    /// Panics when the Fragment::merge fails on the children.
    pub(self) fn new_merge(left: impl Into<Expression<'s>>,
                 op: BinaryOp,
                 right: impl Into<Expression<'s>>
    ) -> Self {
        let e1 = left.into();
        let e2 = right.into();
        // currently use unwrap here. fragment merging should not fail
        // internally.
        let frag =
            Fragment::merge(e1.get_fragment(), e2.get_fragment())
                .unwrap();
        Self::new(frag, e1, op, e2)
    }



    /// Parse a binary expression in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        todo!("binary expression parser")
    }

}

impl<'s> HasFragment<'s> for BinaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> Into<Expression<'s>> for BinaryExpression<'s> {
    fn into(self) -> Expression<'s> {
        Expression::BinaryExpression(self)
    }
}

impl<'s> AstEq for BinaryExpression<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.op == snd.op
            && AstEq::ast_eq(&*fst.left, &*snd.left)
            && AstEq::ast_eq(&*fst.right, &*snd.right)
    }
}
