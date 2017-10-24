use quote::{Tokens, ToTokens};
use syn::Ident;

/// Wraps an `Ident` so it can be tokenized as a C-friendly string literal.
///
/// For example, if the ident holds the name Foo, then this will
/// generate "`b"Foo\0" as *const u8 as *const i8`", which is suitable
/// for calling C APIs.
pub struct CStringIdent(pub Ident);

impl ToTokens for CStringIdent {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append("b\"");
        tokens.append(self.0.as_ref());
        tokens.append("\\0\"");
    }
}
