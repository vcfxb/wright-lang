use std::fmt;
use position::Position;
extern crate ansi_term;
use errors::ERROR_COLORS;

#[derive(Debug, Clone)]
/// Structure for lexer errors.
pub struct LexerError {
    // none are pub as access should be in this module and the impl LexerError only
    module_name: String,
    position: Position,
    info: String,
    line: String,
    arrow_str: String
}

impl LexerError {
    /// Constructor.
    /// Defaults module name to "AnonymousModule".
    /// Defaults info name to "There was an error while lexing.".
    /// Use `set_module_name` and the info setter methods to change this.
    pub fn new(arg_position: Position, current_line: String) -> Self {
        LexerError {
            module_name: "AnonymousModule".to_string(),
            position: arg_position,
            info: "There was an error while lexing.".to_string(),
            line: current_line,
            arrow_str: String::new(),
        }
    }
    /// Sets info string based on an expected character and the character that was found.
    /// Auto-generates error message.
    /// Follows builder style.
    pub fn set_info(self, expected: char, found: Option<char>) -> Self {
        let mut self_cloned = self.clone();
        self_cloned.info = format!("Expected '{}' found {}.", expected,
                            // conversion from char -> String
                            if let Some(n) = found {
                                let mut temp_slice: [u8; 4] = [0;4];
                                let n_as_str = n.encode_utf8(&mut temp_slice);
                                ["'", n_as_str, "'"].concat()
                            } else {"end of file".to_string()});
        let current_line_borrow = self.line.clone();
        for c in current_line_borrow.chars().take(self.position.get_col()-1) {
            match c {
                '\t' => self_cloned.arrow_str.push('\t'),
                _ =>  self_cloned.arrow_str.push(' '),
            }
        }
        self_cloned.arrow_str.push('^');
        return self_cloned;
    }
    /// Very similar to `set_info` however it takes a `String` argument.
    /// Sets info string based on a string and the character that was found.
    /// Automatically generates an error message.
    /// Follows builder style.
    pub fn set_info_as_string(self, expected: &'static str, found: Option<char>) -> Self {
        let mut self_cloned = self.clone();
        self_cloned.info = format!("Expected {} found {}.", expected,
                            // conversion from char -> String
                            if let Some(n) = found {
                                let mut temp_slice: [u8; 4] = [0;4];
                                let n_as_str = n.encode_utf8(&mut temp_slice);
                                ["'", n_as_str, "'"].concat()
                            } else {"end of file".to_string()});
        let current_line_borrow = self.line.clone();
        for c in current_line_borrow.chars().take(self.position.get_col()-1) {
            match c {
                '\t' => self_cloned.arrow_str.push('\t'),
                _ =>  self_cloned.arrow_str.push(' '),
            }
        }
        self_cloned.arrow_str.push('^');
        return self_cloned;
    }
    /// Sets the module name used by the error reporting and formatting system.
    pub fn set_module_name(&mut self, name: String) {
        self.module_name = name;
    }
    /// Very similar to `set_info` however this method takes a `Vec<char>` argument,
    /// specifying that any of those characters would have been acceptable coming next.
    /// Automatically generates an error message.
    /// Follows builder style.
    /// Panics if `expected` is an empty vector.
    pub fn set_info_as_vec(self, expected: Vec<char>, found: Option<char>) -> Self {
        // formats the error message properly.
        let mut expected_string = "'".to_string();
        let mut self_cloned = self.clone();
        for e in expected[0..expected.len()-1].iter() {
            expected_string.push(*e);
            expected_string.push_str("', '");
        }
        // remove extra ', '
        for _ in 0..3 {expected_string.pop();}
        expected_string.push_str(" or ");
        expected_string.push(*(expected.last().unwrap()));
        self_cloned.info = format!("Expected {} found {}.", expected_string,
                            // conversion from char -> String
                            if let Some(n) = found {
                                let mut temp_slice: [u8; 4] = [0;4];
                                let n_as_str = n.encode_utf8(&mut temp_slice);
                                ["'", n_as_str, "'"].concat()
                            } else {"end of file".to_string()});
        let current_line_borrow = self.line.clone();
        for c in current_line_borrow.chars().take(self.position.get_col()-1) {
            match c {
                '\t' => self_cloned.arrow_str.push('\t'),
                _ =>  self_cloned.arrow_str.push(' '),
            }
        }
        self_cloned.arrow_str.push('^');
        return self_cloned;
    }
    /// Very similar to `set_info` however it takes a `String` argument.
    /// Sets info to a string directly.
    /// Follows builder style.
    pub fn set_info_raw(self, arg_info: &'static str) -> Self {
        let mut self_cloned = self.clone();
        self_cloned.info = arg_info.to_string();
        let current_line_borrow = self.line.clone();
        for c in current_line_borrow.chars().take(self.position.get_col()-1) {
            match c {
                '\t' => self_cloned.arrow_str.push('\t'),
                _ =>  self_cloned.arrow_str.push(' '),
            }
        }
        self_cloned.arrow_str.push('^');
        return self_cloned;
    }
}

/// Display formatting for LexerError.
/// Uses the [ansi_term](https://crates.io/crates/ansi_term) crate to color output.
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name} in {module} at {line}:{col}:\n{five}{i}\n{five} {b}\n{line:>width$} {b} \
                {l}\n{five} {b} {a}\n",
            name = ERROR_COLORS[0].paint("LexerError"),
            line = self.position.get_line(),
            width = 5,
            module = ERROR_COLORS[1].paint(self.module_name.clone()),
            col = self.position.get_col(),
            five = " ".repeat(5),
            i = ERROR_COLORS[3].paint(self.info.clone()),
            l = ERROR_COLORS[2].paint(self.line.clone()),
            b = ERROR_COLORS[3].paint("|"),
            a = ERROR_COLORS[1].bold().paint(self.arrow_str.clone())
        )
    }
}