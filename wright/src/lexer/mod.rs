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
    const SYMBOLS: [&'static str; 55] = [
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
        "@", "$", "#", "?",
        "?!", // for compiler builtin checks
        "==", "!=", ">", "<", ">=", "<=",
        ">>", "<<",
        "\"", "'",
        "`",
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
        //println!("{:?}", symbol_char_pairs);
        // while there's another character
        'consumption : while let Some(character) = chars.pop() {
            self.current_position.increment_column();
            current_line.push(character);
            current_token.push(character);
            if is_symbol(character) {
                // todo: test and fix
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
                        self.current_position.increment_column();
                        // special cases
                        // todo: finish special cases
                        match current_token.clone().as_str() {
                            "//" => {   // single line comment
                                // if EOF is reached, this will just stop and
                                // push the current token.
                                'take_comment : while let Some(comment_char) = chars.pop() {
                                    // until end of line
                                    // follow principals of true loss-less lexing;
                                    // the newline character will be put in the token,
                                    // since it's not worth the work of putting back on the stack.
                                    // todo: Wright Book on Single Line Comments
                                    if comment_char != '\n' {
                                        current_token.push(comment_char);
                                        current_line.push(comment_char);
                                        self.current_position.increment_column();
                                    } else {
                                        current_token.push(comment_char);
                                        current_line.push(comment_char);
                                        self.current_position.increment_line();
                                        current_line = String::new();
                                        break 'take_comment;
                                    }
                                }
                            },
                            _ => {},
                        }
                        self.tokens.push(current_token);
                        current_token = String::new();
                        // move to next iter
                    } else {
                        // put the next_char back on the char stack if it doesn't make a possible token
                        chars.push(next_char);
                        self.tokens.push(current_token);
                        current_token = String::new();
                    }
                } else {
                    self.tokens.push(current_token);
                    current_token = String::new();
                }
            }
            else if is_alpha(character) {
                // todo: Wright Book variable and identifier names
                //
                // take chars for an identifier. (a-z, 0-9, _)
                // is_alpha could also imply the start of a keyword
                // but that doesn't really matter at this point.
                'take_identifier : while let Some(next_char) = chars.pop() {
                    if is_alphanumeric(next_char) || next_char == '_' {
                        self.current_position.increment_column();
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
            else {
                // todo: (this is a temp-fix)
                self.tokens.push(current_token);
                current_token = String::new();
            }
        }
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