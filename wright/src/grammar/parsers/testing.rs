use codespan::{FileId, Files};
use crate::grammar::model::Fragment;
use nom::IResult;
use crate::grammar::ast::{AstEq};

/// Parser wrapper trait used in testing. Should be implemented on all wright
/// parsers.
pub trait Parser<'s> {
    /// Output of this parser.
    type Output;
    /// Call this parser.
    fn call(&self, input: Fragment<'s>) -> IResult<Fragment<'s>, Self::Output>;
}

impl<'s, F, T> Parser<'s> for F
where F: Fn(Fragment<'s>) -> IResult<Fragment<'s>, T> {
    type Output = T;
    fn call(&self, input: Fragment<'s>) -> IResult<Fragment<'s>, T> {
        self(input)
    }
}

// impl <'a, F> Parser<'a> for F
// where F: for<'b> Parser<'b> {
//     type Output = <F as Parser<'a>>::Output;
//     fn call(&self, input: Fragment<'a>) -> IResult<Fragment, Self::Output> {
//         self(input)
//     }
// }


/// Setup function to create a Files object and return a handle with it.
pub fn setup(src: &'static str) -> (Files<String>, FileId) {
    let mut f: Files<String> = Files::new();
    let id = f.add("test", src.to_string());
    (f, id)
}

/// Test the ast equality of the trees produced by a parser over two inputs.
pub fn test_ast_eq<P>(
    parser: P,
    src1: &'static str,
    src2: &'static str
)
where
    P: for<'a> Parser<'a>,
    for<'b> <P as Parser<'b>>::Output: AstEq,
{
    let mut f: Files<String> = Files::new();
    let h1 = f.add("src1", src1.to_owned());
    let h2 = f.add("src2", src2.to_owned());
    let fr1 = Fragment::new(&f, h1);
    let fr2 = Fragment::new(&f, h2);
    let ast1 = parser.call(fr1).unwrap().1;
    let ast2 = parser.call(fr2).unwrap().1;
    assert!(AstEq::ast_eq(&ast1, &ast2));
}

// ==== ALL OF THE BELOW CODE IS WAITING ON A SOLUTION to
// ==== https://github.com/rust-lang/rust/issues/71955
// ==== or the implementation of HKTs (higher kinded types.)
// ==== until then this code should remain commented out.

// /// Run a specific test on a given parser.
// pub(crate) fn run_test<P, T>(
//     parser: P,
//     src: &'static str,
//     should_fail: bool,
//     remaining: Option<&'static str>,
//     output_test: T
// )
// where
//     P: for<'a> Parser<'a>,
//     T: FnOnce(&<P as Parser>::Output) -> bool,
// {
//     let mut f: Files<String> = Files::new();
//     let h: FileId = f.add("test", src.to_string());
//     let fr = Fragment::new(&f, h);
//     let res = parser.call(fr);
//     if should_fail {
//         assert!(res.is_err());
//     } else {
//         assert!(res.is_ok());
//         let (rem, node) = res.unwrap();
//         assert_eq!(rem.source(), remaining.unwrap());
//         assert!(output_test(&node));
//     }
// }

// /// Run a test on a parser using a certain input and expect it to succeed.
// #[inline]
// pub fn test_should_succeed<O, T>(
//     parser: for<'s> fn(Fragment<'s>) -> IResult<Fragment<'s>, O>,
//     src: &'static str,
//     remaining: &'static str,
//     output_test: T
// )
// where T: FnOnce(O) -> bool
// {
//     run_test(parser, src, false, Some(remaining), output_test)
// }

// /// Run a test on a parser using a given input and expect it to fail.
// #[inline]
// pub fn test_should_fail<T>(
//     parser: fn(Fragment) -> IResult<Fragment, T>,
//     src: &'static str,
// ) {run_test(parser, src, true, None, |_| true)}