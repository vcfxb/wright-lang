//! First pass lexer that gets run on the source code and returns a series of tokens with their associated [Fragment]s.
//! 
//! Note that this will strip out comments and whitespace, returning only fragments that match one of the paterns 
//! defined for tokens. 

use super::fragment::Fragment;

/// The 
#[derive(Debug)]
pub struct Lexer<'src> {
    /// The remaining source code that has not been processed and returned as a token from the iterator yet. 
    pub remaining: Fragment<'src>,
}

