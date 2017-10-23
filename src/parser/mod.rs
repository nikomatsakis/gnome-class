use ast;
use errors::*;
//use lalrpop_util::ParseError;
//use proc_macro2::{TokenStream, TokenTree, TokenNode, Term, Delimiter};
//use tok::{self, Tok};

//mod tokens;

//mod grammar;

pub fn parse_program(input: &str) -> Result<ast::Program> {
    unimplemented!()
    /*
    let tokenizer = tok::Tokenizer::new(input, 0);
    match grammar::parse_Program(tokenizer) {
        Ok(p) => Ok(p),
        Err(e) => bail!("parse error: {:?}", e),
    }
     */
}

/*
type ParseResult<'input, T> =
    ::std::result::Result<T, ParseError<usize, Tok<'input>, Error>>;
fn parse_members(input: &str,
                 offset: usize)
                 -> ParseResult<Vec<ast::Member>> {
    assert_eq!(input.chars().next(), Some('{'));
    assert!(input.len() >= 2); // at least `{}`
    let len = input.len();
    let contents = &input[1..len - 1];
    let tokenizer = tok::Tokenizer::new(contents, offset + 1);
    grammar::parse_Members(tokenizer)
}

fn parse_var_tys(input: &str,
                 offset: usize)
                 -> ParseResult<Vec<ast::VarTy>> {
    assert_eq!(input.chars().next(), Some('{'));
    assert!(input.len() >= 2); // at least `{}`
    let len = input.len();
    let contents = &input[1..len - 1];
    let tokenizer = tok::Tokenizer::new(contents, offset + 1);
    grammar::parse_VarTys(tokenizer)
}
 */

use synom::delimited::Delimited;
use synom::{Synom, Cursor, PResult, parse_error, tokens};
use syn::{Block, DeriveInput, FunctionRetTy, Ident, Path};

// class Foo [: SuperClass [, ImplementsIface]*] {
// }
impl Synom for ast::Class {
    named!(parse -> Self, do_parse!(
        call!(keyword("class"))                 >>
        name: syn!(Ident)                       >>
        extends: option!(do_parse!(
            syn!(tokens::Colon)                 >>
            superclass: syn!(Path)              >>
            // FIXME: interfaces
            (superclass)))                      >>
        block: syn!(Block)                      >>
        (ast::Class {
            name:    name,
            extends: extends,
            members: Vec::new() // FIXME
        })
    ));
}

// struct Foo {
//     <fields>*
// }
//
// FIXME: we assume that the DeriveInput will have a Body -> Struct ->
// data: VariantData::Struct i.e. *not* a Tuple.  Would anyone really
// want to have a private tuple struct for a GObject's data?
//
// If the answer is "no", maybe we should peek!() that there is a Brace there.
impl Synom for ast::PrivateStruct {
    named!(parse -> Self, do_parse!(
        peek!(syn!(tokens::Struct))     >>
        derive_input: syn!(DeriveInput) >>
        (ast::PrivateStruct {
            derive_input: derive_input
        })
    ));
}

// private_init () -> PrivateStructName {
//     ...
// }
//
// This is the initialization function for the user's PrivateStruct.
impl Synom for ast::PrivateInit {
    named!(parse -> Self, do_parse!(
        call!(keyword("private_init"))                >>
        inputs: parens!(Delimited::parse_terminated)  >>
        output: syn!(FunctionRetTy)                   >>
        block_and_braces: braces!(call!(Block::parse_within))    >>
        (ast::PrivateInit {
            inputs: inputs.0,
            output: output,
            block:  Block {
                brace_token: block_and_braces.1,
                stmts: block_and_braces.0,
            },
        })
    ));
}

/// Creates a parsing function for use with synom's call!().  For
/// example, if you need to parse a keyword "foo" as part of a bigger
/// parser, you could do this:
///
/// ```ignore
/// call!(keyword("foo"))
/// ```
fn keyword<'a>(name: &'static str) -> impl Fn(Cursor<'a>) -> PResult<()> {
    move |input: Cursor<'a>| {
        if let Some((rest, _, s)) = input.word() {
            if s.as_str() == name {
                return Ok((rest, ()));
            }
        }
        parse_error() // FIXME: use a meaningful error message when synom allows for it
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::{TokenStream};
    use quote;
    use quote::ToTokens;
    use syn::{Body, BodyStruct, Ty, VariantData};
    use synom::{SynomBuffer};
    use synom::delimited::Element;

    #[test]
    fn parses_class_name() {
        let raw = "class Foo {}";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        // We can't use
        //
        //   let class: ast::Class = syn::parse(token_stream).unwrap().1;
        //
        // because syn::parse() takes a proc_macro::TokenStream, not a
        // proc_macro2::TokenStream.
        //
        // So, we'll do the conversion to a Cursor by hand.

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();
        let class: ast::Class = ast::Class::parse(cursor).unwrap().1;
        assert_eq!(class.name.as_ref(), "Foo");
        assert!(class.extends.is_none());
    }

    #[test]
    fn parses_class_name_and_superclass() {
        let raw = "class Foo: Bar {}";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();
        let class: ast::Class = ast::Class::parse(cursor).unwrap().1;
        assert_eq!(class.name.as_ref(), "Foo");

        let mut path_tokens = quote::Tokens::new();
        class.extends.unwrap().to_tokens(&mut path_tokens);
        assert_eq!(path_tokens.to_string(), "Bar");
    }

    #[test]
    fn parses_private_struct() {
        let raw = "struct FooPrivate {
                       foo: u32,
                       bar: String
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();

        let private_struct = ast::PrivateStruct::parse(cursor).unwrap().1;

        assert_eq!(private_struct.derive_input.ident.as_ref(), "FooPrivate");

        match private_struct.derive_input.body {
            Body::Struct(BodyStruct {
                data: VariantData::Struct(ref delimited, ..),
                .. })
                => {
                    let mut iter = delimited.iter();

                    let element = iter.next().unwrap();
                    if let Element::Delimited(ref field, ..) = element {
                        assert_eq!(field.ident.unwrap(), "foo");

                        if let Ty::Path(ref typath) = field.ty {
                            let mut path_tokens = quote::Tokens::new();
                            typath.path.to_tokens(&mut path_tokens);
                            assert_eq!(path_tokens.to_string(), "u32");
                        } else {
                            unreachable!();
                        }
                    } else {
                        unreachable!();
                    }

                    let element = iter.next().unwrap();
                    if let Element::End(ref field) = element {
                        assert_eq!(field.ident.unwrap(), "bar");

                        if let Ty::Path(ref typath) = field.ty {
                            let mut path_tokens = quote::Tokens::new();
                            typath.path.to_tokens(&mut path_tokens);
                            assert_eq!(path_tokens.to_string(), "String");
                        } else {
                            unreachable!();
                        }
                    } else {
                        unreachable!();
                    }
                },

            _ => unreachable!()
        }
    }
}
