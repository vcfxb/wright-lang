//extern crate regex;
//use super::errors::*;
//use self::regex::Regex;

// todo: docs

/// Lexer for wright. Splits input file into
pub fn lex_lines(lines: String) -> Vec<String> {
    // this is the point where some language syntax is defined
    let mut ret_vec: Vec<String> = vec![];
    let mut current_word: String = String::new();
    let mut last_char = ' ';
    for character in lines.as_str().chars() {
        if character.is_alphabetic() {
            current_word.push(character);
        } else {
            if !current_word.is_empty() {
                ret_vec.push(current_word);
                current_word = String::new();
            }
            let mut buffer: [u8; 4] = [0; 4];
            ret_vec.push(character.encode_utf8(&mut buffer).to_string());
        }
    }
    return ret_vec;
}