#![feature(catch_expr)]
#![feature(proc_macro)]

#[macro_use] extern crate error_chain;
extern crate lalrpop_intern;
extern crate lalrpop_util;
#[macro_use] extern crate quote;
extern crate regex;
extern crate proc_macro;
extern crate unicode_xid;
use proc_macro::TokenStream;
use errors::*;

mod ast;
mod errors;
mod parser;
mod tok;

#[proc_macro]
pub fn gobject_gen(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let result: Result<TokenStream> = do catch {
        let program = parser::parse_program(&input)?;
        let program_str = format!("{:#?}", program);
        let quote = quote! {
            const XXX: &str = #program_str;
        };
        Ok(quote.parse().unwrap())
    };

    match result {
        Ok(token_stream) => token_stream,
        Err(e) => {
            println!("{:?}", e);
            panic!("cannot generate gobjects")
        }
    }
}

