//! Error module.
//! Contains traits and Constants for error printing.

use std::fmt;
use std::fmt::Debug;
use position::span::Span;
extern crate ansi_term;
use self::ansi_term::Color::*;
use self::ansi_term::Color;

/// Color code for errors used throughout entire error reporting system.
///
#[deprecated(since = "0.5.0")]
pub const ERROR_COLORS: [Color; 4] = [
    Red, Cyan, Green, Cyan,
];


/// Enum for error levels.
#[derive(Debug, Copy, Clone)]
pub enum ErrorLevel {Warning, Error}

impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &ErrorLevel::Warning => Yellow,
            &ErrorLevel::Error   => Red,
        }.paint(format!("{:?}", self)))
    }
}

/// Color for names which are printed by the compiler.
pub const NAME_COLOR: Color = Green;

/// Color for modules when printed by the compiler.
pub const MODULE_COLOR: Color = Cyan;

/// Color for dividing bars used when displaying errors.
pub const BAR_COLOR: Color = Cyan;

/// Color for info line displayed.
pub const INFO_COLOR: Color = Blue;

/// Color for underlining arrows for error spans.
pub const ARROW_COLOR: Color = Red;

/// Color for source code that is displayed in error reporting.
pub const CONTENT_COLOR: Color = Green;

/// Trait for Errors. Any error used throughout the wright compiler/interpreter must implement
/// this trait for consistency.
pub trait Error<'source> : Debug + Sized {
    /// Return the name of the error.
    fn get_name(&self)   -> &str;

    /// Return the module or location the error came from.
    fn get_module(&self) -> &str;

    /// Returns the error level.
    fn get_level(&self) -> ErrorLevel;

    /// Returns a vector of the content spans of the offending content.
    ///
    /// If there are no spans (the vector is empty) then line numbers will not
    /// be used in error reporting. (The entire source will be printed.)
    ///
    /// Spans must be single line only. (The error is invalid otherwise.)
    fn get_spans(&self)  -> &Vec<Span>;

    /// Return information about the error.
    ///
    /// The first entry is displayed as the error information at the top of the error
    /// and the following ones are displayed next to the the underlining string for each span.
    ///
    /// Any additional information (past the appropriate number for the given set of spans)
    /// will not show up in the error report.
    ///
    /// If the vector is empty or too short then each informative will default to an empty string
    /// except for the first one, which defaults to "An error occurred."
    fn get_info(&self)   -> &Vec<&str>;

    /// Return a reference to the source code of the module (file) containing the error.
    /// Or the source of the error if it was not in code.
    ///
    /// If line numbers are not being used, the full source returned by this
    /// function will be printed.
    ///
    /// The error will be invalid if the source is not long enough to contain the errors found.
    fn get_lines(&self)  -> &'source Vec<&'source str>;

    /// Check to see if the error is valid.
    /// (Checks to make sure there are the appropriate numbers of spans,
    /// informatics, and source lines).
    ///
    /// # Errors
    /// - If any of the spans from get_spans are multiple lines.
    /// - If any of the spans are outside of the of the source module/file.
    fn validate(&self) -> Result<(), ()> {
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
    /// Panics if validate() returns an Err().
    fn get_displayable(&self) ->  ErrorToDisplay<'source> {
        self.validate().unwrap();
        let result = ErrorToDisplay {
            level:         self.get_level(),
            name:          self.get_name().to_string(),
            module:        self.get_module().to_string(),
            line_info:     ErrorToDisplay::get_line_info(self.get_spans()),
            error_info:    self.get_info().iter().map(|s| s.to_string()).collect(),
            source_lines:  self.get_lines(),
            spans:         self.get_spans().clone(),
        };
        return result;
    }

    /// Displays a given error via the get_displayable method.
    ///
    /// # Panics
    /// Panics if validate() returns and Err().
    fn display(&self) {
        self.get_displayable().display();
    }
}

#[derive(Debug, Clone)]
/// ErrorToDisplay is an intermediate type used to go from a raw error
/// into a format that can be printed easily.
pub struct ErrorToDisplay<'source> {
    level:          ErrorLevel,
    name:           String,
    module:         String,
    line_info:      String,
    error_info:     Vec<String>,
    source_lines:   &'source Vec<&'source str>,
    spans:          Vec<Span>,
}

impl<'source> ErrorToDisplay<'source> {
    /// Prints this error into the terminal.
    pub fn display(&self) {
        println!("{}", self);
    }
    // not pub because it doesn't need to be and it is pretty specific.
    /// Takes vec of spans and uses it to set line info.
    fn get_line_info(span_vec: &Vec<Span>) -> String {
        if span_vec.is_empty() {"".to_string()}
        else {
            format!("on line {}.", span_vec.last().unwrap().get_start().get_line())
        }
    }
}

impl<'source> fmt::Display for ErrorToDisplay<'source> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = 5;
        let five = " ".repeat(width);
        let bar = BAR_COLOR.paint("|");
        let info = self.clone().error_info;
        let mut info_iter = info.iter();
        let no_span = self.spans.is_empty();
        let mut span_iter = self.spans.iter();
        writeln!(f, "{level}: {n} in {m} {l_info}\n{f}{info}\n{f} {b}",
            level = format!("{}", self.level),
            n = NAME_COLOR.paint(self.name.clone()),
            m = MODULE_COLOR.paint(self.module.clone()),
            l_info = self.line_info,
            f = five,
            b = bar,
            info = INFO_COLOR.paint(info_iter.next()
                .unwrap_or(&"An Error Occurred.".to_string())
                .as_str()),
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
                p_line = CONTENT_COLOR.paint(
                    self.source_lines.get(span.get_start().get_line()-2).unwrap().trim_right()),
                c_line = CONTENT_COLOR.paint(current_line),
                w = width,
                b = bar,
                f = five,
                a_line = ARROW_COLOR.paint(arrow_line),
                info = INFO_COLOR.paint(info_iter.next()
                    .unwrap_or(&"".to_string())
                    .as_str()),
            )?;
        }
        if no_span {
            for line in self.source_lines {
                writeln!(f, "{f} {b} {l}",
                    f= five,
                    b = bar,
                    l = CONTENT_COLOR.paint(*line),
                )?;
            }
        }
        writeln!(f, "{} {}", five, bar)
    }
}