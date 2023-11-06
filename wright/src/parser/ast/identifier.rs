//! Identifiers in wright source code.

use super::metadata::AstNodeMeta;

/// An identifier in the source code being parsed.
#[derive(Debug, Clone, Copy)]
pub struct Identifier<'src> {
    /// An identifier is just a string in source code so we use a single metadata here
    /// and pass on the indetifier from the matching source.
    pub inner: AstNodeMeta<'src>,
}

// impl<'src> Parser<'src> {
//     /// Parse an identifier in source code or error.
//     /// 
//     /// If the parse is unsuccessful, return an error and do not update the parser state. 
//     pub fn parse_identifier(&mut self) -> ParserResult<Identifier<'src>> {
//         // Clone the lexer to try to parse an identifier.
//         let mut lexer = self.lexer.clone();

//         match lexer.next() {
//             Some(IndexedToken {
//                 token:
//                     Token {
//                         variant: TokenTy::Identifier,
//                         ..
//                     },
//                 ..
//             }) => Ok(Identifier {
//                 inner: self.update_lexer(lexer),
//             }),

//             _ => Err(ParserError {
//                 byte_range: self.lexer.index..lexer.index,
//                 ty: ParserErrorVariant::Expected("identifier"),
//             }),
//         }
//     }
// }
