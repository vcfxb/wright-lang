use crate::grammar::model::Fragment;
use crate::grammar::testing::TestingContext;
use nom::bytes::complete::take_while1;
use nom::error::ErrorKind;
use nom::IResult;
use nom::{Err, InputTakeAtPosition};
use std::ptr::eq as ptr_eq;

fn test_frag_str<'a, F1, F2>(s: &'a str, f1: F1, f2: F2)
where
    F1: for<'b> Fn(Fragment<'b>) -> IResult<Fragment<'b>, Fragment<'b>>,
    F2: Fn(&'a str) -> IResult<&'a str, &'a str>,
{
    let tcx = TestingContext::with(&[s]);
    let frag = tcx.get_fragment(0);
    let p1 = f1(frag.clone());
    let p2 = f2(s);
    match (p1, p2) {
        (Ok((rem1, out1)), Ok((rem2, out2))) => {
            assert_eq!(rem1.source(), rem2);
            assert_eq!(out1.source(), out2);
            assert_eq!(out1.len(), out2.len());
            assert_eq!(out1.source().len(), out1.len());
            assert!(ptr_eq(rem1.files(), frag.files()));
            assert_eq!(rem1.len(), rem2.len());
            assert_eq!(rem1.get_handle(), frag.get_handle());
        }
        (Result::Err(Err::Error((fr, ek1))), Result::Err(Err::Error((str, ek2)))) => {
            let fr: Fragment = fr;
            assert_eq!(ek1, ek2);
            assert_eq!(fr.get_handle(), frag.get_handle());
            assert!(ptr_eq(fr.files(), frag.files()));
            assert_eq!(fr.len(), str.len());
            assert_eq!(fr.source().len(), fr.len());
            assert_eq!(fr.source(), str);
        }
        (fr, s) => panic!(
            "Parser output mismatch!\nFrag parser:\n{:?}\nStr parser:\n{:?}",
            fr, s
        ),
    }
}

#[test]
fn test_take_while1() {
    test_frag_str(
        "",
        |i| take_while1(char::is_alphabetic)(i),
        take_while1(char::is_alphabetic),
    );
}

#[test]
fn test_split_at_position1_complete_empty() {
    let tcx = TestingContext::with(&[""]);
    let fr = tcx.get_fragment(0);
    let res: IResult<Fragment, Fragment> =
        fr.split_at_position1_complete(char::is_alphabetic, ErrorKind::TakeWhile1);
    assert!(res.is_err());
}

#[test]
fn test_eq() {
    let ctx = TestingContext::with(&[
        "abc",
        "abc",
    ]);
    assert_eq!(ctx.get_fragment(0), ctx.get_fragment(1));
}