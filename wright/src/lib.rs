#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
//#![warn(trivial_casts, trivial_numeric_casts)]
//#![warn(unused_extern_crates, unused_qualifications)]

pub mod version;
pub mod position;
pub mod errors;
pub mod parser;
pub mod lexer;
pub mod interpreter;
pub mod jvm;
