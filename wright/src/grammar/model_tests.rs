use crate::grammar::model::Fragment;
use crate::grammar::parsers::testing::setup;
use codespan::Files;
use nom::bytes::complete::take_while1;
use nom::character::is_alphabetic;
use nom::Err;
use nom::IResult;
use std::fmt::Debug;
use std::mem::discriminant;
use std::ptr::eq as ptr_eq;

fn test_frag_str<'a, F1, F2>(s: &'a str, f1: F1, f2: F2)
where
    F1: for<'b> Fn(Fragment<'b>) -> IResult<Fragment<'b>, Fragment<'b>>,
    F2: Fn(&'a str) -> IResult<&'a str, &'a str>,
{
    let mut f: Files<String> = Files::new();
    let h = f.add("test", s.to_owned());
    let frag: Fragment = Fragment::new(&f, h);
    let p1 = f1(frag);
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
    let is_alpha = move |c: char| c.is_alphabetic();
    let fs = take_while1(is_alpha);
    test_frag_str("", |input: Fragment| take_while1(is_alpha)(input), fs);
}
