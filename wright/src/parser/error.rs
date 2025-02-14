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
    ExpectedBooleanLiteral,
    ExpectedImportDeclaration,
    ImportMustEndWithSemicolon,
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
            ExpectedBooleanLiteral => "expected boolean literal",
            ExpectedPath => "expected path or identifier",
            ExpectedWhitespace => "expected whitespace character(s)",
            ExpectedImportDeclaration => "expected import declaration",
            ImportMustEndWithSemicolon => "import declarations must end with a semicolon",
        }
    }

    /// Construct a [ParserError] by adding a location [Fragment] to this error variant.
    pub const fn at(self, f: Fragment) -> ParserError {
        ParserError {
            kind: self,
            location: f,
            help: None,
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
    /// Builder-style method to add a help string to a [ParserError].
    pub fn with_help(mut self, help: impl Into<Cow<'static, str>>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Turn this parser error into a full blown compiler error.
    pub fn as_diagnostic(self) -> Diagnostic {
        let description = self.kind.describe();

        // Put a little clarification if the parser reached end of source and then produced an error.
        let message = if self.location.is_empty_at_end_of_source() {
            Cow::Borrowed("found end of source here")
        } else {
            Cow::Borrowed("")
        };

        let mut diagnostic = Diagnostic::error()
            .with_message(description)
            .with_highlights([Highlight::primary(self.location.clone(), message)]);

        if let Some(help) = self.help {
            diagnostic = diagnostic.with_notes([help]);
        }

        diagnostic
    }
}
