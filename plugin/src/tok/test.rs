#![cfg(test)]

use errors::*;
use super::{Tok, Tokenizer};
use super::Tok::*;

enum Expectation<'a> {
    ExpectTok(Tok<'a>),
    ExpectErr(&'a str)
}

use self::Expectation::*;

fn gen_test(input: &str, expected: Vec<(&str, Expectation)>)
{
    // use $ to signal EOL because it can be replaced with a single space
    // for spans, and because it applies also to r#XXX# style strings:
    let input = input.replace("$", "\n");

    let tokenizer = Tokenizer::new(&input, 0);
    let len = expected.len();
    for (token, (expected_span, expectation)) in tokenizer.zip(expected.into_iter()) {
        let expected_start = expected_span.find("~").unwrap();
        let expected_end = expected_span.rfind("~").unwrap() + 1;
        println!("token: {:?}", token);
        match expectation {
            ExpectTok(expected_tok) => {
                match token {
                    Ok(triple) => {
                        if triple == (expected_start, expected_tok, expected_end) {
                            continue;
                        }
                    }
                    Err(_) => { }
                }

                panic!("expected token `{:?}` from {}-{}, but found `{:?}`",
                       expected_tok, expected_start, expected_end, token);
            }
            ExpectErr(expected_msg) => {
                match token {
                    Err(Error(ErrorKind::LexError(location, code), _)) => {
                        if location == expected_start && code.contains(expected_msg) {
                            continue;
                        }
                    }
                    _ => { }
                }

                panic!("expected `{}` at {}, but found `{:?}`",
                       expected_start, expected_msg, token);
            }
        }
    }

    let tokenizer = Tokenizer::new(&input, 0);
    assert!(tokenizer.skip(len).next().is_none());
}

fn test(input: &str, expected: Vec<(&str, Tok)>)
{
    let generic_expected = expected.into_iter().map( | (span, tok) | (span, ExpectTok(tok)) ).collect();
    gen_test(input, generic_expected);
}

fn test_err(input: &str, span: &str, ec: &str) {
    gen_test(input, vec![(span, ExpectErr(ec))])
}

#[test]
fn basic() {
    test("class Foo", vec![
        ("~~~~~    ", Class),
        ("      ~~~", Id("Foo")),
    ]);
}

#[test]
fn eol_comment() {
    test("class // This is a comment$ foo", vec![
        ("~~~~~                          ", Class),
        ("                            ~~~", Id("foo")),
    ]);
}

#[test]
fn block() {
    test("class { foo(); [] // comment$ bar(); } foo", vec![
        ("~~~~~                          ", Class),
        ("      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~", Block("{ foo(); [] // comment\n bar(); }")),
        ("                                       ~~~", Id("foo")),
    ]);
}

#[test]
fn unbalanced_delim() {
    test_err("{ foo(); ]",
             "         ~",
             "unbalanced delimeters");
}

#[test]
fn unterminated_delim() {
    test_err("{ foo();",
             "~",
             "unterminated block");
}
