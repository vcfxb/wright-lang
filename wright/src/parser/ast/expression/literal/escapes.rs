//! Utilities for dealing with escaped characters in string and char literals. 

use std::{iter::{Peekable, FusedIterator, self}, str::CharIndices, ops::Range};
use super::string::StringLiteralValue;

/// Handle all the escapes in a string literal and produce a [`StringLiteralValue`] or error. 
pub fn unescape(source_str_lit_body: &str) -> Result<StringLiteralValue, Vec<StringLiteralError>> {
    // Make an iterator over the parts of the string literal. 
    let mut parts_iter = StringLiteralPartsIterator::new(source_str_lit_body);

    // Create a vector to hold any errors we generate. 
    let mut errors: Vec<StringLiteralError> = Vec::new();

    // Match on the first two parts detected -- if there is more than one then we need to allocate a string and 
    // fill it with the converted parts. 
    match (parts_iter.next(), parts_iter.next()) {
        // No string -- allocate nothing, return Ok. 
        (None, _) => Ok(StringLiteralValue::WithoutEscapes(source_str_lit_body)),

        // String is purely unescaped characters. Return as-is. 
        (Some(StringLiteralPart::UnescapedCharacters(_)), None) => Ok(StringLiteralValue::WithoutEscapes(source_str_lit_body)),

        // String contains at least one escape or error. Allocate a new string buffer and read values into it. 
        (Some(head), tail) => {
            // Allocate with the same capacity as the source string as this cannot get any bigger over the 
            // process of un-escaping. 
            let mut result = String::with_capacity(source_str_lit_body.len());

            // Recreate the original parts iter so we can iterate through in a chain. 
            let recreated_iter = iter::once(head)
                .chain(tail)
                .chain(parts_iter);

            for string_literal_part in recreated_iter {
                match string_literal_part {
                    // For unescaped characters, push the slice containing them to the result string. 
                    StringLiteralPart::UnescapedCharacters(range) => result.push_str(&source_str_lit_body[range]),

                    // Escaped quotes: 
                    StringLiteralPart::DoubleQuoteEscape => result.push('\"'),
                    StringLiteralPart::SingleQuoteEscape => result.push('\''),

                    // ASCII Escape errors
                    StringLiteralPart::AsciiEscape { source_range, parsed: Err(ascii_err) } => 
                        errors.push(StringLiteralError { byte_range: source_range, ty: StringLiteralErrorTy::AsciiEscapeError(ascii_err) }),

                    // ASCII Escape successes 
                    StringLiteralPart::AsciiEscape { parsed: Ok(ascii_char), .. } => result.push(ascii_char),

                    // Unicode Escape errors
                    StringLiteralPart::UnicodeEscape { source_range, parsed: Err(unicode_err) } => 
                        errors.push(StringLiteralError { byte_range: source_range, ty: StringLiteralErrorTy::UnicodeEscapeError(unicode_err) }),

                    // Unicode Escape successes
                    StringLiteralPart::UnicodeEscape { parsed: Ok(unicode_char), .. } => result.push(unicode_char),

                    // Unrecognized escape sequences
                    StringLiteralPart::UnrecognizedEscapeSequence(range) => 
                        errors.push(StringLiteralError { byte_range: range, ty: StringLiteralErrorTy::UnrecognizedEscapeSequence }),
                }
            }

            // Return the result if there are no errors. 
            if errors.is_empty() {
                Ok(StringLiteralValue::WithEscapes(result.into()))
            } else {
                Err(errors)
            }
        }
    }
}

/// An error in a string literal. 
#[derive(Debug)]
pub struct StringLiteralError {
    /// The byte range in the string literal body where the error ocurred. 
    pub byte_range: Range<usize>,

    /// The type of error. 
    pub ty: StringLiteralErrorTy,
}

/// An error in a string literal. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringLiteralErrorTy {
    /// An error with an ASCII escape.
    AsciiEscapeError(AsciiEscapeErrorTy),

    /// An error with a Unicode escape. 
    UnicodeEscapeError(UnicodeEscapeErrorTy),

    /// An unrecognized escape sequence. 
    UnrecognizedEscapeSequence
}

#[derive(Debug)]
struct StringLiteralPartsIterator<'str_lit> {
    /// The body of the string literal being unescaped. 
    str_lit_body: &'str_lit str,
    
    /// An [Peekable] [Iterator] over the characters in the string literal body and their indices. 
    iter: Peekable<CharIndices<'str_lit>>,
}

