//! Representation and implementation relating to errors that may be encountered in parsing.

use crate::{
    reporting::{Diagnostic, Highlight},
    source_tracking::fragment::Fragment,
};
use std::borrow::Cow;

/// All the different errors that can be produced in the process of parsing.
/// The names of these should be self-describing, but in cases when one of these needs to appear in a diagnostic,
/// use [ParserErrorKind::describe].
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserErrorKind {
    EncounteredUnknownToken,
    EncounteredUnterminatedComment,
    EncounteredUnterminatedString,
    ExpectedIdentifier,
    ExpectedPath,
    ExpectedWhitespace,
    ExpectedIntegerLiteral,
}

impl ParserErrorKind {
    /// Get a short description of this kind of error.
    pub const fn describe(self) -> &'static str {
        use ParserErrorKind::*;

        match self {
            EncounteredUnknownToken => "encountered unknown token",
            EncounteredUnterminatedComment => {
                "encountered unterminated multiline comment while parsing"
            }
            EncounteredUnterminatedString => {
                "encountered unterminated string literal while parsing"
            }
            ExpectedIdentifier => "expected identifier",
            ExpectedIntegerLiteral => "expected integer literal",
            ExpectedPath => "expected path or identifier",
            ExpectedWhitespace => "expected whitespace character(s)",
        }
    }
}

/// An error that occurred while parsing.
/// This error structure is pretty simple compared to what can be represented using a diagnostic. That's fine,
/// since most of the more complex errors arise when typechecking, rather than checking syntax.
#[derive(Debug)]
pub struct ParserError {
    /// What type/cause there is for this error.
    pub kind: ParserErrorKind,

    /// Where this error occurred.
    pub location: Fragment,

    /// Optionally, a help string that can be printed with this error.
    pub help: Option<Cow<'static, str>>,
}

impl ParserError {
    /// Turn this parser error into a full blown compiler error.
    pub fn as_diagnostic(self) -> Diagnostic {
        let description = self.kind.describe();

        let mut diagnostic = Diagnostic::error()
            .with_message(description)
            .with_highlights([Highlight::primary(self.location, "")]);

        if let Some(help) = self.help {
            diagnostic = diagnostic.with_notes([help]);
        }

        diagnostic
    }
}
