use wright::grammar::parser::ignored as ign_parser;
use wright::grammar::parser::integer_literal;
use wright::grammar::parser::identifier;

#[test]
fn ignored() {
    let tests = [
        " ",
        "",
        "# comment \n",
        "#* m comment \n *#",
        "#* a #* nested *# comment *#",
        "#* nested #* a *##*b*#c \n*#",
        "#* double #* nested #* comment *# a *#a *#c",
    ];
    for s in tests.iter() {
        eprintln!("{}", s);
        ign_parser().parse(s.as_bytes()).unwrap();
    }
}

#[test]
fn integers() {
    let tests = [
        "0",
        "1000",
        "0xFFfa1982001",
        "0b010100010",
        "4",
    ];
    for s in tests.iter() {
        eprintln!("{}", s);
        integer_literal().parse(s.as_bytes()).unwrap();
    }
}

#[test]
fn ident() {
    fn ok(s: &'static str) {
        assert!(identifier().parse(s.as_bytes()).is_ok())
    }
    fn err(s: &'static str) {
        assert!(identifier().parse(s.as_bytes()).is_err())
    }
    ok("_identifier");
    err("20");
    ok("test");
    err("true");
    err("_");
}