//! # CodeMap reporting utilities.
//!
//! The top level structure from this module is the Diagnostic struct.
//! Diagnostics are used to display warnings, errors, and general information
//! about the source code being operated on.
//!
//! This submodule was significantly influenced by the
//! [codespan-reporting](https://crates.io/crates/codespan-reporting) crate.
//!
//!
// todo: Examples of Diagnostic print out.


use crate::codemap;
use codemap::CodeMap;
use codemap::charspan::*;

pub use termcolor;
pub use termcolor::Color;
pub use termcolor::WriteColor;
use termcolor::ColorSpec;
use termcolor::BufferedStandardStream;
use termcolor::ColorChoice;

use std::io::Result as IOResult;
use std::sync::Arc;
use crate::codemap::sourcemap::Source;

/// The different levels of severity of diagnostic messages.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Severity {
    /// A help message
    Help,
    /// A note
    Note,
    /// A non-fatal warning
    Warning,
    /// A fatal user error
    Error,
    /// A fatal compiler error
    CompilerBug,
}

impl Severity {
    /// Get termcolor Color used when rendering a message.
    pub fn color(self) -> Color {
        match self {
            Severity::Help => Color::Green,
            Severity::Note => Color::Cyan,
            Severity::Warning => Color::Yellow,
            Severity::Error => Color::Red,
            Severity::CompilerBug => Color::Magenta,
        }
    }
    /// Get a string to explain severity.
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::CompilerBug => "Internal Compiler Error",
            Severity::Error => "Error",
            Severity::Warning => "Warning",
            Severity::Help => "Help",
            Severity::Note => "Note",
        }
    }
}

/// Style of the diagnostic label.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LabelStyle {
    /// Primary diagnostic
    Primary,
    /// Secondary or supporting diagnostic.
    Secondary
}

/// Diagnostic label, used for showing an offending piece of code and an
/// optional message.
#[derive(Debug, Clone)]
pub struct Label {
    /// The character span associated with this label.
    /// Usually an offending token or expression.
    pub span: CharSpan,
    /// The optional message to be displayed with this label.
    pub message: Option<String>,
    /// The style of the label.
    pub style: LabelStyle,
}

impl Label {
    /// Construct a new label from a span and a style.
    pub fn new(span: CharSpan, style: LabelStyle) -> Self { Label {span, message: None, style} }
    /// Construct new primary level label from a span.
    pub fn new_primary(span: CharSpan) -> Self { Label::new(span, LabelStyle::Primary) }
    /// Construct a new secondary level label from a span.
    pub fn new_secondary(span: CharSpan) -> Self { Label::new(span, LabelStyle::Secondary) }
    /// Add a message to a label.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

/// A diagnostic about the source code being operated on.
///
/// The `'map` lifetime is the lifetime of the associated CodeMap.
#[derive(Clone)]
pub struct Diagnostic<'map> {
    /// The severity of this diagnostic.
    pub severity: Severity,
    /// A reference to the associated CodeMap.
    pub codemap: &'map CodeMap,
    /// The message to be displayed with this diagnostic.
    pub message: String,
    /// The labeled sections of code associated with this diagnostic.
    pub labels: Vec<Label>,
    /// Error code, i.e. `E0045`.
    /// This will get emitted in brackets, so `E0045` is emitted as `[E0045]`
    pub code: Option<String>,
}

impl<'a> Diagnostic<'a> {
    /// Construct a new diagnostic.
    pub fn new(severity: Severity, message: String, map: &'a CodeMap) -> Self {
        Diagnostic {
            severity,
            codemap: map,
            message,
            labels: Vec::new(),
            code: None
        }
    }
    /// Construct a new diagnostic for a compiler bug
    pub fn new_compiler_bug(message: String, map: &'a CodeMap) -> Self {
        Diagnostic::new(Severity::CompilerBug, message, map)
    }
    /// Construct a new diagnostic for a user  error
    pub fn new_error(message: String, map: &'a CodeMap) -> Self {
        Diagnostic::new(Severity::Error, message, map)
    }
    /// Construct a new diagnostic for a warning
    pub fn new_warning(message: String, map: &'a CodeMap) -> Self {
        Diagnostic::new(Severity::Warning, message, map)
    }
    /// Construct a new diagnostic to note something
    pub fn new_note(message: String, map: &'a CodeMap) -> Self {
        Diagnostic::new(Severity::Note, message, map)
    }
    /// Construct a new diagnostic to give helpful information to the user.
    pub fn new_help(message: String, map: &'a CodeMap) -> Self {
        Diagnostic::new(Severity::Help, message, map)
    }

    /// Adds the given label to the list of labels to be emitted.
    pub fn add_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    /// Set this diagnostic's error code.
    pub fn set_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Emit / Write this Diagnostic to a writer.
    ///
    /// We use termcolor's writer to manage the coloration of the displayed
    /// diagnostic.
    pub fn emit(&self, mut writer: impl WriteColor) -> IOResult<()> {
        let color_supported = writer.supports_color();
        let line_color = ColorSpec::new()
            .set_fg(Some(if cfg!(windows) {Color::Cyan} else {Color::Blue}))
            .clone();
        let diagnostic_color = ColorSpec::new()
            .set_fg(Some(self.severity.color()))
            .clone();
        let highlight_color = ColorSpec::new()
            .set_bold(true)
            .set_intense(true)
            .clone();
        writer.set_color(highlight_color.clone()
                .set_fg(diagnostic_color
                    .fg()
                    .map_or(None, |c| Some(*c))))?;
        write!(writer, "{} ", self.severity.as_str())?;
        if let Some(ref code) = self.code {
            write!(writer, "[{}]", code)?;
        }
        writer.set_color(&highlight_color)?;
        writeln!(writer, ": {}", self.message)?;
        writer.reset()?;
        // labels
        for label in &self.labels {
            if let Some(weak_source) = self.codemap.get_source(label.span.start) {
                let source: Arc<Source> = weak_source
                    .upgrade()
                    .expect("Failed to unwrap weak source!");
                let (start_line, col) = source.location(label.span.start).expect("Label not in source.");
                let (end_line, _) = source.location(label.span.end).expect("Label not in source.");
                writeln!(
                    writer,
                    "- {}:{}:{}",
                    source.name, start_line, col
                )?;
                let start_line_span = source.line_span(start_line).unwrap();
                let end_line_span = source.line_span(end_line).unwrap();
                unimplemented!()
            } else {
                if let Some(ref msg) = label.message {
                    writeln!(writer, "- {}", msg)?;
                }
            }
        }
        Ok(())
    }

    /// Writes this diagnostic to the standard output.
    pub fn display(&self) {
        self.emit(BufferedStandardStream::stdout(ColorChoice::Always))
            .expect("Could not write to Standard Output.")
    }
}