/// A part of a string literal being processed. 
#[derive(Debug)]
enum StringLiteralPart {
    /// A sequence of unescaped characters.
    UnescapedCharacters(Range<usize>),

    /// A unicode escape like in rust <https://doc.rust-lang.org/reference/tokens.html#unicode-escapes>.
    UnicodeEscape {
        /// The part of the string literal that contains this escape sequence. 
        source_range: Range<usize>,
        /// The result of attempting to parse the escaped value into a unicode codepoint. 
        parsed: Result<char, UnicodeEscapeErrorTy>,
    },

    /// An ASCII escape like in rust <https://doc.rust-lang.org/reference/tokens.html#ascii-escapes>. 
    AsciiEscape {
        /// The part of the string literal body that contains this escape sequence.
        source_range: Range<usize>,
        /// The result of attempting to parse this escape sequence into a character.
        parsed: Result<char, AsciiEscapeErrorTy>,
    },

    /// An escaped single quote. 
    SingleQuoteEscape,

    /// An escaped double quote. 
    DoubleQuoteEscape,

    /// An unrecognized escape sequence. 
    UnrecognizedEscapeSequence(Range<usize>), 
}

/// An error with a unicode escape sequence. 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnicodeEscapeErrorTy {
    /// Empty escape sequence,
    Empty,
    /// The escaped digits do not represent a valid unicode codepoint. 
    InvalidCodepoint,
    /// Expected an open brace after `\u`. 
    ExpectedOpenBrace,
    /// No closing brace was supplied after the hex digits. 
    MissingClosingBrace,
    /// A non-ascii hex digit character was reached. 
    NonDigitCharacter,
    /// Too many digits supplied. 
    TooManyDigits,
}

/// An error with an ASCII escape sequence. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsciiEscapeErrorTy {
    /// Error in cases when the escaped character is higher than `0x7F` 
    /// <https://doc.rust-lang.org/reference/tokens.html#ascii-escapes>.
    HexEscapeTooHigh,

    /// Not enough hex digits supplied. 
    NotEnoughHexDigits, 

    /// The two characters following the `\x` were not ASCII hex digits.
    CharactersAreNotHexDigits,
}

impl<'str_lit> StringLiteralPartsIterator<'str_lit> {
    /// Create a new iterator for parts of a string literal. 
    fn new(str_lit_body: &'str_lit str) -> Self {
        Self { str_lit_body, iter: str_lit_body.char_indices().peekable() }
    }

    /// Get the byte offset from the start of the next character if there is a next character available.
    /// If there is no next character, return the length of the string literal body in bytes. 
    fn byte_offset(&mut self) -> usize {
        self.iter
            .peek()
            .map(|(offset, _)| *offset)
            .unwrap_or(self.str_lit_body.len())
    }
}

impl<'str_lit> Iterator for StringLiteralPartsIterator<'str_lit> {
    type Item = StringLiteralPart;

