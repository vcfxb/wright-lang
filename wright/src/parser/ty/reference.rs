//! Referenced types. Types that are defined by users or in the standard library.

use crate::{
    ast::ty::{ReferenceTy, Type},
    lexer::token::TokenTy,
    parser::{
        Parser,
        error::{ParserError, ParserErrorKind},
    },
    source_tracking::fragment::Fragment,
};

impl ReferenceTy {
    /// Attempt to parse a reference type signature, i.e. `@u64`.
    ///
    /// This will leave the parser unmodified and return an error if it doesn't match the `@` symbol, however
    /// if it does match the `@` symbol it will advance the parser and then may still return an error if the
    /// `@` symbol is not followed by a type signature.
    pub fn parse(parser: &mut Parser) -> Result<Self, ParserError> {
        let Some(at_symbol) = parser.next_if_is(TokenTy::At) else {
            return Err(ParserErrorKind::ExpectedReferenceTypeSignature
                .at(parser.peek_fragment_or_rest_cloned()));
        };

        parser.consume_optional_whitespace();

        let referenced_type = Type::parse(parser)?;

        Ok(ReferenceTy {
            matching_source: Fragment::cover(
                &at_symbol.fragment,
                referenced_type.matching_source(),
            ),
            target_ty: Box::new(referenced_type),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ty::{AtomicTyVariant, ReferenceTy},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_reference_to_atomic() {
        let mut parser = Parser::new(Lexer::new_test("@u64"));
        let result = ReferenceTy::parse(&mut parser).unwrap();

        assert_eq!(result.matching_source.as_str(), "@u64");
        assert_eq!(result.target_ty.downcast_primitive().unwrap().variant, AtomicTyVariant::U64);
    }

    #[test]
    fn test_reference_to_a_reference_to_atomic() {
        let mut parser = Parser::new(Lexer::new_test("@@u64"));
        let result = ReferenceTy::parse(&mut parser).unwrap();

        assert_eq!(result.matching_source.as_str(), "@@u64");
        assert!(result.target_ty.downcast_reference().is_some());
    }

    #[test]
    fn test_u8_ref() {
        let mut parser = Parser::new(Lexer::new_test("@u8"));
        let ref_ty = ReferenceTy::parse(&mut parser).unwrap();
        assert_eq!(ref_ty.matching_source.len(), 3);
        assert_eq!(ref_ty.target_ty.downcast_primitive().unwrap().variant, AtomicTyVariant::U8);
    }

    #[test]
    fn test_nested_ref() {
        let mut parser = Parser::new(Lexer::new_test("@@u8"));
        let ref_ty = ReferenceTy::parse(&mut parser).unwrap();
        assert_eq!(ref_ty.matching_source.len(), 4);
        let inner_ref = ref_ty.target_ty.downcast_reference().unwrap();
        assert_eq!(inner_ref.matching_source.len(), 3);
        assert_eq!(inner_ref.target_ty.downcast_primitive().unwrap().variant, AtomicTyVariant::U8);
    }
}
