#![feature(catch_expr)]
#![feature(conservative_impl_trait)]
#![feature(proc_macro)]
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

use proc_macro::TokenStream;
use errors::*;
use std::process::{Command, Stdio};
use std::io::Write;

mod ast;
mod errors;
mod gen;
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
            let output = rustfmt(&token_stream).unwrap();
            println!("{}", output);
            token_stream.parse().unwrap()
        }
        Err(e) => {
            println!("{:?}", e);
            panic!("cannot generate gobjects")
        }
    }
}

fn rustfmt(token_stream: &quote::Tokens) -> Result<String> {
    let mut process =
        Command::new("rustfmt")
        .stdin(Stdio::piped())
        .spawn()?;

    write!(process.stdin.as_mut().unwrap(), "{}", token_stream.as_str())?;
    let output = process.wait_with_output()?;
    Ok(String::from_utf8(output.stdout).unwrap())
}
