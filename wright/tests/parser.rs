#[macro_use]
extern crate pest;

use wright::grammar::parser::{WrightParser, Rule};


#[test]
fn ignored() {

}

#[test]
fn integers() {
    parses_to!(
        parser: WrightParser,
        input: "1000",
        rule:  Rule::NUM_LITERAL,
        tokens: [ NUM_LITERAL(0,4, [ DEC_LITERAL(0,4) ]) ]
    )
}

#[test]
fn ident() {
}