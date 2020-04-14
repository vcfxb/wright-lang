use crate::grammar::ast::FuncCallExpression;
use crate::grammar::model::Fragment;
use crate::grammar::parsers::testing::setup;

fn test_call(s: &'static str, should_err: bool) {
    let (f, h) = setup(s);
    let fr = Fragment::new(&f, h);
    let r = FuncCallExpression::parse(fr);
    if should_err {
        assert!(r.is_err());
        r.map_err(|e| {
            e.map(|t| {
                let fr: Fragment = t.0;
                assert_eq!(fr.source(), s);
            })
        })
        .unwrap_err();
    } else {
        assert!(r.is_ok());
        let o = r.unwrap();
        assert_eq!(o.0.len(), 0);
        assert_eq!(o.1.frag.source(), s);
    }
}

#[test]
fn test_empty() {
    test_call("", true);
}

#[test]
fn test_underscore() {
    test_call("_()", true);
}

#[test]
fn test_true() {
    test_call("true()", false);
}

#[test]
fn test_calls() {
    ["abc()", "a_bc(1, \"abc\")", "n0(a, b, c)", "xd(e)"]
        .iter()
        .for_each(|s| test_call(s, false))
}