    #[rustfmt::skip] // Don't rustfmt this function. 
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next item from the iterator (or return none if none).
        match self.iter.next()? {
            // Backslash detected -- handle escape. 
            // The string cannot end in a backslash (checked by lexer) so we can use .expect here. 
            (offset, '\\') => match self.iter.next().expect("string cannot end with backslash") {
                // ASCII and quote escapes. 
                (_, '\\') => Some(StringLiteralPart::AsciiEscape { source_range: offset..offset+2, parsed: Ok('\\') }),
                (_, 'n')  => Some(StringLiteralPart::AsciiEscape { source_range: offset..offset+2, parsed: Ok('\n') }),
                (_, 'r')  => Some(StringLiteralPart::AsciiEscape { source_range: offset..offset+2, parsed: Ok('\r') }),
                (_, 't')  => Some(StringLiteralPart::AsciiEscape { source_range: offset..offset+2, parsed: Ok('\t') }),
                (_, '\'') => Some(StringLiteralPart::SingleQuoteEscape),
                (_, '\"') => Some(StringLiteralPart::DoubleQuoteEscape),

                // Do not do '\0' like rust does. 

                // ASCII byte escape. 
                (_, 'x') => match (self.iter.next(), self.iter.next()) {
                    (Some((a_offset, a)), Some((b_offset, b))) => {
                        // Make the range that contains this ASCII escape. 
                        let source_range = offset..b_offset+b.len_utf8();

                        if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() {
                            // Parse the value of the escaped hex digits.
                            let parsed_value: u8 = u8::from_str_radix(&self.str_lit_body[a_offset..a_offset+2], 16)
                                // We have confirmed the digits to be ascii hex. 
                                .expect("ASCII escape characters have been confirmed to be hex digits.");

                            if parsed_value > 0x7F {
                                Some(StringLiteralPart::AsciiEscape { source_range, parsed: Err(AsciiEscapeErrorTy::HexEscapeTooHigh) })
                            } else {
                                // Parse the character value. 
                                // SAFETY: We just confirmed that this is a byte value less than 0x7F. 
                                let parsed_value = unsafe { char::from_u32_unchecked(parsed_value as u32) };
                                Some(StringLiteralPart::AsciiEscape { source_range, parsed: Ok(parsed_value) })
                            }
                        } else {
                            Some(StringLiteralPart::AsciiEscape { source_range, parsed: Err(AsciiEscapeErrorTy::CharactersAreNotHexDigits) })
                        }
                    },

                    _ => Some(StringLiteralPart::AsciiEscape { source_range: offset..offset+2, parsed: Err(AsciiEscapeErrorTy::NotEnoughHexDigits) })
                }

                (_, 'u') => {
                    // Handle open brace.
                    if self.iter.next_if(|(_, next_char)| *next_char == '{').is_none() {
                        return Some(StringLiteralPart::UnicodeEscape { source_range: offset..offset+2, parsed: Err(UnicodeEscapeErrorTy::ExpectedOpenBrace) });
                    }

                    // Recieve up to 3 bytes (6 hex digits) containing the hex value of a unicode codepoint. 
                    let mut hex_value: u32;

                    // Match the first character after the opening brace. 
                    match self.iter.next() {
                        // No character after the opening brace. 
                        None => return 
                            Some(StringLiteralPart::UnicodeEscape { source_range: offset..offset+3, parsed: Err(UnicodeEscapeErrorTy::MissingClosingBrace) }),
                        
                        // Closing brace immediately.
                        Some((_, '}')) => return 
                            Some(StringLiteralPart::UnicodeEscape { source_range: offset..offset+4, parsed: Err(UnicodeEscapeErrorTy::Empty) }),

                        // Non-hex digit
                        Some((end_offset, c)) if !c.is_ascii_hexdigit() => {
                            return Some(StringLiteralPart::UnicodeEscape { 
                                source_range: offset..end_offset+c.len_utf8(), 
                                parsed: Err(UnicodeEscapeErrorTy::NonDigitCharacter) 
                            });
                        }

                        // Hex digit. 
                        Some((_, digit)) if digit.is_ascii_hexdigit() => {
                            // Unwrap here since we know for sure that the digit is an ASCII hex digit. 
                            hex_value = digit.to_digit(16).unwrap();
                        }

                        // Any other character or case should be unreachable here. 
                        _ => unreachable!()
                    }

                    // Track the number of digits pulled from the iterator. 
                    let mut digits_taken = 1;
                    // Take digits while the next char is not a closing brace. 
                    while self.iter.peek().map(|(_, c)| *c) != Some('}') {
                        // Get the next indexed char from the iterator. 
                        let next_indexed_char = self.iter.next();
                        
                        // If it's none, return a part.
                        if next_indexed_char.is_none() {
                            return Some(StringLiteralPart::UnicodeEscape { 
                                // Use offset+3 here because it should be close enough and I don't care to carry
                                // an offset tracker all the way through this loop. This should cover the `\u{` of the
                                // current escape sequence. 
                                source_range: offset..offset+3, 
                                parsed: Err(UnicodeEscapeErrorTy::MissingClosingBrace) 
                            });
                        }

                        // Unwrap since we know we have a non-closing character here. 
                        let (end_offset, next_char) = next_indexed_char.unwrap();
                        // Check that the next char is a digit. 
                        if !next_char.is_ascii_hexdigit() {
                            return Some(StringLiteralPart::UnicodeEscape { 
                                source_range: offset..end_offset+next_char.len_utf8(),
                                parsed: Err(UnicodeEscapeErrorTy::NonDigitCharacter) 
                            });
                        }

                        // Increment counter.
                        digits_taken += 1;
                        // Cut the string lit part if there are too many characters. 
                        if digits_taken > 6 {
                            return Some(StringLiteralPart::UnicodeEscape { 
                                source_range: offset..end_offset+1, 
                                parsed: Err(UnicodeEscapeErrorTy::TooManyDigits) 
                            });
                        }

                        // Otherwise we have a valid ASCII hex digit and not too many so we can add it to the value.
                        // We can use unwrap here since we know the next char is a valid ascii hex digit. 
                        hex_value = (hex_value << 4) | next_char.to_digit(16).unwrap();
                    }

                    // The next value from the iterator here cannot be `None` or it would be caught by the while loop. 
                    // It must be a closing brace. 
                    let (end_offset, closing_brace) = self.iter.next().expect("closing brace available");
                    // Use a debug assert to panic if it's not a closing brace in debug mode only. 
                    debug_assert_eq!(closing_brace, '}');
                    // Make the range to return, assuming that a closing brace of UTF-8 len 1 was consumed. 
                    let source_range = offset..end_offset+1;
                    // Try to convert the char. 
                    match char::from_u32(hex_value) {
                        // Failed conversion -- must be an invalid codepoint. 
                        None => Some(StringLiteralPart::UnicodeEscape { source_range, parsed: Err(UnicodeEscapeErrorTy::InvalidCodepoint) }),

                        // Successful conversion -- Return OK. 
                        Some(valid_char) => Some(StringLiteralPart::UnicodeEscape { source_range, parsed: Ok(valid_char) })
                    }
                }

                // Unrecognized escape sequence. 
                (_, other) => Some(StringLiteralPart::UnrecognizedEscapeSequence(offset..offset+1+other.len_utf8())),
            }
            
            // Non-escape handler. 
            (offset, _) => {
                // Record the starting position of the iterator. 
                let start = offset;
                // Consume string to end or just before next backslash. 
                while self.iter.next_if(|(_, c)| *c != '\\').is_some() {}
                // Get next index. 
                let end = self.byte_offset();
                // Return 
                Some(StringLiteralPart::UnescapedCharacters(start..end))
            }
            
        }
        
    }
}

