#![allow(missing_docs)]
use pest::Parser;


/// Wright's parser, generated from a PEST grammar.
#[derive(Parser, Copy, Clone, Debug)]
#[grammar = "grammar/grammar.pest"]
pub struct WrightParser;

