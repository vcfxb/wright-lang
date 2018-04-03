//! Error module.
//! Contains traits and constants for error printing.

use std::fmt;
use std::fmt::Debug;
use position::span::Span;
extern crate colored;
use self::colored::{ColoredString, Colorize};

/// Enum for error levels.
#[derive(Debug, Copy, Clone)]
pub enum ErrorLevel {
    /// Error level for warnings only.  Does not trigger process failure.
    Warning, 
    /// Error level for errors that prevent the process from completing successfully.
    Error,
}

impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str = &format!("{:?}", self);
        let c: ColoredString;
        match *self {
            ErrorLevel::Warning => {c = s.yellow()},
            ErrorLevel::Error   => {c = s.red()},
        }
        write!(f, "{}", c)
    }
}

/// Trait for Errors. Any error used throughout the wright compiler/interpreter must implement
/// this trait for consistency.
///
/// #### Lifetime Clarification:
/// 'source lifetime represents the time during which the source code being interpreted is alive.
/// 'error  lifetime represents the time during which the error is alive.
/// 'source is required to be longer than 'error
pub trait Error<'source: 'error, 'error> : Debug + Sized {
    /// Return the name of the error.
    fn get_name(&'error self)   -> &'error str;

    /// Return the module or location the error came from.
    fn get_module(&'error self) -> &'source str;

    /// Returns the error level.
    fn get_level(&self) -> ErrorLevel;

    /// Returns a vector of the content spans of the offending content.
    ///
    /// If there are no spans (the vector is empty) then line numbers will not
    /// be used in error reporting.
    ///
    /// Spans must be single line only. (The error is invalid otherwise.)
    fn get_spans(&'error self) -> Vec<Span>;

    /// Return information about the error.
    ///
    /// The first item is displayed as the error information at the top of the error
    /// and the following ones are displayed next to the the underlining string for each span.
    ///
    /// Any additional information (past the appropriate number for the given set of spans)
    /// will not show up in the error report.
    ///
    /// If the vector is empty or too short then each informative will default to an empty string
    /// except for the first one, which defaults to "An error occurred."
    fn get_info(&'error self) -> Vec<&'error str>;

    /// Return a reference to the source code of the module (file) containing the error.
    /// Or the source of the error if it was not in code.
    ///
    /// If line numbers are not being used, the full source returned by this
    /// function will be printed.
    ///
    /// The error will be invalid if the source is not long enough to contain the errors found.
    fn get_lines(&self) -> &'source [&'source str];

    /// Check to see if the error is valid.
    /// (Checks to make sure there are the appropriate numbers of spans,
    /// informatics, and source lines).
    ///
    /// # Errors
    /// - If any of the spans from get_spans are multiple lines.
    /// - If any of the spans are outside of the of the source module/file.
    fn validate(&'error self) -> Result<(), ()> {
        if !self.get_spans()
            .iter()
            .all(|span| !span.is_multiple_lines()) {return Err(());}
        if self.get_spans().len() > 0 && self.get_spans().last().unwrap()
            .get_end().get_line() > self.get_lines().len() {return Err(());}
        Ok(())
    }

    /// Turn error into one which can be displayed.
    /// (ErrorToDisplay implements fmt::Display)
    ///
    /// # Panics
    /// Panics if [`validate()`] returns an Err(()).
    ///
    /// [`validate()`]: ./trait.Error.html#method.validate
    fn get_displayable(&'error self) ->  ErrorToDisplay<'source, 'error> {
        self.validate().unwrap();
        let result = ErrorToDisplay {
            level:         self.get_level(),
            name:          self.get_name(),
            module:        self.get_module(),
            line_info:     ErrorToDisplay::get_line_info(self.get_spans()),
            error_info:    self.get_info().iter().map(|s| s.to_string()).collect(),
            source_lines:  self.get_lines(),
            spans:         self.get_spans(),
        };
        return result;
    }

    /// Displays a given error via the get_displayable method.
    ///
    /// # Panics
    /// Panics if [`validate()`] returns and Err().
    ///
    /// [`validate()`]: ./trait.Error.html#method.validate
    fn display(&'error self) {
        self.get_displayable().display();
    }
}

#[derive(Debug, Clone)]
/// ErrorToDisplay is an intermediate type used to go from a raw error
/// into a format that can be printed easily.
pub struct ErrorToDisplay<'src, 'err> {
    level:          ErrorLevel,
    name:           &'err str,
    module:         &'src str,
    line_info:      String,
    error_info:     Vec<String>,
    source_lines:   &'src [&'src str],
    spans:          Vec<Span>,
}

impl<'src, 'err> ErrorToDisplay<'src, 'err> {
    /// Prints this error into the terminal.
    pub fn display(&self) {
        println!("{}", self);
    }
    // not pub because it doesn't need to be and it is pretty specific.
    /// Takes vec of spans and uses it to set line info.
    fn get_line_info(span_vec: Vec<Span>) -> String {
        if span_vec.is_empty() { "".to_string() }
        else {
            format!("on line {}.", span_vec
                .last()
                .unwrap()
                .get_start()
                .get_line())
        }
    }
}

impl<'source, 'spans> fmt::Display for ErrorToDisplay<'source, 'spans> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = 5;
        let five = " ".repeat(width);
        let bar = "|".cyan();
        let info = self.clone().error_info;
        let mut info_iter = info.iter();
        let no_span = self.spans.is_empty();
        let mut span_iter = self.spans.iter();
        writeln!(f, "{level}: {n} in {m} {l_info}\n{f}{info}\n{f} {b}",
            level = format!("{}", self.level),
            n = self.name.clone().green(),
            m = self.module.clone().cyan(),
            l_info = self.line_info,
            f = five,
            b = bar,
            info = info_iter.next()
                .unwrap_or(&"An Error Occurred.".to_string())
                .as_str()
                .blue(),
        )?;
        while let Some(span) = span_iter.next() {
            let current_line: &str = self.source_lines
                .get(span.get_start()
                    .get_line()-1)
                .unwrap().trim_right();
            let mut arrow_line = String::new();
            current_line.chars().take(span.get_start().get_col()).for_each(|c| {
                match c {
                    '\t' => arrow_line.push('\t'),
                    _    => arrow_line.push(' '),
                }
            });
            arrow_line.push_str("^".repeat(span.get_end().get_col()-span.get_start().get_col())
                .as_str());
            writeln!(f, "{prev:>w$} {b} {p_line}\n{cur:>w$} {b} {c_line}\n{f} {b} {a_line} {info}",
                prev = span.get_start().get_line()-1,
                cur = span.get_start().get_line(),
                p_line = self.source_lines
                    .get(span.get_start().get_line()-2)
                    .unwrap()
                    .trim_right()
                    .green(),
                c_line = current_line.green(),
                w = width,
                b = bar,
                f = five,
                a_line = arrow_line.red(),
                info = info_iter.next()
                    .unwrap_or(&"".to_string())
                    .as_str()
                    .blue(),
            )?;
        }
        if no_span {
            for line in self.source_lines {
                writeln!(f, "{f} {b} {l}",
                    f= five,
                    b = bar,
                    l = line.green(),
                )?;
            }
        }
        writeln!(f, "{} {}", five, bar)
    }
}