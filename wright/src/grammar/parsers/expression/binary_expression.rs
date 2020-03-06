use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::with_input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, value};
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

/// alt() for one or more parsers (instead of two or more).
fn alt_single(
    first: BinaryOp,
    rest: &[BinaryOp],
) -> impl Fn(Fragment) -> IResult<Fragment, BinaryOp> + '_ {
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
fn binary_parser<'s, E>(
    ends: E,
    first: BinaryOp,
    rest: &'s [BinaryOp],
) -> impl Fn(Fragment<'s>) -> IResult<Fragment<'s>, BinaryExpression<'s>>
where
    E: Fn(Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>>,
{
    move |input| {
        // first (leftmost) inner binary expression
        let (input, leftmost) = map(
            with_input(tuple((
                &ends,
                delimited(multispace0, alt_single(first, rest), multispace0),
                &ends,
            ))),
            |(frag, (left, op, right))| BinaryExpression::new(frag, left, op, right),
        )(input)?;
        // collapse consecutive binary expressions into
        // a single binary expression
        fold_many0(
            with_input(pair(
                delimited(multispace0, alt_single(first, rest), multispace0),
                &ends,
            )),
            leftmost,
            |left, (frag, (op, right))| {
                BinaryExpression::new(frag, Expression::BinaryExpression(left), op, right)
            },
        )(input)
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

macro_rules! bin_end {
    ($inner:expr) => {
        alt((
            map($inner, Expression::BinaryExpression),
            Expression::primary,
        ))
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

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn dot(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            Expression::primary,
            BinaryOp::Dot
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn dotdot(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::dot),
            BinaryOp::DotDot
        )(input)
    }

    fn mul(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::dotdot),
            BinaryOp::Mul,
            BinaryOp::Div,
            BinaryOp::Mod
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn add(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::mul),
            BinaryOp::Add,
            BinaryOp::Sub
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn and(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::add),
            BinaryOp::And
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn xor(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::and),
            BinaryOp::Xor
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn or(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::xor),
            BinaryOp::Or
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn andand(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::or),
            BinaryOp::AndAnd
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn oror(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::andand),
            BinaryOp::OrOr
        )(input)
    }

    fn cmp(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::oror),
            BinaryOp::Lt,
            BinaryOp::Gt,
            BinaryOp::Le,
            BinaryOp::Ge
        )(input)
    }

    #[rustfmt::skip] // allow easy addition of new binary operators
    fn eq(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        binary_parser!(
            bin_end!(Self::cmp),
            BinaryOp::EqEq,
            BinaryOp::NotEq
        )(input)
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

    /// Parse a binary expression in source code.
    fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        alt((
            Self::eq,
            Self::cmp,
            Self::oror,
            Self::andand,
            Self::or,
            Self::xor,
            Self::and,
            Self::add,
            Self::mul,
            Self::dotdot,
            Self::dot,
        ))(input)
    }
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
