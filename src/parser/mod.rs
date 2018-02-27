use proc_macro::TokenStream;

use syn::{self, Ident, Path, parse_error};
use syn::punctuated::{Punctuated};
use syn::synom::{Synom, PResult};
use syn::buffer::Cursor;

use ast;
use errors::*;

pub fn parse_program(token_stream: TokenStream) -> Result<ast::Program> {
    syn::parse(token_stream).map_err(|e| e.into())
}

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
            punct!(:)                                            >>
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
    ));
}

impl Synom for ast::InstancePrivateItem {
    named!(parse -> Self, do_parse!(
        type_: keyword!(type)             >>
        call!(keyword("InstancePrivate")) >>
        eq: punct!(=)                     >>
        path: syn!(Path)                  >>
        semi: punct!(;)                   >>
        (ast::InstancePrivateItem {
            type_token: type_,
            eq_token:   eq,
            path:       path,
            semi_token: semi
        })
    ));
}

impl Synom for ast::Impl {
    named!(parse -> Self, do_parse!(
        keyword!(impl) >>
        trait_: option!(do_parse!(
            path: syn!(Ident) >>
            keyword!(for) >>
            (path)
        )) >>
        self_path: syn!(Ident) >>
        body: braces!(many0!(syn!(ast::ImplItem))) >>
        (ast::Impl {
            trait_: trait_,
            self_path: self_path,
            items: body.0
        })
    ));
}

impl Synom for ast::ImplItem {
    named!(parse -> Self, do_parse!(
        attrs: many0!(call!(syn::Attribute::parse_outer)) >>
        node: syn!(ast::ImplItemKind) >>
        (ast::ImplItem { attrs, node })
    ));
}

impl Synom for ast::ImplItemKind {
    named!(parse -> Self, alt!(
        syn!(ast::ImplItemMethod) => { |x| ast::ImplItemKind::Method(x) }
        |
        do_parse!(
            call!(keyword("reserve_slots")) >>
            slots: parens!(syn!(syn::Lit)) >>
            (ast::ImplItemKind::ReserveSlots(slots.1))
        )
    ));
}

impl Synom for ast::ImplItemMethod {
    named!(parse -> Self, do_parse!(
        public: map!(option!(call!(keyword("pub"))), |x| x.is_some()) >>
        virtual_: map!(option!(call!(keyword("virtual"))), |x| x.is_some()) >>
        signal: map!(option!(call!(keyword("signal"))), |x| x.is_some()) >>
        keyword!(fn) >>
        name: syn!(syn::Ident) >>
        params: parens!(Punctuated::parse_terminated) >>
        output: syn!(syn::ReturnType) >>
        body: alt!(
            syn!(syn::Block) => { Some }
            |
            punct!(;) => { |_| None }
        ) >>
        (ast::ImplItemMethod {
            public,
            virtual_,
            signal,
            name,
            inputs: params.1,
            output,
            body,
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
        if let Some((_, s, rest)) = input.term() {
            if s.as_str() == name {
                return Ok(((), rest));
            }
        }
        parse_error() // FIXME: use a meaningful error message when synom allows for it
    }
}

pub mod tests {
    use super::*;
    use quote;
    use quote::ToTokens;
    use syn::{parse_str};

    pub fn run() {
        parses_class_with_no_superclass();
        parses_class_with_superclass();
        parses_instance_private_item();
        parses_class_item();
        parses_program();
        parses_plain_impl_item();
        parses_impl_item_with_trait();
    }

    fn assert_tokens_equal<T: ToTokens>(x: &T, s: &str) {
        let mut tokens = quote::Tokens::new();
        x.to_tokens(&mut tokens);
        assert_eq!(tokens.to_string(), s);
    }

    fn parses_class_with_no_superclass() {
        let raw = "class Foo {}";
        let class = parse_str::<ast::Class>(raw).unwrap();

        assert_eq!(class.name.as_ref(), "Foo");
        assert!(class.extends.is_none());
    }

    fn parses_class_with_superclass() {
        let raw = "class Foo: Bar {}";
        let class = parse_str::<ast::Class>(raw).unwrap();

        assert_eq!(class.name.as_ref(), "Foo");
        assert_tokens_equal(&class.extends, "Bar");
    }

    fn parses_instance_private_item() {
        let raw = "type InstancePrivate = FooPrivate;";
        let item = parse_str::<ast::ClassItem>(raw).unwrap();

        match item {
            ast::ClassItem::InstancePrivate(item) => {
                assert_tokens_equal(&item.path, "FooPrivate");
            }
        }
    }

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

    fn parses_program() {
        let raw = "class Foo {
                       type InstancePrivate = FooPrivate;
                   }";
        let program = parse_str::<ast::Program>(raw).unwrap();

        assert!(program.items.len() == 1);
        assert!(ast::get_program_classes(&program).len() == 1);
    }

    fn test_parsing_impl_item(raw: &str, trait_name: Option<&str>, self_name: &str) {
        let item = parse_str::<ast::Item>(raw).unwrap();

        if let ast::Item::Impl(ref impl_) = item {
            if let Some(trait_path) = impl_.trait_ {
                assert_tokens_equal(&trait_path, trait_name.as_ref().unwrap());
            } else {
                assert!(trait_name.is_none());
            }

            assert_tokens_equal(&impl_.self_path, self_name);
        } else {
            unreachable!();
        }
    }

    fn parses_plain_impl_item() {
        test_parsing_impl_item("impl Foo {}", None, "Foo");
    }

    fn parses_impl_item_with_trait() {
        test_parsing_impl_item("impl Foo for Bar {}", Some("Foo"), "Bar");
    }
}
