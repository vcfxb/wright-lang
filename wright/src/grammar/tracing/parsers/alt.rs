use nom::error::{ErrorKind};
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;
use crate::grammar::model::WrightInput;
use nom::Err;


/// A trait used to replace
/// [nom's trait of the same name](https://docs.rs/nom/5.1.1/nom/branch/trait.Alt.html).
pub trait Alt<I, O, E> {

    /// Choose the appropriate parser, in this case
    /// the first one that succeeds.
    fn choice(&self, input: I) -> IResult<I, O, E>;
}

macro_rules! impl_alt {
    ($first:ident $second:ident $($rest:ident)*) => {
        impl_alt!(inner1 $first $second; $($rest)*);
    };
    (inner1 $($current:ident)+; $head:ident $($rest:ident)+) => {
        impl_alt_inner!( $($current)+ );
        impl_alt!(inner1 $($current)+ $head; $($rest)+);
    };
    (inner1 $($current:ident)+; $head:ident) => {
        impl_alt_inner!( $($current)+ );
        impl_alt_inner!( $($current)+ $head);
    };

}

macro_rules! impl_alt_inner {
    ($($id:ident)+) => {
        #[allow(bad_style)]
        impl<Input: WrightInput, Output, $($id: Fn(Input) -> IResult<Input, Output>),+ >
            Alt<Input, Output, (Input, ErrorKind)> for ( $($id),+ )
        {
            fn choice(&self, input: Input) -> IResult<Input, Output> {
                let mut source = input;
                let ( $($id),+ ) = self;
                $(
                    match ($id)(source) {
                        Result::Err(Err::Error((s, _))) => source = s,
                        other => return other,
                    }
                )+
                IResult::Err(Err::Error((source, ErrorKind::Alt)))
            }
        }
    };
}

impl_alt!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);


/// A traced version of nom's
/// [`alt`](https://docs.rs/nom/5.1.1/nom/branch/fn.alt.html)
/// combinator.
pub fn alt<I, O, List>(l: List) -> impl Fn(I) -> IResult<I, O, (I, ErrorKind)>
where
    I: Clone + OptionallyTraceable,
    List: Alt<I, O, (I, ErrorKind)>
{
    let trace = "alt";
    move |input: I| {
        let input= input.trace_start_clone(trace);
        let res= l.choice(input);
        trace_result(trace, res)
    }
}