#![warn(missing_copy_implementations)]
#![warn(missing_docs)]

extern crate regex;
#[macro_use]
extern crate pest_derive;
extern crate pest;
extern crate codespan;
extern crate codespan_reporting;
extern crate clap;

pub mod version;
pub mod interpreter;
pub mod target;
pub mod parser;
