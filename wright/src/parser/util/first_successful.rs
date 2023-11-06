//! The [`FirstSuccessful`] parse combinator takes a series of parsers and applies them in order until one is
//! successful.

// use crate::parser::state::ParserState;

// use super::NodeParserOption;

// /// Object that holds the series of parsers to test in order. The first one that produces
// /// an [`Ok`] will return, all others will be discarded.
// ///
// /// - Each item must implement [`Parse`] and
// ///     will be erased when added to this object.
// /// - Each item will be run on a fresh clone of the original [`Parser`]
// ///     so that the partial progress of one will not effect any others.
// /// - Errors from any of the contained/child parsers will be ignored.
// /// - If none of the parse functions are successful then an [`Err`] with no content will be returned.
// ///
// pub struct FirstSuccessful<T> {
//     /// All parse functions passed to this object should be erased.
//     pub parse_fns: Vec<Box<dyn for<'pf_s> Fn(ParserState<'pf_s>) -> NodeParserOption<'pf_s, T>>>
// }

// pub fn first_successful

// impl<T> FirstSuccessful<T> {
//     /// Create a new empty [`FirstSuccessful`] parse function.
//     pub fn new() -> Self {
//         Self { parse_fns: Vec::new() }
//     }

//     /// Builder style method to add a [`Parse`] function to this object at the end of the list of parse functions
//     /// to try.
//     pub fn builder_push<E>(mut self, p: impl Parse<Success = T, Error = E>) -> Self {
//         self.parse_fns.push(p.discard_error());
//         self
//     }

//     /// Push an additional parse function to the end of this objects list of [`Parse`] functions to try.
//     pub fn push<E>(&mut self, p: impl Parse<Success = T, Error = E>) {
//         self.parse_fns.push(p.discard_error())
//     }
// }

// impl<T> Parse for FirstSuccessful<T> {
//     type Success = T;

//     type Error = ();

//     fn parse<'src>(&self, parser: &mut Parser<'src>) -> Result<Self::Success, ()> {
//         // Iterate in order over child parsers.
//         for parse_fn in self.parse_fns.iter() {
//             // Clone the parser state so that we have a clean parser every time.
//             let mut clean_parser = parser.clone();
//             // Try the given parse function.
//             let parse_result = parse_fn.parse(&mut clean_parser);
//             // Check and return
//             if parse_result.is_ok() {
//                 // Update the lexer on the parent parser.
//                 parser.lexer = clean_parser.lexer;
//                 return parse_result;
//             }
//         }

//         Err(())
//     }
// }
