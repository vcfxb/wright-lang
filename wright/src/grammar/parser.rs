use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct WrightParser;