use preproc::NumberedString;
pub fn lex_numbered_line() {
    
}

pub struct LexResult {
    in_quotation: bool,
    content: Vec<String>,
    line_num: u64,
}