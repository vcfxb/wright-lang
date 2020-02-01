use crate::grammar::ast::{
    BinaryExpression, BinaryOp, BooleanLit, CharLit, Expression, Identifier, NumLit, Parens,
    StringLit, Underscore,
};
use crate::grammar::model::Fragment;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::IResult;

impl<'s> Expression<'s> {
    pub fn frag(&self) -> Fragment<'s> {
        match self {
            Expression::NumLit(inner) => inner.frag,
            Expression::CharLit(inner) => inner.frag,
            Expression::StringLit(inner) => inner.frag,
            Expression::BooleanLit(inner) => inner.frag,
            Expression::Identifier(inner) => inner.frag,
            Expression::Underscore(inner) => inner.frag,
            Expression::Parens(inner) => inner.frag,
            Expression::BinaryExpression(inner) => inner.frag,
        }
    }

    fn new_bin_expr(
        frag: Fragment<'s>,
        left: Expression<'s>,
        op: BinaryOp,
        right: Expression<'s>,
    ) -> Expression<'s> {
        Expression::BinaryExpression(BinaryExpression {
            frag,
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn parse_parens(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression> {
        map(
            delimited(char('('), Expression::parse, char(')')),
            |inner| {
                Expression::Parens(Parens {
                    frag: input,
                    inner: Box::new(inner),
                })
            },
        )(input)
    }

    fn parse_atom(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        macro_rules! generalize_parser {
            ($parse_fcn:expr, $wrap_fcn:expr) => {
                |input| $parse_fcn(input).map(|(input, inner)| (input, $wrap_fcn(inner)))
            };
        }

        alt((
            generalize_parser!(BooleanLit::parse, Expression::BooleanLit),
            generalize_parser!(CharLit::parse, Expression::CharLit),
            generalize_parser!(Identifier::parse, Expression::Identifier),
            generalize_parser!(NumLit::parse, Expression::NumLit),
            generalize_parser!(StringLit::parse, Expression::StringLit),
            generalize_parser!(Underscore::parse, Expression::Underscore),
        ))(input)
    }

    fn parse_factor(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression> {
        alt((Self::parse_parens, Self::parse_atom))(input)
    }

    fn parse_mul_prec(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_factor(input)?;

        fold_many0(
            pair(
                alt((
                    map(char('*'), |_| BinaryOp::Mul),
                    map(char('/'), |_| BinaryOp::Div),
                    map(char('%'), |_| BinaryOp::Mod),
                )),
                Self::parse_factor,
            ),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag(), left, op, right),
        )(input)
    }

    fn parse_add_prec(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_mul_prec(input)?;

        fold_many0(
            pair(
                alt((
                    map(char('+'), |_| BinaryOp::Add),
                    map(char('-'), |_| BinaryOp::Sub),
                )),
                Self::parse_mul_prec,
            ),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag(), left, op, right),
        )(input)
    }

    fn parse_and(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_add_prec(input)?;

        fold_many0(
            pair(map(char('&'), |_| BinaryOp::And), Self::parse_add_prec),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag(), left, op, right),
        )(input)
    }

    fn parse_xor(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_and(input)?;

        fold_many0(
            pair(map(char('^'), |_| BinaryOp::Xor), Self::parse_and),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag, left, op, right),
        )(input)
    }

    fn parse_or(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_xor(input)?;

        fold_many0(
            pair(map(char('|'), |_| BinaryOp::Or), Self::parse_xor),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag, left, op, right),
        )(input)
    }

    fn parse_cmp(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_or(input)?;

        fold_many0(
            pair(
                alt((
                    map(tag("<="), |_| BinaryOp::Le),
                    map(tag(">="), |_| BinaryOp::Ge),
                    map(tag("=="), |_| BinaryOp::Eq),
                    map(tag("!="), |_| BinaryOp::NotEq),
                    map(tag("<"), |_| BinaryOp::Lt),
                    map(tag(">"), |_| BinaryOp::Gt),
                )),
                Self::parse_or,
            ),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag, left, op, right),
        )(input)
    }

    fn parse_and_and(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_cmp(input)?;

        fold_many0(
            pair(map(tag("&&"), |_| BinaryOp::AndAnd), Self::parse_cmp),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag, left, op, right),
        )(input)
    }

    fn parse_or_or(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        let (input, left) = Self::parse_and_and(input)?;

        fold_many0(
            pair(map(tag("||"), |_| BinaryOp::OrOr), Self::parse_and_and),
            left,
            |left, (op, right)| Self::new_bin_expr(left.frag, left, op, right),
        )(input)
    }

    fn parse_dot_dot(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            tuple((
                Self::parse_or_or,
                map(tag(".."), |_| BinaryOp::DotDot),
                Self::parse_or_or,
            )),
            |(left, op, right)| Self::new_bin_expr(input, left, op, right),
        )(input)
    }

    fn parse_eq(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(
            tuple((
                Self::parse_dot_dot,
                map(char('='), |_| BinaryOp::Eq),
                Self::parse_dot_dot,
            )),
            |(left, op, right)| Self::new_bin_expr(input, left, op, right),
        )(input)
    }

    /// Parse a binary expression
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        context("expected binary expression", Self::parse_eq)(input)
    }
}
