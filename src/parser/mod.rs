use proc_macro::TokenStream;
use syn;

use ast;
use errors::*;
//use lalrpop_util::ParseError;
//use proc_macro2::{TokenStream, TokenTree, TokenNode, Term, Delimiter};

//mod grammar;

pub fn parse_program(token_stream: TokenStream) -> Result<ast::Program> {
    syn::parse(token_stream).map_err(|e| e.into())
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
use syn::{Block, DeriveInput, FunctionRetTy, Ident, ImplItem, Path};

impl Synom for ast::Program {
    named!(parse -> Self, do_parse!(
        items: many0!(syn!(ast::Item)) >>
        (ast::Program {
            items: items
        })
    ));
}

impl Synom for ast::Item {
    named!(parse -> Self, alt!(
        syn!(ast::Class) => { |x| ast::Item::Class(x) }
        |
        syn!(ast::Impl) => { |x| ast::Item::Impl(x) }
    ));
}

// class Foo [: SuperClass [, ImplementsIface]*] {
//     struct FooPrivate {
//         ...
//     }
//
//     private_init() -> FooPrivate {
//         ...
//     }
// }
impl Synom for ast::Class {
    named!(parse -> Self, do_parse!(
        call!(keyword("class"))                                  >>
        name: syn!(Ident)                                        >>
        extends: option!(do_parse!(
            syn!(tokens::Colon)                                  >>
            superclass: syn!(Path)                               >>
            // FIXME: interfaces
            (superclass)))                                       >>
        items_and_braces: braces!(many0!(syn!(ast::ClassItem)))  >>
        (ast::Class {
            name:    name,
            extends: extends,
            items:   items_and_braces.0
        })
    ));
}

impl Synom for ast::ClassItem {
    named!(parse -> Self, alt!(
        syn!(ast::InstancePrivateItem) => { |x| ast::ClassItem::InstancePrivate(x) }
        |
        syn!(ast::PrivateStruct) => { |x| ast::ClassItem::PrivateStruct(x) }
        |
        syn!(ast::PrivateInit) => { |x| ast::ClassItem::PrivateInit(x) }
    ));
}

impl Synom for ast::InstancePrivateItem {
    named!(parse -> Self, do_parse!(
        type_: syn!(tokens::Type)         >>
        call!(keyword("InstancePrivate")) >>
        eq: syn!(tokens::Eq)              >>
        path: syn!(Path)                  >>
        semi: syn!(tokens::Semi)          >>
        (ast::InstancePrivateItem {
            type_token: type_,
            eq_token:   eq,
            path:       path,
            semi_token: semi
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

impl Synom for ast::Impl {
    named!(parse -> Self, do_parse!(
        impl_: syn!(tokens::Impl) >>
        trait_: alt!(
            do_parse!(
                path: syn!(Path) >>
                for_: syn!(tokens::For) >>
                (Some((path, for_)))
            )
            |
            epsilon!() => { |_| None }
        ) >>
        self_path: syn!(Path) >>
        body: braces!(many0!(syn!(ImplItem))) >>
        (ast::Impl {
            impl_token: impl_,
            trait_: trait_,
            self_path: self_path,
            brace_token: body.1,
            items: body.0
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
    use quote;
    use quote::ToTokens;
    use syn::{Body, BodyStruct, Ty, VariantData, TyPath, parse_str};
    use synom::delimited::Element;

    fn assert_tokens_equal<T: ToTokens>(x: &T, s: &str) {
        let mut tokens = quote::Tokens::new();
        x.to_tokens(&mut tokens);
        assert_eq!(tokens.to_string(), s);
    }

    #[test]
    fn parses_class_with_no_superclass() {
        let raw = "class Foo {}";
        let class = parse_str::<ast::Class>(raw).unwrap();

        assert_eq!(class.name.as_ref(), "Foo");
        assert!(class.extends.is_none());
    }

    #[test]
    fn parses_class_with_superclass() {
        let raw = "class Foo: Bar {}";
        let class = parse_str::<ast::Class>(raw).unwrap();

        assert_eq!(class.name.as_ref(), "Foo");
        assert_tokens_equal(&class.extends, "Bar");
    }

    #[test]
    fn parses_instance_private_item() {
        let raw = "type InstancePrivate = FooPrivate;";
        let item = parse_str::<ast::ClassItem>(raw).unwrap();

        if let ast::ClassItem::InstancePrivate(item) = item {
            assert_tokens_equal(&item.path, "FooPrivate");
        } else {
            unreachable!();
        }
    }

    #[test]
    fn parses_class_item() {
        let raw = "class Foo {}";
        let item = parse_str::<ast::Item>(raw).unwrap();

        if let ast::Item::Class(class) = item {
            assert_eq!(class.name.as_ref(), "Foo");
            assert!(class.extends.is_none());
        } else {
            unreachable!();
        }
    }

    #[test]
    fn parses_private_struct() {
        let raw = "struct FooPrivate {
                       foo: u32,
                       bar: String
                   }";
        let private_struct = parse_str::<ast::PrivateStruct>(raw).unwrap();

        assert_eq!(private_struct.name_as_ref(), "FooPrivate");

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
                            assert_tokens_equal(typath, "u32");
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
                            assert_tokens_equal(typath, "String");
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

    #[test]
    fn parses_private_init() {
        let raw = "private_init () -> FooPrivate {
                       FooPrivate {
                           foo: 42,
                           bar: \"hello\".to_string()
                       }
                   }";
        let private_init = parse_str::<ast::PrivateInit>(raw).unwrap();

        assert!(private_init.inputs.is_empty());

        match private_init.output {
            FunctionRetTy::Ty(Ty::Path(TyPath { ref path, .. }), _) => {
                assert_tokens_equal(path, "FooPrivate");
            },

            _ => unreachable!()
        }
    }

    #[test]
    fn parses_class_items() {
        let raw = "struct FooPrivate {
                       foo: u32,
                       bar: String
                   }";
        let item = parse_str::<ast::ClassItem>(raw).unwrap();

        match item {
            ast::ClassItem::PrivateStruct(_) => (),
            _ => unreachable!()
        };

        let raw = "private_init () -> FooPrivate {
                       FooPrivate {
                           foo: 42,
                           bar: \"hello\".to_string()
                       }
                   }";
        let item = parse_str::<ast::ClassItem>(raw).unwrap();

        match item {
            ast::ClassItem::PrivateInit(_) => (),
            _ => unreachable!()
        };
    }

    #[test]
    fn parses_private_struct_class_items() {
        let raw = "class Foo {
                       struct FooPrivate {
                           foo: u32,
                           bar: String
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                               bar: \"hello\".to_string()
                           }
                       }
                   }";
        let class = parse_str::<ast::Class>(raw).unwrap();

        let mut iter = class.items.iter();

        let m = iter.next().unwrap();
        match *m {
            ast::ClassItem::PrivateStruct(_) => {
                (); // okay
            },

            _ => unreachable!()
        };

        let m = iter.next().unwrap();
        match *m {
            ast::ClassItem::PrivateInit (ref i) => {
                assert!(i.inputs.is_empty());

                match i.output {
                    FunctionRetTy::Ty(Ty::Path(TyPath { ref path, .. }), _) => {
                        assert_tokens_equal(path, "FooPrivate");
                    },

                    _ => unreachable!()
                }
            },

            _ => unreachable!()
        };

        assert!(iter.next().is_none());
    }

    #[test]
    fn parses_program() {
        let raw = "class Foo {
                       struct FooPrivate {
                           foo: u32,
                           bar: String
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                               bar: \"hello\".to_string()
                           }
                       }
                   }";
        let program = parse_str::<ast::Program>(raw).unwrap();

        assert!(program.items.len() == 1);
        assert!(ast::get_program_classes(&program).len() == 1);
    }

    fn test_parsing_impl_item(raw: &str, trait_name: Option<&str>, self_name: &str) {
        let item = parse_str::<ast::Item>(raw).unwrap();

        if let ast::Item::Impl(ref impl_) = item {
            if let Some((ref trait_path, _)) = impl_.trait_ {
                assert_tokens_equal(trait_path, trait_name.as_ref().unwrap());
            } else {
                assert!(trait_name.is_none());
            }

            assert_tokens_equal(&impl_.self_path, self_name);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn parses_plain_impl_item() {
        test_parsing_impl_item("impl Foo {}", None, "Foo");
    }

    #[test]
    fn parses_impl_item_with_trait() {
        test_parsing_impl_item("impl Foo for Bar {}", Some("Foo"), "Bar");
    }
}
