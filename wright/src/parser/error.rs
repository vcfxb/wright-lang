//! Representation and implementation relating to errors that may be encountered in parsing.

use crate::{
    reporting::{Diagnostic, Highlight},
    source_tracking::fragment::Fragment,
};
use std::borrow::Cow;

/// All the different errors that can be produced in the process of parsing.
/// The names of these should be self-describing, but in cases when one of these needs to appear in a diagnostic,
/// use [ParserErrorKind::description].
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserErrorKind {
    UnternminatedStringLiteralEncountered,
    UnterminatedMultilineCommentEncountered,
    ExpectedIdentifier,
    ExpectedPath,
}

/// Table of all the definition strings for v
pub const ERROR_VARIANT_DESCRIPTION_TABLE: &[(ParserErrorKind, &str)] = &[
    (
        ParserErrorKind::UnternminatedStringLiteralEncountered,
        "encountered unterminated string literal while parsing",
    ),
    (
        ParserErrorKind::UnterminatedMultilineCommentEncountered,
        "encountered unterminated multiline comment while parsing",
    ),
    (ParserErrorKind::ExpectedIdentifier, "expected identifier"),
    (ParserErrorKind::ExpectedPath, "expected path or identifier"),
];

impl ParserErrorKind {
    /// Check (at compile time) if this [ParserErrorKind] has a descrition in the [ERROR_VARIANT_DESCRIPTION_TABLE].
    pub const fn has_descrition(self) -> bool {
        let mut i = 0;

        while i < ERROR_VARIANT_DESCRIPTION_TABLE.len() {
            if ERROR_VARIANT_DESCRIPTION_TABLE[i].0 as u64 == self as u64 {
                return true;
            }

            i += 1;
        }

        false
    }

    /// Get the description string of this [ParserErrorKind], if one exists. Calls to this against literals
    /// should be zero-cost since all the lookups are done at compile time. You can use a `const { }` block
    /// to ensure this.
    ///
    /// Calls against variables might be a bit more expensive, since this does an iterative lookup against the
    /// [ERROR_VARIANT_DESCRIPTION_TABLE].
    pub const fn find_description(self) -> Option<&'static str> {
        let mut i = 0;

        while i < ERROR_VARIANT_DESCRIPTION_TABLE.len() {
            if ERROR_VARIANT_DESCRIPTION_TABLE[i].0 as u64 == self as u64 {
                return Some(ERROR_VARIANT_DESCRIPTION_TABLE[i].1);
            }

            i += 1;
        }

        None
    }

    /// Return this [ParserErrorKind] cast to a [u64] preceded by the letters "WPE" standing for "Wright Parser Error".
    pub fn error_code_string(self) -> String {
        format!("WPE{}", self as u64)
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
        let description = self
            .kind
            .find_description()
            .map(ToOwned::to_owned)
            .unwrap_or(format!("parser error ({:?})", self.kind));

        let mut diagnostic = Diagnostic::error()
            .with_code(self.kind.error_code_string())
            .with_message(description)
            .with_highlights([Highlight::primary(self.location, "")]);

        if let Some(help) = self.help {
            diagnostic = diagnostic.with_notes([help]);
        }

        diagnostic
    }
}
