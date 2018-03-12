//! Module containing struct to represent Interpreter Errors.

use errors::{Error, ErrorLevel};
use position::span::Span;

/// Struct for an Interpreter Error.
/// 
#[derive(Debug, Copy, Clone)]
pub struct InterpreterError<'source: 'error, 'error> {
    /// Name of the file involved in the Error.
    pub file_name: &'source str,
    /// TODO: docs
    pub reasons: &'error [&'error str],
}

impl<'src: 'err, 'err> Error<'src, 'err> for InterpreterError<'src, 'err> {
    fn get_name(&self) -> &'err str { "Input Error" }
    fn get_module(&self) -> &'src str { self.file_name }
    fn get_level(&self) -> ErrorLevel { ErrorLevel::Error }
    fn get_spans(&self) -> &'err [Span] { &[] }
    fn get_info(&'err self) -> &'err [&'err str] { self.reasons }
    fn get_lines(&self) -> &'src [&'src str] { &[] }
}