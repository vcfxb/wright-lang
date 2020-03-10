use crate::grammar::ast::SelfLit;
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::testing::setup;

#[test]
fn test_self_lit() {
    let (f, h) = setup("self");
    let fr = Fragment::new(&f, h);
    let parse = SelfLit::parse(fr).unwrap();
    assert_eq!(parse.0.len(), 0);
    assert_eq!(parse.1.get_fragment().source(), "self");
}
