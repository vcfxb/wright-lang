#![allow(missing_docs)]


/// Wright's parser, generated from a PEST grammar.
#[derive(Parser, Copy, Clone, Debug)]
#[grammar = "grammar/grammar.pest"]
pub struct WrightParser;

/// Wright Parser rule type alias.
pub type WrightRule = Rule;
