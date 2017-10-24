use quote::{Tokens, ToTokens};
use syn::Ident;

struct CStringIdent(Ident);

impl ToTokens for CStringIdent {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append("b\"");
        tokens.append(self.0.as_ref());
        tokens.append("\\0\"");
    }
}
