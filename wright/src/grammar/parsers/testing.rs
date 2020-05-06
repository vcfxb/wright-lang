use codespan::{FileId, Files};
use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::{AstEq};

/// Setup function to create a Files object and return a handle with it.
pub fn setup(src: &'static str) -> (Files<String>, FileId) {
    let mut f: Files<String> = Files::new();
    let id = f.add("test", src.to_string());
    (f, id)
}

/// Test the ast equality of the trees produced by a parser over two inputs.
pub fn test_ast_eq<T: AstEq>(
    parser: fn(Fragment) -> IResult<Fragment, T>,
    src1: &'static str,
    src2: &'static str
) {
    let mut f: Files<String> = Files::new();
    let h1 = f.add("src1", src1.to_owned());
    let h2 = f.add("src2", src2.to_owned());
    let fr1 = Fragment::new(&f, h1);
    let fr2 = Fragment::new(&f, h2);
    let ast1 = parser(fr1).unwrap().1;
    let ast2 = parser(fr2).unwrap().1;
    assert!(AstEq::ast_eq(&ast1, &ast2));
}

/// Run a specific test on a given parser.
fn run_test<'a, O, T>(
    parser: fn(Fragment) -> IResult<Fragment, O>,
    src: &'static str,
    should_fail: bool,
    remaining: Option<&'static str>,
    output_test: T
)
where
    T: FnOnce(O) -> bool,
{
    let mut f: Files<String> = Files::new();
    let h: FileId = f.add("test", src.to_string());
    let fr = Fragment::new(&f, h);
    let res = parser(fr);
    if should_fail {
        assert!(res.is_err());
    } else {
        assert!(res.is_ok());
        let (rem, node) = res.unwrap();
        assert_eq!(rem.source(), remaining.unwrap());
        assert!(output_test(node.into()));
    }
}

/// Run a test on a parser using a certain input and expect it to succeed.
#[inline]
pub fn test_should_succeed<O, T>(
    parser: fn(Fragment) -> IResult<Fragment, O>,
    src: &'static str,
    remaining: &'static str,
    output_test: T
)
where T: FnOnce(O) -> bool
{
    run_test(parser, src, false, Some(remaining), output_test)
}

// /// Run a test on a parser using a given input and expect it to fail.
// #[inline]
// pub fn test_should_fail<T>(
//     parser: fn(Fragment) -> IResult<Fragment, T>,
//     src: &'static str,
// ) {run_test(parser, src, true, None, |_| true)}