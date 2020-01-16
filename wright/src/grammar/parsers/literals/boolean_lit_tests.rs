use codespan::Files;
use crate::grammar::model::Fragment;
use crate::grammar::ast::BooleanLit;

#[test]
fn test_bool_lit() {
    let mut f: Files<String> = Files::new();
    let h = f.add("t", "true".to_string());
    let fr = Fragment::new(&f,h);
    match BooleanLit::parse(fr) {
        Ok((rem, bool_lit)) => {
            assert_eq!(rem.len(), 0);
            assert_eq!(bool_lit.frag.get_span(), fr.get_span());
            assert_eq!(bool_lit.inner, true);
        },
        Err(e) => {eprintln!("{:#?}", e); panic!();}
    }
}