/// Checks if a char is a digit
pub fn is_digit(c: char) -> bool { c >= '0' && c <= '9' }

/// Checks if a char is a hexadecimal digit.
pub fn is_hex_digit(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

/// Checks if a char is a binary digit.
pub fn is_bin_digit(c: char) -> bool { c == '0' || c == '1' }

/// Checks if a char is in the alphabet.
pub fn is_alpha(c: char) -> bool { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') }

/// Checks if a char is alphanumeric.
pub fn is_alphanumeric(c: char) -> bool { is_digit(c) || is_alpha(c) }

/// Checks if a char is whitespace.
pub fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\r' | '\t' | '\n' => true,
        _ => false,
    }
}

