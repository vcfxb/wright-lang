use crate::grammar::testing::TestingContext;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::parsers::{alt, tag};
use nom::multi::many1;

#[test]
fn test_alt_simple() {
    let ctx = TestingContext::with(&["abc", "123", "def"]);

    let fr0 = ctx.get_fragment(0);
    let fr1 = ctx.get_fragment(1);
    let fr2 = ctx.get_fragment(2);

    let p = alt((tag("abc"), tag("123"), tag("def")));

    p(fr0).unwrap().0.get_trace().unwrap().print().unwrap();
    print!("\n");
    p(fr1).unwrap().0.get_trace().unwrap().print().unwrap();
    print!("\n");
    p(fr2).unwrap().0.get_trace().unwrap().print().unwrap();
}

#[test]
fn test_alt_multi() {
    let p = |i| many1(alt((tag("abc"), tag("123"))))(i);
    let ctx = TestingContext::with(&["abc", "abc123", "abcabc123abc", "abca23d"]);

    for i in 0..=2 {
        ctx.test_output(p, i, |(rem, v)| {
            rem.get_trace().unwrap().print().unwrap();
            assert_eq!(rem.len(), 0);
        });
    }
}
