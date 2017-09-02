// preprocessing function to remove comments from code.
// todo: line number tracking.
pub fn preproc(input_string: String) -> String {
    let mut return_string: String = String::new();
    // no two will ever be true simultaneously.
    let mut in_quotes = false;
    let mut in_multiline_comment = false;
    let mut in_single_line_comment = false;
    let mut last_char: char = ' ';
    let input_vec = input_string.into_bytes();
    for byte in input_vec {
        let character = byte as char;
        if in_quotes {
            if character == '"' {
                in_quotes = false;
            }
            return_string.push(character);
        } else if in_multiline_comment {
            if character == '/' && last_char == '*' {
                in_multiline_comment = false;
            }
        } else {
            // not in quote or multi line comment.
            if in_single_line_comment && character == '\n' {
                in_single_line_comment = false;
            }
            if !in_single_line_comment {
                if character == '"' {
                    in_quotes = true;
                    return_string.push(character);
                } else if character == '/' && last_char == '/' {
                    in_single_line_comment = true;
                    return_string.pop(); // remove previous '/'
                } else if character == '*' && last_char == '/' {
                    in_multiline_comment = true;
                    return_string.pop(); // remove previous '/'
                } else {
                    return_string.push(character);
                }
            }
        }
        last_char = character;
    }
    return return_string;
}
