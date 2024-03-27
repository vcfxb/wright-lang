//! Parsing and AST node model for integer literals in wright source code.

use crate::parser::ast::AstGeneratorContext;
use crate::parser::lexer::token::TokenTy;
use crate::parser::{ast::AstNode, fragment::Fragment};
use num::bigint::ParseBigIntError;
use num::{BigUint, Num};

/// An integer literal in wright source code. Currently these are very simple.
/// The format for integers is currently:
///
/// `("0x" | "0X" | "0o" | "0b" | "0B")? (digit of given radix or underscore)+`
///
/// See the [lexer module] for more details.
///
/// [lexer module]: crate::parser::lexer::integer_literal
#[derive(Debug)]
pub struct IntegerLiteral<'src> {
    /// The associated [Fragment] of source code. This is generally pulled directly from the
    /// matched [TokenTy::IntegerLiteral] token.
    pub fragment: Fragment<'src>,

    /// The value that is represented by this integer literal -- this is represented using a [BigUint]
    /// so that the actual type of the literal may be assertained later on depending on its value. Wright may
    /// or may not support integer literals larger than [`u64`] eventually, so we do this to keep our options
    /// open/flexible.
    pub value: BigUint,
}

/// Errors tha can occur when parsing an integer literal.
#[derive(Clone, Debug)]
pub enum IntegerLiteralParsingError<'src> {
    /// Expected to find an [TokenTy::IntegerLiteral] [Token] and didn't.
    ExpectedIntegerLiteral {
        // The fragment we expected to see an integer literal at.
        at: Fragment<'src>,
    },

    /// Error after passing string to [`num`].
    NumParsingError {
        /// The error from [`num`].
        error: ParseBigIntError,

        /// The fragment we were trying to parse to an integer literal.
        at: Fragment<'src>,
    },
}

impl<'src> AstNode<'src> for IntegerLiteral<'src> {
    type Error = IntegerLiteralParsingError<'src>;

    fn fragment(&self) -> Fragment<'src> {
        self.fragment
    }

    fn try_parse(ctx: &mut AstGeneratorContext<'src>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        // Get the next token from the context if it is an integer literal.
        // Otherwise error.
        // We only care about the fragment from the token, so extract that.
        let fragment: Fragment = ctx
            .next_if_is(TokenTy::IntegerLiteral)
            .ok_or(IntegerLiteralParsingError::ExpectedIntegerLiteral {
                at: ctx.peek_fragment(),
            })?
            .fragment;

        // Get the fragment's internal string so we can slice it up and pass it to the num crate for
        // heavy lifting.
        let literal: &str = fragment.inner;

        // Make a list of prefixes with their radixes to try.
        let prefixes = [("0x", 16), ("0X", 16), ("0o", 8), ("0b", 2), ("0B", 2)];

        for (prefix, radix) in prefixes {
            if let Some(prefix_stripped) = literal.strip_prefix(prefix) {
                // Strip any leading undescores, since `num` eerors on those but we're less strict.
                let fully_stripped: &str = prefix_stripped.trim_start_matches('_');
                // Pass the rest of the parsing to `num`.
                // If this errors, pass it upwards -- it shouldn't because the lexer should radix check
                // for us and we just removed all leading undescores but on the off chance that it does, just
                // report it.
                let value: BigUint = BigUint::from_str_radix(fully_stripped, radix).map_err(
                    |err: ParseBigIntError| IntegerLiteralParsingError::NumParsingError {
                        error: err,
                        at: fragment,
                    },
                )?;

                return Ok(IntegerLiteral { fragment, value });
            }
        }

        // If no prefixes matched, it's a decimal number -- pass it right to `num`.
        // Deal with any errors the same way as above, but this time don't bother stripping undescores
        // since the lexer enforces starting with an ascii digit.
        let value: BigUint = BigUint::from_str_radix(literal, 10).map_err(|err| {
            IntegerLiteralParsingError::NumParsingError {
                error: err,
                at: fragment,
            }
        })?;

        Ok(IntegerLiteral { fragment, value })
    }
}
