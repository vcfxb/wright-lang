//! Reporting for errors, warnings, and everything else.
//!
//! The code in this module is heavily inspired by [codespan-reporting] and [ariadne].
//!
//! [codespan-reporting]: https://crates.io/crates/codespan-reporting
//! [ariadne]: https://crates.io/crates/ariadne

use self::{owned_string::OwnedString, severity::Severity};
use crate::source_tracking::fragment::Fragment;
use std::io;
use termcolor::{ColorChoice, StandardStream, StandardStreamLock, WriteColor};

mod draw;
pub mod owned_string;
pub mod severity;

/// A diagnostic to help the user to understand details of their interactions with the Wright compiler.
#[derive(Debug)]
pub struct Diagnostic {
    /// The severity of this diagnostic, helps determine coloration when shown to the user.
    pub severity: Severity,

    /// An optional error code, that identifies this type of diagnostic.
    pub code: Option<OwnedString>,

    /// The main message of the diagnostic. This should be short enough to display on one terminal line in most cases.
    pub message: OwnedString,

    /// The primary [Highlight] of this diagnostic, which contains the section of source code where the
    /// error/warning lies.
    pub primary_highlight: Option<Highlight>,

    /// Any secondary [Highlight]s that help the reader understand this diagnostic.
    pub secondary_highlights: Vec<Highlight>,

    /// Optionally a note giving extra context or re-stating this diagnostic.
    pub note: Option<OwnedString>,
}

/// Some highlighted source code that can be printed with a [Diagnostic], usually with its own message(s).
#[derive(Debug)]
pub struct Highlight {
    /// A valid [Fragment] representing the source where the error occurred.
    /// The surrounding source will be printed to the best of the ability of this
    /// implementation, which may be modified or updated to better draw [Diagnostic]s.
    pub fragment: Fragment,

    /// Optionally a message to display with the highlighted region.
    pub message: Option<OwnedString>,
}

impl Diagnostic {
    /// Construct a new [Diagnostic].
    /// Use the arguments to fill their corresponding fields,
    /// then fill the rest with the following defaults:
    /// - [Diagnostic::code] defaults to [None].
    pub fn new<M: Into<OwnedString>>(severity: Severity, message: M) -> Self {
        Diagnostic {
            severity,
            code: None,
            message: message.into(),
            primary_highlight: None,
            secondary_highlights: Vec::new(),
            note: None,
        }
    }

    /// Construct a new [Diagnostic] using [Severity::Bug].
    /// See [Diagnostic::new].
    pub fn bug<M: Into<OwnedString>>(message: M) -> Self {
        Diagnostic::new(Severity::Bug, message)
    }

    /// Construct a new [Diagnostic] using [Severity::Error].
    /// See [Diagnostic::new].
    pub fn error<M: Into<OwnedString>>(message: M) -> Self {
        Diagnostic::new(Severity::Error, message)
    }

    /// Construct a new [Diagnostic] using [Severity::Warning].
    /// See [Diagnostic::new].
    pub fn warning<M: Into<OwnedString>>(message: M) -> Self {
        Diagnostic::new(Severity::Warning, message)
    }

    /// Construct a new [Diagnostic] using [Severity::Info].
    /// See [Diagnostic::new].
    pub fn info<M: Into<OwnedString>>(message: M) -> Self {
        Diagnostic::new(Severity::Info, message)
    }

    /// Add a [Diagnostic::code] to this [Diagnostic].
    pub fn with_code(mut self, c: impl Into<OwnedString>) -> Self {
        self.code = Some(c.into());
        self
    }

    /// Add a [Diagnostic::note] to this [Diagnostic].
    pub fn with_note(mut self, n: impl Into<OwnedString>) -> Self {
        self.note = Some(n.into());
        self
    }

    /// Add a [Diagnostic::primary_highlight] to this [Diagnostic].
    pub fn with_primary_highlight(mut self, h: Highlight) -> Self {
        self.primary_highlight = Some(h);
        self
    }

    /// Add a secondary [Highlight] to this [Diagnostic]'s list of [Diagnostic::secondary_highlights].
    pub fn add_secondary_highlight(mut self, h: Highlight) -> Self {
        self.secondary_highlights.push(h);
        self
    }

    /// Print this diagnostic to the standard output.
    ///
    /// Uses [supports_unicode] to determine whether to print unicode characters.
    pub fn print(&self, color_choice: ColorChoice) -> io::Result<()> {
        // Check if the standard output supports unicode.
        let write_unicode: bool = supports_unicode::on(supports_unicode::Stream::Stdout);
        // Get the standard output stream.
        let stdout: StandardStream = StandardStream::stdout(color_choice);
        // Lock it to make sure we can write without interruption.
        let mut stdout_lock: StandardStreamLock = stdout.lock();
        // Write to the locked stream.
        self.write(&mut stdout_lock, write_unicode)
    }

    /// Print this diagnostic to the standard error.
    ///
    /// Uses [supports_unicode] to determine whether to print unicode characters.
    pub fn eprint(&self, color_choice: ColorChoice) -> io::Result<()> {
        // Check if the standard error supports unicode.
        let write_unicode: bool = supports_unicode::on(supports_unicode::Stream::Stderr);
        // Get the standard error stream.
        let stderr: StandardStream = StandardStream::stderr(color_choice);
        // Lock it to make sure we can write without interruption.
        let mut stderr_lock: StandardStreamLock = stderr.lock();
        // Write to the locked stream.
        self.write(&mut stderr_lock, write_unicode)
    }

    /// Write this [Diagnostic] to the given writer.
    ///
    /// It is suggested to use [supports_unicode] to determine a good value for `write_unicode` when writing to
    /// standard streams. That is what this crate does in functions like [Diagnostic::print].
    pub fn write<W: WriteColor>(&self, w: &mut W, write_unicode: bool) -> io::Result<()> {
        draw::draw(self, w, write_unicode)
    }
}

#[cfg(test)]
mod tests {
    // Drawing tests currently covered in [super::draw].
}
