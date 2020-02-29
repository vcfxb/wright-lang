use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::with_input;
use crate::grammar::parsers::expression::ToExpression;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::value;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};
use nom::IResult;

/// alt() for one or more parsers (instead of two or more).
fn alt_single(first: BinaryOp, rest: &[BinaryOp]) -> impl Fn(Fragment) -> IResult<Fragment, BinaryOp> + '_ {
    move |input| {
        let mut res = value(first, tag(first.token()))(input);
        if res.is_err() {
            for op in rest {
                res = value(*op, tag(op.token()))(input);
                if res.is_ok() {
                    break;
                }
            }
        }
        res
    }
}

/// Parses a left-associative binary expression
/// `left op right` using a left fold operation.
fn binary_parser<'s, E>(ends: E, first: BinaryOp, rest: &'s [BinaryOp]) ->
    impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>>
    where E: Fn(Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>>,
{
    move |input| {
        let (input, left) = ends(input)?;
        let inner = {
            pair(
                delimited(
                    multispace0,
                    alt_single(first, rest),
                    multispace0,
                ),
                &ends,
            )
        };
        let r = fold_many0(
            with_input(&inner),
            left,
            |left, (frag, (op, right))| {
                Expression::BinaryExpression(
                    BinaryExpression::new(frag, left, op, right),
                )
            },
        )(input);
        r
    }
}

macro_rules! binary_parser {
    ($ends:expr, $op:expr) => {
        binary_parser($ends, $op, &[])
    };
    ($ends:expr, $first:expr, $($rest:expr),+) => {
        binary_parser($ends, $first, &[$($rest,)*])
    };
}

impl<'s> BinaryExpression<'s> {
    fn new(frag: Fragment<'s>, left: Expression<'s>, op: BinaryOp, right: Expression<'s>) -> Self {
        Self {
            frag,
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    fn dot(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Expression::primary,
            BinaryOp::Dot
        )(input)
    }

    fn dotdot(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::dot,
            BinaryOp::DotDot
        )(input)
    }

    fn mul(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::dotdot,
            BinaryOp::Mul,
            BinaryOp::Div,
            BinaryOp::Mod
        )(input)
    }

    fn add(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::mul,
            BinaryOp::Add,
            BinaryOp::Sub
        )(input)
    }

    fn and(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::add,
            BinaryOp::And
        )(input)
    }

    fn xor(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::and,
            BinaryOp::Xor
        )(input)
    }

    fn or(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::xor,
            BinaryOp::Or
        )(input)
    }

    fn andand(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::or,
            BinaryOp::AndAnd
        )(input)
    }

    fn oror(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::andand,
            BinaryOp::OrOr
        )(input)
    }

    fn cmp(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::oror,
            BinaryOp::Lt,
            BinaryOp::Gt,
            BinaryOp::Le,
            BinaryOp::Ge
        )(input)
    }

    fn eq(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        binary_parser!(
            Self::cmp,
            BinaryOp::EqEq,
            BinaryOp::NotEq
        )(input)
    }

    /// Parse a binary expression in source code.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        Self::eq(input)
    }
}

impl<'s> HasFragment<'s> for BinaryExpression<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for BinaryExpression<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::BinaryExpression(self)
    }
    fn parse_self(f: Fragment<'s>) -> IResult<Fragment<'s>, Self> {Self::parse(f)}
}

impl BinaryOp {
    /// The operator token used for this binary operator.
    pub fn token(&self) -> &'static str {
        use BinaryOp::*;
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            And => "&",
            AndAnd => "&&",
            Or => "|",
            OrOr => "||",
            Mod => "%",
            Lt => "<",
            Gt => ">",
            Le => "<=",
            Ge => ">=",
            EqEq => "==",
            NotEq => "!=",
            Xor => "^",
            Dot => ".",
            DotDot => "..",
        }
    }
}
