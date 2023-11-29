//! Utilities for dealing with escaped characters in string and char literals. 

use std::{borrow::Cow, iter::Peekable, str::CharIndices};

pub fn unescape(source_str_lit_body: &str) -> Cow<'_, str> {
    unimplemented!()
}

#[derive(Debug)]
struct StringLiteralPartsIterator<'str_lit> {
    /// The body of the string literal being unescaped. 
    str_lit_body: &'str_lit str,
    
    /// An iterator over the 
    iter: Peekable<CharIndices<'str_lit>>,
}

enum StringLiteralPart<'str_lit> {
    /// A sequence of unescaped characters.
    UnescapedCharacters(&'str_lit str),

    UnicodeEscape {
        /// The part of the string literal that contains this escape sequence. 
        matching_source: &'str_lit str,
        /// The result of attempting to parse the escaped value into a unicode codepoint. 
        parsed: Option<char>,
    },
}

enum UnicodeEscapeError {
    /// There were too many digits in the escape sequence. 
    TooManyDigits,
    /// Empty escape sequence,
    Empty,
    /// The escaped digits do not represent a valid unicode codepoint. 
    InvalidCodepoint,
}

impl<'str_lit> StringLiteralPartsIterator<'str_lit> {
    
}

impl<'str_lit> Iterator for StringLiteralPartsIterator<'str_lit> {
    type Item = StringLiteralPart<'str_lit>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
        
    }
}
