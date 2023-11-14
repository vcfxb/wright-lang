//! Erase is a useful parser combinator that erases the concrete type of a parser function and turns it into a [BoxedParserFn].

use super::BoxedParserFn;
use crate::parser::state::ParserState;

/// Erase the concrete type of a parser function and put it in a [`BoxedParserFn`].
pub fn erase<'src, PF, O>(parser_function: PF) -> BoxedParserFn<'src, O>
where
    PF: (Fn(&mut ParserState<'src>) -> O) + 'src,
{
    Box::new(parser_function)
}
