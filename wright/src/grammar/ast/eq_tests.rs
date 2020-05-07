use crate::grammar::ast::{eq::AstEq, NumLit};
use crate::grammar::parsers::testing::TestingContext;
use crate::grammar::ast::eq::ast_eq;

#[test]
fn test_ast_eq() {
    fn test_aeq<T: AstEq>(v: &Vec<T>) {
        assert!(ast_eq(&v[0], &v[1]));
        assert!(!ast_eq(&v[1], &v[2]));
    }

    let tcx = TestingContext::with(&["5", "5", "6"]);
    let nodes = tcx.run_parser_on_all(NumLit::parse)
        .iter()
        .map(|r| r.as_ref().unwrap())
        .map(|(_, node)| *node)
        .collect();

    test_aeq(&nodes);
    test_aeq(&nodes.iter().map(Box::new).collect());
}