//! Reporting for errors, warnings, and everything else.
//!
//! The code in this module is heavily inspired by [codespan-reporting] and [ariadne].
//!
//! [codespan-reporting]: https://crates.io/crates/codespan-reporting
//! [ariadne]: https://crates.io/crates/ariadne

use crate::source_tracking::SourceMap;
use crate::source_tracking::filename::FileName;
use crate::source_tracking::immutable_string::ImmutableString;
use crate::source_tracking::{fragment::Fragment, source::SourceId};
use codespan_reporting::diagnostic::Diagnostic as CRDiagnostic;
use codespan_reporting::diagnostic::Label;
use codespan_reporting::files::{Error as CRError, Files};
use codespan_reporting::term::Config;
use std::io::Write;
use std::sync::Mutex;
use termcolor::{ColorChoice, StandardStream, WriteColor};

#[cfg(doc)]
use crate::source_tracking::source::Source;

// Publicly re-export codespan's severity type, since it's pretty much exactly what we want/use.
pub use codespan_reporting::diagnostic::Severity;

/// The style/priority applied to a [Highlight] being rendered.
pub use codespan_reporting::diagnostic::LabelStyle;

/// The global-static color choice for printing to the standard output.
static STDOUT_COLOR_CHOICE: Mutex<ColorChoice> = Mutex::new(ColorChoice::Auto);

/// Set the [ColorChoice] to use when printing [Diagnostic]s to the standard output.
pub fn set_stdout_color(c: ColorChoice) {
    *STDOUT_COLOR_CHOICE.lock().unwrap() = c;
}

/// Get the current [ColorChoice] used when printing [Diagnostic]s to the standard output.
pub fn get_stdout_color() -> ColorChoice {
    *STDOUT_COLOR_CHOICE.lock().unwrap()
}

/// Wright's [Diagnostic] type wraps one from [codespan_reporting] to make it compatible with
/// things like [Fragment] and [SourceId].
#[derive(Debug)]
pub struct Diagnostic(pub CRDiagnostic<SourceId>);

impl Diagnostic {
    /// Construct a new [Diagnostic] with the given [Severity].
    pub fn new(severity: Severity) -> Self {
        Diagnostic(CRDiagnostic::new(severity))
    }

    /// Construct a new [Diagnostic] representing a wright compiler bug.
    #[inline]
    pub fn bug() -> Self {
        Self::new(Severity::Bug)
    }

    /// Construct a new [Diagnostic] representing an error.
    #[inline]
    pub fn error() -> Self {
        Self::new(Severity::Error)
    }

    /// Construct a new [Diagnostic] representing a warning.
    #[inline]
    pub fn warning() -> Self {
        Self::new(Severity::Warning)
    }

    /// Construct a new [Diagnostic] representing a note.
    #[inline]
    pub fn note() -> Self {
        Self::new(Severity::Note)
    }

    /// Construct a new [Diagnostic] representing a help message to the user.
    #[inline]
    pub fn help() -> Self {
        Self::new(Severity::Help)
    }

    /// Builder style function to set an error/warning code for this [Diagnostic].
    pub fn with_code(self, code: impl Into<String>) -> Self {
        Diagnostic(self.0.with_code(code))
    }

    /// Builder style function to set a message for this [Diagnostic].
    pub fn with_message(self, message: impl Into<String>) -> Self {
        Diagnostic(self.0.with_message(message))
    }

    /// Add all the notes from the given [Iterator] to this [Diagnostic].
    pub fn with_notes<I: Into<String>>(mut self, notes: impl IntoIterator<Item = I>) -> Self {
        self.0.notes.extend(notes.into_iter().map(Into::into));
        self
    }

    /// Add all the [Highlight]s from a given [Iterator] to this [Diagnostic].
    pub fn with_highlights(mut self, highlights: impl IntoIterator<Item = Highlight>) -> Self {
        self.0.labels.extend(highlights.into_iter().map(Into::into));
        self
    }

    /// Write this [Diagnostic] to a given [WriteColor]. This will error if any of the [Highlight]s are not in
    /// the referenced [SourceMap], or if any were constructed from invalid [Fragment]s.
    pub fn write(
        &self,
        map: &SourceMap,
        writer: &mut dyn WriteColor,
        config: &Config,
    ) -> Result<(), CRError> {
        codespan_reporting::term::emit(writer, config, map, &self.0)
    }

    /// Print this [Diagnostic] to the standard output. This locks the standard output until the diagnostic is printed.
    /// This uses the global [get_stdout_color] function to determine whether or not to use colors while printing.
    /// This uses the [Config::default] configuration from [codespan_reporting] when printing.
    pub fn print(&self, map: &SourceMap) -> Result<(), codespan_reporting::files::Error> {
        let stream = StandardStream::stdout(get_stdout_color());
        let mut lock = stream.lock();
        self.write(map, &mut lock, &Config::default())?;
        lock.flush()?;
        Ok(())
    }
}

/// A highlighted section of a [Source] that's used in a [Diagnostic].
#[derive(Clone, Debug)]
pub struct Highlight {
    /// The style/importance of this [Highlight]. [Highlight]s with [LabelStyle::Primary] are given priority
    /// when being displayed.
    pub style: LabelStyle,
    /// The [Fragment] containing the relevant section of code.
    pub fragment: Fragment,
    /// The message attached to this [Highlight]. This can be empty, in which case the [Fragment] will
    /// just be underlined when displayed.
    pub message: String,
}

impl Highlight {
    /// Construct a new [Highlight] with [LabelStyle::Primary].
    pub fn primary(fragment: Fragment, message: impl Into<String>) -> Self {
        Highlight {
            style: LabelStyle::Primary,
            fragment,
            message: message.into(),
        }
    }

    /// Construct a new [Highlight] with [LabelStyle::Secondary].
    pub fn secondary(fragment: Fragment, message: impl Into<String>) -> Self {
        Highlight {
            style: LabelStyle::Secondary,
            fragment,
            message: message.into(),
        }
    }

    /// Builder style function to set the [Highlight::message].
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl From<Highlight> for Label<SourceId> {
    fn from(value: Highlight) -> Self {
        Label {
            style: value.style,
            file_id: value.fragment.source.id,
            range: value.fragment.range,
            message: value.message,
        }
    }
}

impl<'f> Files<'f> for SourceMap {
    type FileId = SourceId;

    type Name = FileName;

    type Source = ImmutableString;

    fn name(&'f self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        self.get(id)
            .map(|source| source.name().clone())
            .ok_or(CRError::FileMissing)
    }

    fn source(
        &'f self,
        id: Self::FileId,
    ) -> Result<Self::Source, codespan_reporting::files::Error> {
        self.get(id)
            .map(|source| source.source().clone())
            .ok_or(CRError::FileMissing)
    }

    fn line_index(
        &'f self,
        id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        Ok(self
            .get(id)
            .ok_or(CRError::FileMissing)?
            .line_index(byte_index))
    }

    fn line_range(
        &'f self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, codespan_reporting::files::Error> {
        Ok(self
            .get(id)
            .ok_or(CRError::FileMissing)?
            .get_line(line_index)
            .range)
    }
}
