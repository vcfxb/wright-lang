use crate::grammar::ast::BooleanLit;
use crate::grammar::model::Fragment;
use codespan::Files;
use crate::grammar::parsers::testing::setup;

#[test]
fn test_bool_lit() {
    let (f, h) = setup("true");
    let fr = Fragment::new(&f, h);
    match BooleanLit::parse(fr) {
        Ok((rem, bool_lit)) => {
            assert_eq!(rem.len(), 0);
            assert_eq!(bool_lit.frag.get_span(), fr.get_span());
            assert_eq!(bool_lit.inner, true);
        }
        Err(e) => {
            eprintln!("{:#?}", e);
            panic!();
        }
    }
}
