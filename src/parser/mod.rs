use ast;
use errors::*;
//use lalrpop_util::ParseError;
use proc_macro::{TokenStream, TokenTree, TokenNode, Term, Delimiter};
//use tok::{self, Tok};

//mod tokens;

//mod grammar;

pub fn parse_program(input: &str) -> Result<ast::Program> {
    unimplemented!()
    /*
    let tokenizer = tok::Tokenizer::new(input, 0);
    match grammar::parse_Program(tokenizer) {
        Ok(p) => Ok(p),
        Err(e) => bail!("parse error: {:?}", e),
    }
     */
}

/*
type ParseResult<'input, T> =
    ::std::result::Result<T, ParseError<usize, Tok<'input>, Error>>;
fn parse_members(input: &str,
                 offset: usize)
                 -> ParseResult<Vec<ast::Member>> {
    assert_eq!(input.chars().next(), Some('{'));
    assert!(input.len() >= 2); // at least `{}`
    let len = input.len();
    let contents = &input[1..len - 1];
    let tokenizer = tok::Tokenizer::new(contents, offset + 1);
    grammar::parse_Members(tokenizer)
}

fn parse_var_tys(input: &str,
                 offset: usize)
                 -> ParseResult<Vec<ast::VarTy>> {
    assert_eq!(input.chars().next(), Some('{'));
    assert!(input.len() >= 2); // at least `{}`
    let len = input.len();
    let contents = &input[1..len - 1];
    let tokenizer = tok::Tokenizer::new(contents, offset + 1);
    grammar::parse_VarTys(tokenizer)
}
 */

use synom::{Synom, Cursor, PResult, parse_error};
use syn;

impl Synom for ast::Class {
    named!(parse -> Self, do_parse!(
        call!(keyword("class"))    >>
        name:  syn!(syn::Ident)    >>
        block: syn!(syn::Block)    >>
        (ast::Class {
            name: name,
            extends: None, // FIXME
            members: Vec::new() // FIXME
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
fn keyword<'a>(name: &'static str) -> Box<Fn(Cursor<'a>) -> PResult<()>> {
    Box::new(move |input: Cursor<'a>| {
        if let Some((rest, _, s)) = input.word() {
            if s.as_str() == name {
                return Ok((rest, ()));
            }
        }
        parse_error() // FIXME: use a meaningful error message when synom allows for it
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let raw = "class Foo {}";

        let token_stream = raw.parse::<TokenStream>().unwrap();
    }
}