impl<'str_lit> FusedIterator for StringLiteralPartsIterator<'str_lit> {}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use std::panic;
    use crate::parser::ast::expression::literal::escapes::{unescape, StringLiteralErrorTy, UnicodeEscapeErrorTy};

    #[test]
    fn test_all_ascii_byte_escapes_dont_panic_or_error() {
        for n in 0..=0x7F {
            // Create the string to test
            let source_str = format!("\\x{n:02X}");
            
            // Create the target string to compare against. 
            let target_char = char::from_u32(n).expect("valid ASCII character");
            let target_string = String::from(target_char);

            // Call the unescape function and assert that it does not panic or error. 
            assert_eq!(unescape(&source_str).unwrap().as_str(), &target_string);
        }
    }

    #[test]
    fn test_string_with_no_escapes() {
        assert_eq!(unescape("test string").unwrap().as_str(), "test string");
    }

    #[test]
    fn test_unclosed_unicode_escape() {
        assert_eq!(unescape("\\u{FFFF").unwrap_err()[0].ty, StringLiteralErrorTy::UnicodeEscapeError(UnicodeEscapeErrorTy::MissingClosingBrace));
    }


    #[test] 
    fn test_all_unicode_codepoints() {
        (0u32..=0x00FF_FFFF)
            // Use rayon's parallel iterator to speed up testing all possible unicode codepoints. 
            .into_par_iter()
            // We have to use the `try_for_each` and `panic::catch_unwind` here so that any panic on a thread
            // will cancel the execution of other testing threads. 
            .try_for_each(|n| panic::catch_unwind(|| {
                // Construct a source string.
                let source_str: String = format!("\\u{{{n:06X}}}");
                // Determine if there is a target char to match by codepoint. 
                let target_char: Option<char> = char::from_u32(n);
                // Call unescape. 
                let unescape_result = unescape(&source_str);

                // If invalid codepoint, assert the correct error type. 
                // Otherwise check that the one in the string is correct. 
                if target_char.is_none() {
                    assert_eq!(unescape_result.unwrap_err()[0].ty, StringLiteralErrorTy::UnicodeEscapeError(UnicodeEscapeErrorTy::InvalidCodepoint));
                } else {
                    assert_eq!(unescape_result.unwrap().as_str().chars().next().unwrap(), target_char.unwrap());
                }
            })).expect("A unicode codepoint is not being handled properly.")
    }

    #[test]
    fn test_empty_unicode_escape() {
        assert_eq!(unescape("\\u{}").unwrap_err()[0].ty, StringLiteralErrorTy::UnicodeEscapeError(UnicodeEscapeErrorTy::Empty))
    }
}
