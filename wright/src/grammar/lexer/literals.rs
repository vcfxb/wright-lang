use crate::grammar::lexer::*;
use codespan_reporting::diagnostic::*;

#[derive(Debug, Copy, Clone)]
/// Parser for boolean literals like 'true' and 'false'.
pub struct Bool;

impl Parser for Bool {
    const RULE: &'static str = "BOOLEAN LITERAL";

    fn try_parse(lexer: &mut Lexer) -> Option<Token> {

    }

    fn do_parse(lexer: &mut Lexer) -> Result<Token, Diagnostic> {
        Self::try_parse(lexer).ok_or_else(|| unimplemented!())
    }
}