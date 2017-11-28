//! Lexer Module.

/// Module used for tracking read-head position in file.
pub mod position;
use lexer::position::*;
/// Module of functions for checking characters.
pub mod char_tests;
use lexer::char_tests::*;

/// Module for defining, tracking, and printing lexer related errors.
pub mod error;
use lexer::error::LexerError;

use std::collections::HashSet;

#[derive(Debug, Clone)]
/// Lexer struct, which stores publicly a `tokens` field
/// which is generated using the `lex` method.  Tokens will be an internal
/// representation of source code, sliced in to parsable "lexemes" or "tokens".
pub struct Lexer {
    // source doesn't need pub right?
    source: String,
    pub tokens: Vec<String>,
}

impl Lexer {
    /// Constant containing all Strings that can represent any symbol or operator.
    /// Length of every symbol is 2 characters at most.
    /// Not all symbols currently have functionality in the wright language.
    pub const SYMBOLS: [&'static str; 55] = [
        "!", "~", "^", "=",
        "&", "&&", "|", "||",
        "+", "+=", "++",
        "-", "-=", "--",
        "*", "*=",
        "/", "/=",
        "%", "%=",
        "//", "/*", "*/",
        "/!","/?","?/", // doc comments
        ":", "::", "->", ".",
        "..",
        ";",
        "(", ")", "[", "]", "{", "}",
        "=>",
        "@", "#", "?", "$",
        "?!", // for compiler builtin checks
        "==", "!=", ">", "<", ">=", "<=",
        ">>", "<<",
        "\"", "'",
        "`",
    ];
    /// Constructor.
    /// Content argument is source code written in wright.
    pub fn new(content: String) -> Self {
        Lexer {
            source: content,
            tokens: vec![],
        }
    }
    /// Tokenizes `self.source` and stores to `self.tokens`.
    /// #### Is completely loss-less.
    /// No source-code is lost in this conversion, it's all just split into parsable tokens.
    /// Note that this lexing follows the rules of the Wright language syntax, detailed in the
    /// Wright book docs.
    pub fn lex(&mut self) -> Result<(), LexerError> {
        let mut current_position = Position::new();
        let mut current_token = String::new();
        let mut current_line = String::new();
        let mut chars: Vec<char> = self.source.chars().collect();
        // Turns symbol list into workable vector of Vec<char>
        let symbol_char_pairs: Vec<Vec<char>> = Lexer::SYMBOLS
            .iter()
            .map(|x| x
                .to_string()
                .chars()
                .collect()
            ).collect();
        // reverse chars so that pop() and push() read L->R, Top->End
        chars.reverse();
        //println!("{:?}", symbol_char_pairs);
        // while there's another character
        'consumption : while let Some(character) = chars.pop() {
            current_position.increment_column();
            current_line.push(character);
            current_token.push(character);
            if is_symbol(character) {
                let mut possible_next_chars: HashSet<char> = HashSet::new();
                // go through every pair, and add the second character if it starts with `character`
                for pair in symbol_char_pairs.clone() {
                    if pair.len() == 2 && pair[0] == character {
                        if !possible_next_chars.contains(&pair[1]) {
                            possible_next_chars.insert(pair[1]);
                        }
                    }
                }
                // get the next character if possible
                if let Some(next_char) = chars.pop() {
                    if possible_next_chars.contains(&next_char) {
                        current_token.push(next_char);
                        current_line.push(next_char);
                        current_position.increment_column();
                        // special case with double char symbols (not quotes)
                        match current_token.clone().as_str() {
                            "//"|"/!" => {   // single line comment or single line doc comment
                                // if EOF is reached, this will just stop and
                                // push the current token.
                                'take_comment : while let Some(comment_char) = chars.pop() {
                                    // until end of line
                                    // follow principals of true loss-less lexing;
                                    // the newline character will be put in the token
                                    current_token.push(comment_char);
                                    current_line.push(comment_char);
                                    current_position.increment_column();
                                    if comment_char == '\n' {
                                        current_position.increment_line();
                                        current_line = String::new();
                                        break 'take_comment;
                                    }
                                }
                            },
                            "/*" => {   // multi line comments
                                let mut last = ' ';
                                'take_multiline_comment: while let Some(comment_char) = chars.pop(){
                                    current_position.increment_column();
                                    current_token.push(comment_char);
                                    if comment_char == '\n' {
                                        current_position.increment_line();
                                        current_line = String::new();
                                    }
                                    else if comment_char == '/' && last == '*'{
                                        break 'take_multiline_comment;
                                    }
                                    last = comment_char;
                                }

                            },
                            "/?" => {   // multi line doc comments
                                let mut last = ' ';
                                'take_multi_doc_comment: while let Some(comment_char) = chars.pop(){
                                    current_position.increment_column();
                                    current_token.push(comment_char);
                                    if comment_char == '\n' {
                                        current_position.increment_line();
                                        current_line = String::new();
                                    }
                                        else if comment_char == '/' && last == '?'{
                                            break 'take_multi_doc_comment;
                                        }
                                    last = comment_char;
                                }
                            },
                            _ => {},
                        }
                        // factored out of match statement
                        self.tokens.push(current_token);
                        current_token = String::new();
                        // move to next iteration
                    } else {
                        // put the next_char back on the char stack if it doesn't make a possible
                        // token
                        chars.push(next_char);
                        // single symbol token so far, no eof reached
                        match current_token.clone().as_str() {
                            "\"" => {
                                'take_quote : while let Some(quote_char) = chars.pop() {
                                    current_position.increment_column();
                                    current_line.push(quote_char);
                                    current_token.push(quote_char);
                                    if quote_char == '\n' {
                                        current_position.increment_line();
                                        current_line = String::new();
                                    }
                                    // escaped characters
                                    else if quote_char == '\\' {
                                        if let Some(escaped_char) = chars.pop() {
                                            current_position.increment_column();
                                            current_line.push(escaped_char);
                                            current_token.push(escaped_char);
                                            if escaped_char == '\n' {
                                                current_position.increment_line();
                                                current_line = String::new();
                                            }
                                        } else {
                                            // reach EOF and break.
                                            break 'take_quote;
                                        }
                                    }
                                    else if quote_char == '"' {     // end of quote reached
                                        break 'take_quote;
                                    }
                                }
                            },
                            "'" => {
                                'take_char_literal: while let Some(char_literal_char) = chars.pop(){
                                    current_token.push(char_literal_char);
                                    current_position.increment_column();
                                    current_line.push(char_literal_char);
                                    if char_literal_char == '\n' {
                                        current_position.increment_line();
                                        current_line = String::new();
                                    }
                                    if char_literal_char == '\'' {
                                        break 'take_char_literal;
                                    }
                                }
                                if current_token.len() == 1 {
                                    let char_literal_error = LexerError::
                                        new(current_position.clone(), current_line)
                                        .set_info_as_string("character literal", None);
                                    return Err(char_literal_error);
                                } else if current_token.len() == 2 {
                                    let token_chars: Vec<char> = current_token
                                        .clone()
                                        .chars()
                                        .collect();
                                    match token_chars[1] {
                                        '\'' => {
                                            current_position.decrement_column();
                                            let char_literal_error = LexerError::
                                                new(current_position.clone(), current_line)
                                                .set_info_as_string("character literal",Some('\''));
                                            return Err(char_literal_error);
                                        },
                                        _ => {
                                            let char_literal_error = LexerError::
                                                new(current_position.clone(), current_line)
                                                .set_info('\'', None);
                                            return Err(char_literal_error);
                                        },
                                    }
                                } else if current_token.len() == 3 { 
                                    let token_chars: Vec<char> = current_token.clone()
                                        .chars()
                                        .collect();
                                    if !(token_chars[2] == '\'') {
                                        current_position.decrement_column();
                                        let char_literal_error = LexerError::
                                            new(current_position.clone(), current_line)
                                            .set_info('\'',Some(token_chars[2]));
                                        return Err(char_literal_error);
                                    } // otherwise ok.
                                } else if current_token.len() == 4 {
                                    let token_chars: Vec<char> = current_token.clone()
                                        .chars()
                                        .collect();
                                    if token_chars[1] != '\\' || token_chars[3] != '\'' {
                                        if token_chars[1] != '\'' {
                                            // twice to move back to offending char
                                            for _ in 0..2 {current_position.decrement_column();}
                                            let char_literal_error = LexerError::
                                                new(current_position.clone(), current_line)
                                                .set_info('\'',Some(token_chars[2]));
                                            return Err(char_literal_error);
                                        }
                                        if token_chars[3] !='\'' {
                                            let char_literal_error = LexerError::
                                                new(current_position.clone(), current_line)
                                                .set_info('\'',None);
                                            return Err(char_literal_error);
                                        }
                                    } // otherwise all good
                                } else {
                                    current_position.decrement_column();
                                    let char_literal_error = LexerError::
                                    new(current_position.clone(), current_line)
                                        .set_info_raw("Character literal is too long.");
                                    return Err(char_literal_error);
                                }
                            },
                            _ => {}, //otherwise do nothing
                        }
                    }
                }
                // found one symbol character and reached eof...
                self.tokens.push(current_token);
                current_token = String::new();

            }
            else if is_alpha(character) {
                // take chars for an identifier. (a-z, 0-9, _)
                // is_alpha could also imply the start of a keyword
                // but that doesn't really matter at this point.
                'take_identifier : while let Some(next_char) = chars.pop() {
                    if is_alphanumeric(next_char) || next_char == '_' {
                        current_position.increment_column();
                        current_token.push(next_char);
                        current_line.push(next_char);
                    } else {
                        chars.push(next_char);
                        self.tokens.push(current_token);
                        current_token = String::new();
                        break 'take_identifier;
                    }
                }
            }
            else if is_digit(character) {
                // take chars for a number literal.
                let mut had_decimal = false;
                if character == '0' {
                    if let Some('x') = chars.pop() {
                        current_token.push('x');
                        current_line.push('x');
                        current_position.increment_column();
                        'take_hex_literal: while let Some(next_char) = chars.pop() {
                            if is_hex_digit(next_char) {
                                current_position.increment_column();
                                current_line.push(next_char);
                                current_token.push(next_char);
                            }
                            else {
                                // not a digit; put it back
                                chars.push(next_char);
                                break 'take_hex_literal;
                            }
                        }
                    }
                    else if let Some('b') = chars.pop() {
                        current_token.push('b');
                        current_line.push('b');
                        current_position.increment_column();
                        'take_bin_literal: while let Some(next_char) = chars.pop() {
                            if is_bin_digit(next_char) {
                                current_position.increment_column();
                                current_line.push(next_char);
                                current_token.push(next_char);
                            }
                            else {
                                // not a digit; put it back
                                chars.push(next_char);
                                break 'take_bin_literal;
                            }
                        }
                    }
                    else {
                        // very similar to below, takes for decimal or integer.
                        'take_num_literal: while let Some(next_char) = chars.pop() {
                            if is_digit(next_char) || (next_char == '.' && !had_decimal) {
                                current_position.increment_column();
                                current_line.push(next_char);
                                current_token.push(next_char);
                                if next_char == '.' {had_decimal = true;}
                            }
                            else {
                                // not a digit; put it back
                                chars.push(next_char);
                                break 'take_num_literal;
                            }
                        }
                    }
                } else {
                    // not a hex or binary
                    let mut had_decimal = false;
                    'take_dec_literal: while let Some(next_char) = chars.pop() {
                        if is_digit(next_char) || (next_char == '.' && !had_decimal) {
                            current_position.increment_column();
                            current_line.push(next_char);
                            current_token.push(next_char);
                            if next_char == '.' {had_decimal = true;}
                        }
                        else {
                            // not a digit; put it back
                            chars.push(next_char);
                            break 'take_dec_literal;
                        }
                    }
                }
                // push token and reset
                // this code is common to all possible lexemes, and is therefore factored out
                self.tokens.push(current_token);
                current_token = String::new();
            }
            else {
                // todo: (this is a temp-fix)
                self.tokens.push(current_token);
                current_token = String::new();
            }
        }
        return Ok(());
    }
}