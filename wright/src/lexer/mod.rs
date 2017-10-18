use preproc::NumberedString;

pub fn lex_vec_numbered_line(lines: Vec<NumberedString>) { // todo: add return type
    // this is the point where Some language syntax is defined
    // The docs for this syntax is somewhere in the
}

pub struct LexResult {
    //in_quotation: bool,
    content: Vec<String>,
    line_num: u64,
}