// use wright::parser::lexer::{
//     tokens::{Token, TokenTy},
//     Lexer,
// };

// /// Test unterminated string literal.
// #[test]
// fn unterminated_string_literal() {
//     let tokens: Vec<Token> = Lexer::new(r#""this string is not closed"#).collect();
//     assert_eq!(tokens.len(), 1);
//     assert_eq!(
//         tokens[0].variant,
//         TokenTy::StringLit {
//             is_format: false,
//             is_terminated: false
//         }
//     );
// }

// /// Test string literal with escaped terminal.
// #[test]
// fn string_with_escape() {
//     let tokens: Vec<Token> =
//         Lexer::new(r#" "this string has an escaped terminator \" " "#).collect();
//     assert_eq!(tokens.len(), 3);
//     assert_eq!(
//         tokens[1].variant,
//         TokenTy::StringLit {
//             is_format: false,
//             is_terminated: true
//         }
//     );
// }
