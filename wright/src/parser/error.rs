use std::fmt;
use position::Position;
extern crate ansi_term;
use interpreter::ERROR_COLORS;

#[derive(Debug, Clone)]
/// Structure for parser errors.
pub struct ParserError {
    // none are pub as access should be in this module and the impls for ParserError only
    module_name: String,
    start_position: Position,
    end_position: Position,
    info: String,
    first_line: String,
    last_line: Option<String>,
    first_line_arrow_str: String,
    last_line_arrow_str: String,
}

impl ParserError {
    /// Constructor.
    /// Error info defaults to an empty string.
    pub fn new(
        arg_module_name: String,
        arg_start_position: Position,
        arg_first_line: String,
        arg_end_position: Position,
        arg_last_line: Option<String>,
    ) -> Self {
        ParserError {
            module_name: arg_module_name,
            start_position: arg_start_position,
            first_line: arg_first_line.clone(),
            last_line: arg_last_line.clone(),
            end_position: arg_end_position,
            info: String::new(),
            first_line_arrow_str: if arg_start_position.get_line() != arg_end_position.get_line() {
                let mut result = String::new();
                for c in arg_first_line.clone().chars().take(arg_start_position.get_col()-1) {
                    match c {
                        '\t' => {result.push('\t');},
                        _ => {result.push(' ');},
                    }
                }
                for c in arg_first_line.clone().chars().skip(arg_start_position.get_col()) {
                    match c {
                        '\t' => {result.push('\t');},
                        _ => {result.push('^');},
                    }
                }
                result
            } else {
                let mut result = String::new();
                for c in arg_first_line.clone().chars().take(arg_start_position.get_col()-1) {
                    match c {
                        '\t' => {result.push('\t');},
                        _ => {result.push(' ');},
                    }
                }
                for c in arg_first_line
                    .clone()
                    .chars()
                    .skip(arg_start_position.get_col())
                    .take(arg_end_position.get_col() - arg_start_position.get_col()) {
                    match c {
                        '\t' => {result.push('\t');},
                        _ => {result.push('^');},
                    }
                }
                result
            },
            last_line_arrow_str: if arg_start_position.get_line() != arg_end_position.get_line() {
                let mut result = String::new();
                for c in arg_last_line.clone().unwrap().chars().take(arg_end_position.get_col()-1) {
                    match c {
                        '\t' => {result.push('\t');},
                        _ => {result.push(' ');},
                    }
                }
                for c in arg_last_line.clone().unwrap().chars().skip(arg_end_position.get_col()) {
                    match c {
                        '\t' => {result.push('\t');},
                        _ => {result.push('^');},
                    }
                }
                result
            } else { String::new() },
        }
    }

    /// Sets info string based on an expected token and the token that was found.
    /// Follows builder style.
    pub fn set_info(self, expected: String, found: Option<String>) -> Self {
        let mut self_cloned = self.clone();
        self_cloned.info = format!("Expected \"{}\" found {}.", expected,
            // conversion from char -> String
            if found.is_some() {["\"", found.unwrap().as_str(), "\""].concat()}
            else {"end of file".to_string()});
        return self_cloned;
    }

    /// Very similar to `set_info` however this method takes a `Vec<String>` argument,
    /// specifying that any of those tokens would have been acceptable coming next.
    /// Follows builder style.
    /// Panics if `expected` is an empty vector.
    pub fn set_info_as_vec(self, expected: Vec<String>, found: Option<String>) -> Self {
        // formats the error message properly.
        let mut expected_string = "\"".to_string();
        let mut self_cloned = self.clone();
        for e in expected[0..expected.len()-1].iter() {
            expected_string.push_str(e.as_str());
            expected_string.push_str("\", \"");
        }
        // remove extra ', '
        for _ in 0..3 {expected_string.pop();}
        expected_string.push_str(" or \"");
        expected_string.push_str([expected.last().unwrap(), "\""].concat().as_str());
        self_cloned.info = format!("Expected {} found {}.", expected_string,
            if found.is_some() { ["\"", found.unwrap().as_str(), "\""].concat()}
            else {"end of file".to_string()});
        return self_cloned;
    }
    /// Very similar to `set_info` however it takes a single `String` argument.
    /// Sets info to a string directly.
    /// Follows builder style.
    pub fn set_info_raw(self, arg_info: &'static str) -> Self {
        let mut self_cloned = self.clone();
        self_cloned.info = arg_info.to_string();
        return self_cloned;
    }
}

/// Display formatting for ParserError.
/// Uses the [ansi_term](https://crates.io/crates/ansi_term) crate to color output.
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !(self.last_line.is_some()) {
            write!(f,"{name} in {module} on line {line} characters {s_char}->{e_char}:\n{five}{i}\n\
                    {five} {b}\n{line:>width$} {b} {l}\n{five} {b} {a}\n",
                name = ERROR_COLORS[0].paint("ParserError"),
                line = self.start_position.get_line(),
                width = 5,
                module = ERROR_COLORS[1].paint(self.module_name.clone()),
                s_char = self.start_position.get_col(),
                e_char = self.end_position.get_col(),
                five = " ".repeat(5),
                i = ERROR_COLORS[3].paint(self.info.clone()),
                l = ERROR_COLORS[2].paint(self.first_line.clone()),
                b = ERROR_COLORS[3].paint("|"),
                a = ERROR_COLORS[0].bold().paint(self.first_line_arrow_str.clone())
            )
        } else {
            write!(f, "{name} in {module} from {s_line}:{s_col} to {e_line}:{e_col}:\n{five}{i}\n\
                    {five} {b}\n{s_line:>width$} {b} {s_source}\n{five} {b} {a_one}\n{five} {b} \
                    ...\n{e_line:>width$} {b} {e_source}\n{five} {b} {a_two}\n",
                name = ERROR_COLORS[0].paint("ParserError"),
                s_line = self.start_position.get_line(),
                e_line = self.end_position.get_line(),
                width = 5,
                module = ERROR_COLORS[1].paint(self.module_name.clone()),
                s_col = self.start_position.get_col(),
                e_col = self.end_position.get_col(),
                five = " ".repeat(5),
                i = ERROR_COLORS[3].paint(self.info.clone()),
                s_source = ERROR_COLORS[2].paint(self.first_line.clone()),
                e_source = ERROR_COLORS[2].paint(self.last_line.clone().unwrap()),
                b = ERROR_COLORS[3].paint("|"),
                a_one = ERROR_COLORS[0].bold().paint(self.first_line_arrow_str.clone()),
                a_two = ERROR_COLORS[0].bold().paint(self.last_line_arrow_str.clone()),
            )
        }
    }
}