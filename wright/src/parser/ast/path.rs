//! A fully qualified sympbol path in wright source code.
//!
//! Path items are separated using `::` similar to rust.

use crate::parser::{Parser, ParserResult, lexer::{IndexedToken, tokens::{Token, TokenTy}}, ParserError, ParserErrorVariant};

use super::{identifier::Identifier, metadata::AstNodeMeta};

/// A double-colon seperated path to a module, type, or function in Wright source code.
///
/// Note that this can be only a single identifier in length, signaling a path/identifier that's in current scope.
#[derive(Debug)]
pub struct Path<'src> {
    /// Node metadata.
    pub meta: AstNodeMeta<'src>,
    /// The first part of the path, read left to right.
    pub head: Identifier<'src>,
    /// The rest of the path.
    pub tail: Option<Box<Path<'src>>>,
}

impl<'src> Parser<'src> {
    /// Try to parse a path from the inner lexer iterator. 
    /// 
    /// If a [`Path`] could not be parsed, return an error and leave the parser unmodified. 
    pub fn parse_path(&mut self) -> ParserResult<Path<'src>> {
        // Clone the lexer -- leave this as the unmodified lexer to replace if the parse fails. 
        let initial_lexer = self.lexer.clone();

        // Try parsing an identifier. 
        let head = self.parse_identifier()
            // If we parse no head, swap out the error type and return.
            .map_err(|ParserError { byte_range, ..} | ParserError {
                byte_range,
                ty: ParserErrorVariant::Expected("fully-qualified symbol reference (path) or identifier")
            })?;
        
        // Try parsing the rest of the path. Pass into box/heap allocation on success. 
        let tail = self.parse_path_tail().map(Box::new);

        // Return the parsed path. 
        Ok(Path { 
            // Make the node metadata. 
            meta: AstNodeMeta { 
                file_map: self.file_map, 
                file_id: self.file_id, 
                index: initial_lexer.index, 
                matching_source: &self.source[initial_lexer.index..self.lexer.index] 
            }, 

            head, 
            tail 
        })
    }

    /// Try parsing the tail of a path. This gets called recursively in path parsing.
    fn parse_path_tail(&mut self) -> Option<Path<'src>> {
        // Clone the initial lexer. 
        let initial_lexer = self.lexer.clone();
        // Allow (ignore) whitespace between initial head and first double colon.
        self.ignore_whitespace();
        // First parse through a single double colon.
        if let Some(IndexedToken { token: Token { variant: TokenTy::ColonColon, .. }, .. }) = self.lexer.next() {
            // Allow a whitespace between the double colon and head identifier. 
            self.ignore_whitespace();
            
            // Try to parse the head identifier. 
            let head = match self.parse_identifier().ok() {
                // On success, keep the head, continue to parse the tail. 
                Some(head) => head,

                // No head -- role back to the initial parser (pre-double colon and whitespaces) and return none.
                None =>  {
                    self.lexer = initial_lexer;
                    return None;
                }
            };

            // Then try to parse the tail. Pass into box on success. 
            let tail = self.parse_path_tail().map(Box::new);

            // Return the parsed path.
            Some(Path { 
                meta: AstNodeMeta { 
                    file_map: self.file_map, 
                    file_id: self.file_id, 
                    index: initial_lexer.index, 
                    matching_source: &self.source[initial_lexer.index..self.lexer.index]
                }, 

                head, 
                tail 
            })

        } else {
            // No tail parsed -- reset the ignored whitespace, and return None
            self.lexer = initial_lexer;
            None
        }
    }
}
