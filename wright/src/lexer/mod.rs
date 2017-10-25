//! Lexer module for Wright. Prepares the input file's content for parsing.
//!

extern crate regex;
//use super::errors::*;
use self::regex::Regex;

/// Lexer for wright. Splits input file into a vector of strings
/// (`Vec<String>`) for further processing. Preserves all original
/// information, but groups by text, then numerically, then by whitespace.
pub fn lex_lines(lines: String) -> Vec<String> {
    // this is the point where some language syntax is defined
    let mut ret_vec: Vec<String> = vec![];
    let mut current_word: String = String::new();
    // first pass
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
    // second pass
    let first_vec = ret_vec;
    ret_vec = vec![];
    current_word = String::new();
    let digit_regex = Regex::new(r"[[:digit:]]").unwrap();
    for word in first_vec {
        if digit_regex.is_match(&word) {
            current_word.push_str(&word);
        } else {
            if !current_word.is_empty() {
                ret_vec.push(current_word);
                current_word = String::new();
            }
            ret_vec.push(word);
        }
    }
    return ret_vec;
}