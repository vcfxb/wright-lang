use crate::grammar::lexer::*;
use codespan_reporting::diagnostic::*;

#[derive(Debug, Copy, Clone)]
/// Parser for boolean literals like 'true' and 'false'.
pub struct BOOL;