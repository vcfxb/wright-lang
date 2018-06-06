#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest"); // relative to this file

/// Wright's Parser.
#[derive(Parser)]
#[grammar = "grammar/grammar.pest"] // relative to src
struct WrightParser;