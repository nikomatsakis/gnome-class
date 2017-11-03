use quote::{Tokens, ToTokens};
use syn::Ident;
use proc_macro2::{Literal, TokenNode};

/// Wraps an `Ident` so it can be tokenized as a C-friendly string literal.
///
/// For example, if the ident holds the name Foo, then this will
/// generate "`b"Foo\0" as *const u8 as *const i8`", which is suitable
/// for calling C APIs.
pub struct CStringIdent(pub Ident);

impl ToTokens for CStringIdent {
    fn to_tokens(&self, tokens: &mut Tokens) {
        // Make a b"Foo\0" byte literal

        let s = self.0.as_ref();
        let mut v = Vec::from (s.as_bytes());
        v.push (0u8);
        tokens.append (TokenNode::Literal(Literal::byte_string(&v)));

        // Cast it to what glib expects

        tokens.append_tokens (quote! { as *const u8 as *const i8 });
    }
}
