//! Lexer Module.
extern crate ansi_term;
use self::ansi_term::Color::*;
//use self::ansi_term::Style;
use std::fmt;
use std::collections::HashSet;
/// Module used for tracking read-head position in file.
pub mod position;
use lexer::position::*;
/// Module of functions for checking characters.
pub mod char_tests;
use lexer::char_tests::*;

#[derive(Debug, Clone)]
// todo: Docs
///
pub struct Lexer {
    current_position: Position,
    pub source: String,
    pub tokens: Vec<String>,
}

impl Lexer {
    /// Constant containing all Strings that can represent any symbol or operator.
    /// Length of every symbol is 2 characters at most.
    const SYMBOLS: [&'static str; 49] = [
        "!", "~", "^",
        "&", "&&", "|", "||",
        "+", "+=", "++",
        "-", "-=", "--",
        "*", "*=",
        "/", "/=",
        "%", "%=",
        "//", "/*", "*/",
        ":", "::", "->", ".",
        "..",
        ";",
        "(", ")", "[", "]", "{", "}",
        "=>",
        "@", "$", "#", "?",
        "==", "!=", ">", "<", ">=", "<=",
        ">>", "<<",
        "\"", "'",
    ];
    /// Constructor
    pub fn new(content: String) -> Self {
        Lexer {
            current_position: Position::new(),
            source: content,
            tokens: vec![],
        }
    }
    /// Tokenizes `self.source` and stores to `self.tokens`.
    pub fn lex(&mut self) -> Result<(), LexerError> {
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
        // while there's another character
        'consumption : while let Some(character) = chars.pop() {
            self.current_position.increment_column();
            if is_symbol(character) {
                current_line.push(character);
                current_token.push(character);
                let mut possible_next_chars: HashSet<Option<char>> = HashSet::new();
                // go through every pair, checking to see if there is a symbol
                // that equals or starts with character
                for pair in symbol_char_pairs.clone() {
                    if pair[0] == character {
                        // if so, add it to a HashSet of possible next characters
                        // (no repeats)
                        if pair.len() == 2 {
                            if !possible_next_chars.contains(&Some(pair[1])) {
                                possible_next_chars.insert(Some(pair[1]));
                            }
                        }
                        if !possible_next_chars.contains(&None) {possible_next_chars.insert(None);}
                    }
                }
                let next_char = chars.pop();
                if possible_next_chars.contains(&next_char) {
                    if let Some(n) = next_char {current_token.push(n);}
                    // if next_char is none, the iterator will end and return anyways.
                    self.tokens.push(current_token);
                    current_token = String::new();
                    // move to next iter
                    continue 'consumption;
                } else {
                    // if it's not a possible next character, put it back.
                    // use if-let statement to make sure we aren't putting back a
                    // None value (even though that is technically impossible, we don't
                    // want to take any chances.) If the value is None there should be a
                    // string with just that symbol in it in the SYMBOLS constant,
                    if let Some(n) = next_char { chars.push(n);}
                }
                // No raising errors here; all symbols starting characters have a single
                // character symbol for them in the constant, and if it doesn't match up,
                // they are just a separate token.
            }
            else {
                // todo: (this is a quick-fix)
                current_token.push(character);
                self.tokens.push(current_token);
                current_token = String::new();
            }
        }
        self.tokens.iter().for_each(|t| println!("{}", t) );
        return Ok(());
    }

}

#[derive(Debug, Clone)]
/// Structure for lexer errors.
pub struct LexerError {
    pub module_name: String,
    pub position: Position,
    pub info: String,
    pub line: String,
    // not pub; access only in this mod.
    arrow_str: String
}

impl LexerError {
    /// Constructor.
    pub fn new(arg_position: Position, current_line: String) -> Self {
        LexerError {
            module_name: String::new(),
            position: arg_position,
            info: String::new(),
            line: current_line,
            arrow_str: String::new(),
        }
    }
    /// Sets info string based on an expected character and the character that was found.
    /// Auto-generates error message.
    pub fn set_info(&mut self, expected: char, found: Option<char>) {
        self.info = format!("Expected {} found {} character.", expected,
            // conversion from char -> String
            if let Some(n) = found {
                let mut temp_slice: [u8; 4] = [0;4];
                let n_as_str = n.encode_utf8(&mut temp_slice);
                n_as_str.to_string()
            } else {"<none>".to_string()});
        let current_line_borrow = self.line.clone();
        for c in current_line_borrow.chars().take(self.position.col-1) {
            match c {
                '\t' => self.arrow_str.push('\t'),
                _ =>  self.arrow_str.push(' '),
            }
        }
        self.arrow_str.push('^');
    }
    /// Very similar to `set_info` however this method takes a `Vec<char>` argument,
    /// specifying that any of those characters would have been acceptable coming next.
    pub fn set_info_as_vec(&mut self, expected: Vec<char>, found: Option<char>) {
        // formats the error message properly.
        let mut expected_string = "( ".to_string();
        for e in expected {
            expected_string.push(e);
            expected_string.push_str(" | ");
        }
        // remove extra '| '
        expected_string.pop();
        expected_string.pop();
        // close with ')'
        expected_string.push(')');
        self.info = format!("Expected {} found {} character.", expected_string,
            // conversion from char -> String
            if let Some(n) = found {
                let mut temp_slice: [u8; 4] = [0;4];
                let n_as_str = n.encode_utf8(&mut temp_slice);
                n_as_str.to_string()
            } else {"<none>".to_string()});
        let current_line_borrow = self.line.clone();
        for c in current_line_borrow.chars().take(self.position.col-1) {
            match c {
                '\t' => self.arrow_str.push('\t'),
                _ =>  self.arrow_str.push(' '),
            }
        }
        self.arrow_str.push('^');
    }
}

/// Display formatting for LexerError.
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name}: {module}:{line}:{col}:\n{five}{i}\n{five} {b}\n{line:>width$} {b} {l}\n{five} {b} {a}\n",
            name = Red.paint("LexerError"),
            line = self.position.line,
            width = 5,
            module = Cyan.paint(self.module_name.clone()),
            col = self.position.col,
            five = " ".repeat(5),
            i = Blue.paint(self.info.clone()),
            l = Green.paint(self.line.clone()),
            b = Blue.paint("|"),
            a = Red.bold().paint(self.arrow_str.clone()))
    }
}