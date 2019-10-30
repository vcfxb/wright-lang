#![allow(missing_docs)]
//! Definitions of keywords and other constant tokens used in parser.

macro_rules! token {
    ($i:ident, $j:expr) => {pub const $i: &'static str = $j;};
}

token!(TRUE, "true");
token!(FALSE, "false");
token!(BOOLEAN, "boolean");
token!(INTEGER, "integer");
token!(UNSIGNED, "unsigned");
token!(IF, "if");
token!(THEN, "then");
token!(ELSE, "else");
token!(START_MULTILINE_COMMENT, "#*");
token!(END_MULTILINE_COMMENT, "*#");
token!(LINE_COMMENT, "#");
token!(DOC_COMMENT, "#?");
token!(MODULE_HEADER, "#?!");
token!(PUBLIC, "pub");
token!(TYPE, "type");
token!(CLASS, "class");
token!(STRUCT, "struct");
token!(CONST, "const");
token!(USE, "use");
token!(DEFINE, "def");
token!(VARIABLE, "var");
token!(VALUE, "val");
token!(IMPLEMENT, "impl");
token!(FUNCTION, "fn");
token!(INLINE, "inline");
token!(OR, "or");
token!(AND, "and");
token!(SELF_V, "self");
token!(SELF_T, "Self");
token!(BYTE, "byte");
token!(INT, "integer");
token!(LONG, "unsigned");
token!(STR, "string");
token!(CHAR, "character");

pub const RESERVED_TOKENS: [&'static str; 34] = [
    TRUE, FALSE, BOOLEAN, INTEGER, UNSIGNED, IF, ELSE, THEN,
    START_MULTILINE_COMMENT, END_MULTILINE_COMMENT, LINE_COMMENT, DOC_COMMENT,
    MODULE_HEADER, PUBLIC, TYPE, CLASS, STRUCT, CONST, USE, DEFINE, VALUE,
    VARIABLE, IMPLEMENT, FUNCTION, INLINE, SELF_T, SELF_V, OR, AND,
    BYTE, INT, LONG, CHAR, STR,
];