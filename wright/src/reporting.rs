//! Reporting for errors, warnings, and everything else. 
//! 
//! The code in this module is heavily inspired by [codespan-reporting] and [ariadne]. 
//! 
//! [codespan-reporting]: https://crates.io/crates/codespan-reporting
//! [ariadne]: https://crates.io/crates/ariadne

use std::io;
use self::{owned_string::OwnedString, severity::Severity};
use termcolor::{ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};

pub mod severity;
pub mod owned_string;
pub mod label;
pub mod draw;

/// A diagnostic to help the user to understand details of their interactions with the Wright compiler.
#[derive(Debug)]
pub struct Diagnostic {
    /// The severity of this diagnostic, helps determine coloration when shown to the user. 
    pub severity: Severity,

    /// An optional error code, that identifies this type of diagnostic. 
    pub code: Option<OwnedString>,

    /// The main message of the diagnostic. This should be short enough to display on one terminal line in most cases.
    pub message: OwnedString,
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
            message: message.into()
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
        // Create a color spec to use as we write to the writer.
        let mut spec: ColorSpec = ColorSpec::new();
        // Set the color spec for the severity and code. 
        spec.set_intense(true).set_fg(Some(self.severity.color()));
        w.set_color(&spec)?;



        // Write the severity and code.
        write!(w, "{}", self.severity)?;

        if let Some(code) = self.code.as_ref() {
            write!(w, " [{code}]")?;
        }

        // Reset the color to write the message. 
        spec.set_fg(None);
        w.set_color(&spec)?;

        // Write the message and a new line.
        writeln!(w, ": {}", self.message)?;

        Ok(())
    }
}

#[cfg(test)] 
mod tests {
    use termcolor::NoColor;

    use super::Diagnostic;

    #[test]
    fn test_basic_error() {
        // Use a byte vec as a test buffer to write to without color. 
        let mut buffer: NoColor<Vec<u8>> = NoColor::new(Vec::new());
        // Create a test diagnostic.
        let d: Diagnostic = Diagnostic::error("test error");
        // Write to buffer.
        d.write(&mut buffer, false).unwrap();
        // Convert the buffer to a string to compare.
        let output: &str = std::str::from_utf8(buffer.get_ref().as_slice()).unwrap();

        assert_eq!(output, "error: test error\n");
    }
}
