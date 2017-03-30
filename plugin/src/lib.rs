#![feature(proc_macro)]

extern crate lalrpop_intern;
extern crate lalrpop_util;
extern crate regex;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn rewrite(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    assert_eq!(input, r#""Hello, world!""#);

    r#""NOT Hello, world!""#.parse().unwrap()
}

