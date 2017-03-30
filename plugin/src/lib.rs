#![feature(catch_expr)]
#![feature(proc_macro)]

#[macro_use] extern crate error_chain;
extern crate lalrpop_intern;
extern crate lalrpop_util;
#[macro_use] extern crate quote;
extern crate regex;
extern crate proc_macro;

use proc_macro::TokenStream;
use errors::*;

mod ast;
mod errors;
mod parser;
mod param;

#[proc_macro]
pub fn gobject_gen(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let result: Result<TokenStream> = do catch {
        let program = parse_program(&input)?;
        let quote = quote! { struct Dummy; };
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

fn parse_program(input: &str) -> Result<ast::Program> {
    match parser::parse_Program(input) {
        Ok(p) => Ok(p),
        Err(_) => bail!("parse error...somewhere...")
    }
}
