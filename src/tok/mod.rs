//! A tokenizer for use in the plugin. LALRPOP's built-in tokenizer
//! doesn't have quite enough features for our use (in particular, our
//! tokenization need is not regular). This tokenizer has a few
//! interesting quirks:
//!
//! The `{...}` sequence parses into a `Block` token with balanced `{`
//! and `}` tokens. We sometimes preserve this "as is" (basically a
//! bucket of bytes) and other times we will recursively process it.

use errors::*;
use std::str::CharIndices;
use unicode_xid::UnicodeXID;

use self::Tok::*;

mod test;

fn unrecognized_token<T>(offset: usize) -> Result<T> {
    bail!(ErrorKind::LexError(offset, "unrecognized token"))
}

fn unbalanced_delims<T>(offset: usize) -> Result<T> {
    bail!(ErrorKind::LexError(offset, "unbalanced delimeters"))
}

fn unterminated_block<T>(offset: usize) -> Result<T> {
    bail!(ErrorKind::LexError(offset, "unterminated block"))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    ClassKeyword, // `class`
    AsKeyword, // `as`
    StructKeyword, // `struct`
    FnKeyword, // `fn`
    PrivateInitKeyword, // `private_init`
    SelfKeyword, // `self`
    SuperKeyword, // `super`
    ExtendsKeyword, // `extends`
    SignalKeyword, // `signal`
    Id(&'input str), // identifier
    ThinArrow, // `->`
    Underscore, // `_`
    LeftParen, // `(`,
    RightParen, // `)`,
    LeftBracket, // `[`
    RightBracket, // `]`
    Comma, // `,`
    DotDot, // `..`
    Colon, // `:`
    ColonColon, // `::`
    LessThan, // `<`
    GreaterThan, // `>`,
    Bang, // `!`
    Semi, // `;`
    Plus, // `+`
    Block(&'input str), // {...}, balanced
    Ampersand, // `&`
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

macro_rules! eof {
    ($x:expr) => {
        match $x { Some(v) => v, None => { return None; } }
    }
}

pub type Spanned<T> = (usize, T, usize);

const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("class", ClassKeyword),
    ("struct", StructKeyword),
    ("fn", FnKeyword),
    ("private_init", PrivateInitKeyword),
    ("extends", ExtendsKeyword),
    ("signal", SignalKeyword),
    ("self", SelfKeyword),
    ("super", SuperKeyword),
    ("as", AsKeyword),
    ("_", Underscore),
    ];


impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
        };
        t.bump();
        t
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>>> {
        loop {
            return match self.lookahead {
                Some((idx0, '!')) => {
                    Some(Ok((idx0, Bang, idx0+1)))
                }
                Some((idx0, ':')) => {
                    match self.bump() {
                        Some((idx1, ':')) => {
                            self.bump();
                            Some(Ok((idx0, ColonColon, idx1+1)))
                        }
                        _ => {
                            Some(Ok((idx0, Colon, idx0+1)))
                        }
                    }
                }
                Some((idx0, '-')) => {
                    match self.bump() {
                        Some((idx1, '>')) => {
                            self.bump();
                            Some(Ok((idx0, ThinArrow, idx1+1)))
                        }
                        _ => {
                            Some(unrecognized_token(idx0))
                        }
                    }
                }
                Some((idx0, ',')) => {
                    self.bump();
                    Some(Ok((idx0, Comma, idx0+1)))
                }
                Some((idx0, '.')) => {
                    match self.bump() {
                        Some((idx1, '.')) => {
                            self.bump();
                            Some(Ok((idx0, DotDot, idx1+1)))
                        }
                        _ => {
                            Some(unrecognized_token(idx0))
                        }
                    }
                }
                Some((idx0, '>')) => {
                    self.bump();
                    Some(Ok((idx0, GreaterThan, idx0+1)))
                }
                Some((idx0, '{')) => {
                    match self.block_contents(idx0) {
                        Ok(idx1) => {
                            Some(Ok((idx0, Block(&self.text[idx0..idx1]), idx1)))
                        }
                        Err(err) => Some(Err(err))
                    }
                }
                Some((idx0, '[')) => {
                    self.bump();
                    Some(Ok((idx0, LeftBracket, idx0+1)))
                }
                Some((idx0, '(')) => {
                    self.bump();
                    Some(Ok((idx0, LeftParen, idx0+1)))
                }
                Some((idx0, '<')) => {
                    self.bump();
                    Some(Ok((idx0, LessThan, idx0+1)))
                }
                Some((idx0, ']')) => {
                    self.bump();
                    Some(Ok((idx0, RightBracket, idx0+1)))
                }
                Some((idx0, ')')) => {
                    self.bump();
                    Some(Ok((idx0, RightParen, idx0+1)))
                }
                Some((idx0, ';')) => {
                    self.bump();
                    Some(Ok((idx0, Semi, idx0+1)))
                }
                Some((idx0, '&')) => {
                    self.bump();
                    Some(Ok((idx0, Ampersand, idx0+1)))
                }
                Some((idx0, '+')) => {
                    self.bump();
                    Some(Ok((idx0, Plus, idx0+1)))
                }
                Some((idx0, '/')) => {
                    match self.bump() {
                        Some((_, '/')) => {
                            self.take_until(|c| c == '\n');
                            continue;
                        }
                        _ => {
                            Some(unrecognized_token(idx0))
                        }
                    }
                }
                Some((idx0, c)) if is_identifier_start(c) => {
                    Some(self.identifierish(idx0))
                }
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((idx, _)) => {
                    Some(unrecognized_token(idx))
                }
                None => {
                    None
                }
            };
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn block_contents(&mut self, idx0: usize) -> Result<usize> {
        const OPEN_DELIMS: &str = "{[(";
        const CLOSE_DELIMS: &str = "}])";

        let mut delims = vec![];
        loop {
            if let Some((idx, c)) = self.lookahead {
                self.bump();
                if let Some(open_index) = OPEN_DELIMS.find(c) {
                    delims.push(open_index);
                } else if let Some(close_index) = CLOSE_DELIMS.find(c) {
                    match delims.pop() {
                        Some(open_index) => {
                            if open_index != close_index {
                                return unbalanced_delims(idx);
                            }
                        }
                        None => {
                            return unbalanced_delims(idx);
                        }
                    }

                    // If we just popped the last delimeter, we are
                    // done.
                    if delims.is_empty() {
                        return Ok(idx+1);
                    }
                } else {
                    // We always invoke `block_contents` on a `{` character.
                    assert!(!delims.is_empty());
                }
            } else {
                return unterminated_block(idx0);
            }
        }
    }

    fn identifierish(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>> {
        let (start, word, end) = self.word(idx0);

        let tok =
            // search for a keyword first; if none are found, this is
            // either a MacroId or an Id, depending on whether there
            // is a `<` immediately afterwards
            KEYWORDS.iter()
                    .filter(|&&(w, _)| w == word)
                    .map(|&(_, ref t)| t.clone())
                    .next()
                    .unwrap_or_else(|| Id(word));

        Ok((start, tok, end))
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>>> {
        match self.next_unshifted() {
            None =>
                None,
            Some(Ok((l, t, r))) =>
                Some(Ok((l+self.shift, t, r+self.shift))),
            Some(Err(Error(ErrorKind::LexError(location, code), trace))) =>
                Some(Err(Error(ErrorKind::LexError(location+self.shift, code), trace))),
            Some(Err(e)) =>
                panic!("only expected `LexError` to result from lexer, not `{:?}`", e),
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c) || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c) || c == '_'
}
