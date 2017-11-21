#![allow(non_snake_case)]

use quote::{Tokens, ToTokens};
use ast;

pub fn tokens_GObject() -> Tokens {
    quote_cs! { glib::Object }
}

pub fn tokens_GObjectFfi() -> Tokens {
    quote_cs! { gobject_ffi::GObject }
}

pub fn tokens_GObjectClassFfi() -> Tokens {
    quote_cs! { gobject_ffi::GObjectClass }
}

pub fn tokens_ParentInstance(class: &ast::Class) -> Tokens {
    class.extends
        .as_ref()
        .map(|path| {
            let mut tokens = Tokens::new();
            path.to_tokens(&mut tokens);
            tokens
        })
        .unwrap_or_else(|| tokens_GObject())
}

pub fn tokens_ParentInstanceFfi(class: &ast::Class) -> Tokens {
    let ParentInstance = tokens_ParentInstance(class);
    quote_cs! {
        <#ParentInstance as glib::wrapper::Wrapper>::GlibType
    }
}

pub fn tokens_ParentClassFfi(class: &ast::Class) -> Tokens {
    let ParentInstance = tokens_ParentInstance(class);
    quote_cs! {
        <#ParentInstance as glib::wrapper::Wrapper>::GlibClassType
    }
}

pub fn glib_callback_guard() -> Tokens {
    quote_cs! {
        let _guard = glib::CallbackGuard::new();
    }
}

pub fn lower_case_instance_name(instance_name: &str) -> String {
    let mut name_chars = instance_name.chars();
    let first_char: char = name_chars.next().unwrap();
    first_char.to_lowercase().chain(name_chars).collect()
}

pub mod tests {
    use super::*;

    pub fn run() {
        lower_cases_simple_names();
    }

    fn lower_cases_simple_names() {
        assert_eq!("foo", lower_case_instance_name("Foo"));
    }
}
