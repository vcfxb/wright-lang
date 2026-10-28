//! Some utility functions/modules shared by various binaries for things like checking unicode support.

use crate::lexer::token::Token;
use std::fmt;
use std::fmt::Display;

pub mod supports_unicode;
#[cfg(feature = "reporting")]
pub mod reporting;

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If the host terminal supports unicode, replace the newline & carriage return characters with pictures,
        // otherwise use ascii.
        let replacements = match supports_unicode::supports_unicode() {
            true => &[("\n", "\u{240A}"), ("\r", "\u{240D}")],
            false => &[("\n", "[nl]"), ("\r", "[cr]")],
        };

        let mut with_replacements = self.fragment.as_str().to_owned();

        for (replace, replace_with) in replacements {
            with_replacements = with_replacements.replace(replace, replace_with);
        }

        write!(f, "\"{with_replacements}\" ({:?})", self.variant)
    }
}
