use super::Properties;
use super::ast::Literal;
pub use pom::parser::Parser;
use pom::parser::{is_a, seq, one_of, empty, not_a, call, end, sym};
use pom::char_class::*;
use crate::grammar::ast::{Expr, ExprVariant, Identifier};
use crate::grammar::tokens::*;

/// Wright Parser type.
pub type WrightParser<ASTNode> = Parser<'static, u8, ASTNode>;
type Unit = ();
type Ignored = WrightParser<Unit>;

fn with_properties<T: 'static>(p: WrightParser<T>) -> WrightParser<(Properties, T)> {
    (empty().pos() + p + empty().pos())
        .map(|((s,i),e)| (Properties::new(s,e), i))
}

fn newline(c: u8) -> bool {
    c == b'\n' || c == b'\r'
}

/// Parses one or more whitespace and discards.
pub fn whitespace_required() -> Ignored {
    is_a(multispace).repeat(1..).discard()
}

/// Parses through whitespace, if there is any.
pub fn whitespace_optional() -> Ignored {
    is_a(multispace).repeat(0..).discard()
}

/// Parses through a single line comment.
fn single_line_comment() -> Ignored {
    (seq(LINE_COMMENT.as_bytes()) +
        not_a(newline).repeat(0..) +
        (is_a(newline).discard() | end()))
        .discard()
}

/// Parses through a multiline comment.
///
/// Note: this is a recursive function that can handle nested comments.
fn multiline_comment() -> Ignored {
    (seq(START_MULTILINE_COMMENT.as_bytes()) +
        (
            !seq(END_MULTILINE_COMMENT.as_bytes()) +
            !seq(START_MULTILINE_COMMENT.as_bytes())
        ).repeat(0..) +
        (
            !seq(END_MULTILINE_COMMENT.as_bytes()) +
            call(multiline_comment) + (
                !seq(END_MULTILINE_COMMENT.as_bytes()) +
                !seq(START_MULTILINE_COMMENT.as_bytes())
            ).repeat(0..)
        ).repeat(0..) +
        seq(END_MULTILINE_COMMENT.as_bytes())
    ).discard()
}

/// Parses through ignored source code (if there is any).
pub fn ignored() -> Ignored {
    (whitespace_optional() +
        ((single_line_comment() | multiline_comment()) + whitespace_optional())
            .repeat(0..))
        .discard()
}

/// Parses a literal decimal number
fn decimal_literal() -> WrightParser<Literal> {
    is_a(digit)
        .repeat(1..)
        .convert(String::from_utf8)
        .convert(|s: String| str::parse::<u64>(&s))
        .map(|n: u64| Literal::Integer(n))
}

/// Parses a binary literal.
/// i.e. `0b10001000`
fn binary_literal() -> WrightParser<Literal> {
    (seq(b"0b") + one_of(b"01").repeat(1..))
        .map(|(a,b)| a
            .iter()
            .map(|c| *c)
            .chain(b)
            .collect::<Vec<u8>>())
        .convert(String::from_utf8)
        .convert(|s:String| str::parse::<u64>(&s))
        .map(|n:u64| Literal::Integer(n))
}

/// Parses hexadecimal literal.
/// i.e. `0xCAFEBABE`
fn hex_literal() -> WrightParser<Literal> {
    (seq(b"0x") + is_a(hex_digit).repeat(1..))
        .map(|(a,b)| a
            .iter()
            .map(|c| *c)
            .chain(b)
            .collect::<Vec<u8>>())
        .convert(String::from_utf8)
        .convert(|s:String| str::parse::<u64>(&s))
        .map(|n| Literal::Integer(n))
}

/// Parses integer literal.
pub fn integer_literal() -> WrightParser<Literal> {
    (decimal_literal() | hex_literal() | binary_literal())
        .name("integer literal")
}

/// Parses identifier.
pub fn identifier() -> WrightParser<Identifier> {
    with_properties(
        RESERVED_TOKENS
            .iter()
            .map(|t| t.as_bytes())
            .fold(empty(),
                  |acc, item| (acc + !seq(item)).discard()) +
        ((is_a(alpha) | sym(b'_')) +
        (is_a(alphanum) | sym(b'_')).repeat(0..))
            .map(move |(a, b)| {
                let mut s: String = b
                    .iter()
                    .map(|c| *c as char)
                    .collect::<String>();
                s.insert(0, a as char); s}))
        .map(move |(p, (_, s))| Identifier {properties: p, inner: s})
}

/// Parses `self`.
pub fn self_var() -> WrightParser<Literal> {
    seq(SELF_V.as_bytes()).map(|_| Literal::SelfVar)
}

fn literal_to_expr(p: WrightParser<Literal>) -> WrightParser<Expr> {
    with_properties(p)
        .map(move |(p,l)| Expr{properties: p, variant: ExprVariant::Literal(l)})
}

/// Parses and returns expression.
pub fn expression() -> WrightParser<Expr> {
    unimplemented!()
}