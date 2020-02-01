use crate::grammar::ast::BinaryExpression;
use crate::grammar::model::Fragment;
use codespan::Files;

#[test]
fn test_binary_expression() {
    let mut f: Files<String> = Files::new();
    let h = f.add("t", "true".to_string());
    let fr = Fragment::new(&f, h);
    let (rem, bin_expr) = BinaryExpression::parse(fr).unwrap();
}
