use crate::{
    ast::{
        path::Path,
        ty::{NamedTy, Type},
    },
    lexer::token::TokenTy,
    parser::{
        Parser,
        error::{ParserError, ParserErrorKind},
    },
    source_tracking::fragment::Fragment,
};

impl NamedTy {
    /// Parse a named type from source code.
    pub fn parse(parser: &mut Parser) -> Result<NamedTy, ParserError> {
        // If we don't start with a path, there's no type here.
        let path = Path::parse(parser)?;

        let mut generic_tys = Vec::new();

        // If the next non-whitespace is an angle bracket, consume it and then read through generics until it's closed.
        if parser.matches_ignore_whitespace(&[TokenTy::Lt]) {
            // Chew through any whitespace.
            parser.consume_optional_whitespace();

            // Skip the `<`
            parser.advance(1);

            loop {
                // Parse a generic type
                let t = Type::parse(parser)?;

                // Push the type to the list of generics.
                generic_tys.push(t);

                // Check if it was the last one.
                const ENDING_SEQUENCES: &[&[TokenTy]] =
                    &[&[TokenTy::Comma, TokenTy::Gt], &[TokenTy::Gt]];

                for seq in ENDING_SEQUENCES {
                    if parser.matches_ignore_whitespace(seq) {
                        // Chew through remaining tokens
                        for _ in 0..(seq.len() - 1) {
                            parser.consume_optional_whitespace();
                            parser.advance(1);
                        }

                        parser.consume_optional_whitespace();

                        // SAFETY: We have confirmed that there's a token here, and it's not an unknown.
                        let last_token =
                            unsafe { parser.next_token().unwrap_unchecked().unwrap_unchecked() };

                        // Compute the fragment to cover the whole thing.
                        let matching_source =
                            Fragment::cover(&path.full_path, &last_token.fragment);

                        return Ok(NamedTy {
                            matching_source,
                            name: path,
                            generic_tys,
                            // generic_consts: (),
                        });
                    }
                }

                // If it wasn't the last one (we get here):
                // Try to parse a comma and then another generic type or fail.
                if !parser.matches_ignore_whitespace(&[TokenTy::Comma]) {
                    let fragment = parser.peek_fragment_or_rest_cloned();
                    return Err(ParserErrorKind::UnterminatedGenericTypeSignature.at(fragment));
                }

                // Chew through whitespace and comma
                parser.consume_optional_whitespace();
                parser.advance(1);
                parser.consume_optional_whitespace();

                // Loop back and consume next type.
            }
        }

        // If parser does not match the angle bracket for generics, just return a named
        // type without them
        Ok(NamedTy {
            matching_source: path.full_path.clone(),
            name: path,
            generic_tys,
            // generic_consts: (),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ty::{AtomicTyVariant, NamedTy},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_basic_named_type() {
        let mut parser = Parser::new(Lexer::new_test("MyType"));
        let named_ty = NamedTy::parse(&mut parser).expect("Failed to parse named type");
        assert_eq!(named_ty.name.full_path.as_str(), "MyType");
        assert_eq!(named_ty.name.full_path, named_ty.matching_source);
        assert!(named_ty.generic_tys.is_empty());
    }

    #[test]
    fn test_named_type_with_generics() {
        let mut parser = Parser::new(Lexer::new_test("MyType<i8, @@@i8, OtherType<ThirdType>, >"));
        let named_ty = NamedTy::parse(&mut parser).unwrap();
        assert_eq!(named_ty.name.full_path.as_str(), "MyType");
        assert_eq!(
            named_ty.generic_tys[0]
                .downcast_primitive()
                .unwrap()
                .variant,
            AtomicTyVariant::I8
        );
        assert_eq!(named_ty.generic_tys[1].matching_source().as_str(), "@@@i8");
        assert_eq!(
            named_ty.generic_tys[2]
                .downcast_named()
                .unwrap()
                .name
                .full_path
                .as_str(),
            "OtherType"
        );
        assert_eq!(
            named_ty.generic_tys[2]
                .downcast_named()
                .unwrap()
                .matching_source
                .as_str(),
            "OtherType<ThirdType>"
        );
    }
}
