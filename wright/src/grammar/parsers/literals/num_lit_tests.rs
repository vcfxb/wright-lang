use crate::grammar::model::Fragment;
use codespan::Files;
use crate::grammar::ast::NumLit;

#[test]
fn dec() {
    let mut files: Files<String> = Files::new();
    let h = files.add("dec", "1000".to_owned());
    let frag = Fragment::new(&files, h);
    let res = NumLit::parse(frag);
    if let Ok((remaining, node)) = res {
        assert_eq!(remaining.len(), 0);
        assert_eq!(node.inner, 1000);
        assert_eq!(node.frag.start(), frag.start());
        assert_eq!(node.frag.end(), frag.end());
    } else {
        eprintln!("{:#?}", res);
        res.unwrap();
    }
}