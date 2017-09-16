#![feature(catch_expr)]
#![feature(conservative_impl_trait)]
#![recursion_limit="128"]

// While under active devel, these warnings are kind of annoying.
#![allow(dead_code)]

#[macro_use] extern crate error_chain;
extern crate lalrpop_intern;
extern crate lalrpop_util;
#[macro_use] extern crate quote;
extern crate regex;
extern crate proc_macro;
extern crate unicode_xid;
extern crate rustfmt;

use proc_macro::TokenStream;
use errors::*;
use std::str::FromStr;

mod ast;
mod errors;
mod gen;
mod param;
mod parser;
mod tok;

#[proc_macro]
pub fn gobject_gen(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let result: Result<quote::Tokens> = do catch {
        let program = parser::parse_program(&input)?;
        gen::classes(&program)
    };

    match result {
        Ok(token_stream) => {
            let mut config: rustfmt::config::Config = Default::default();
            let mut out: Vec<u8> = vec!();
            config.set().write_mode(rustfmt::config::WriteMode::Plain);
            config.set().error_on_line_overflow(false);
            let stream: String = token_stream.as_str().into();
            match rustfmt::format_input(rustfmt::Input::Text(stream),
                                        & config,
                                        Some(& mut out)) {
                Ok(_) => {
                    let output = String::from_utf8(out).unwrap();
                    TokenStream::from_str(& output).unwrap()
                },
                Err(e) => {
                    println!("{}", e.0);
                    panic!("cannot generate gobjects")
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
            panic!("cannot generate gobjects")
        }
    }
}
