use super::parse::ParseError;
use crate::enumerator::parse::{ParseResult, ParsePos};
use std::default::Default;
use std::str::Chars;

enum State {
    Normal,
    InBlock { // inside '/* */'
        depth: usize, // 0-indexed
        last_start: ParsePos, // pos of '/' of last '/*'
    },
    InLine, // after '//'
    Errored, // after returning a Some(Err(...))
}

pub struct WithoutComments<I: Iterator<Item=char>> {
    iter: I,
    state: State,
    buf: Option<char>, // waiting to be interpreted
    pos: ParsePos,
}

#[derive(Debug)]
enum IterRet<T> {
    Finished, // equivalent to usual None
    Wait, // more to come
    Elem(T), // equivalent to usual Some(T)
}

// todo inline
impl<I: Iterator<Item=char>> WithoutComments<I> {
    fn from_iter(mut iter: I) -> Self {
        Self {
            iter,
            state: State::Normal,
            buf: None,
            pos: Default::default(),
        }
    }

    fn _next(&mut self) -> IterRet<ParseResult<char>> {
        use State::*;
        use IterRet::*;

        let ch = self.iter.next();
        let mut ret = Wait;
        match self.state {
            Normal => match (self.buf, ch) {
                (Some('/'), Some('/')) => {
                    self.state = InLine;
                    self.buf = None;
                },
                (Some('/'), Some('*')) => {
                    self.state = InBlock { depth: 0, last_start: self.pos - 1 };
                    self.buf = None;
                },
                (Some('*'), Some('/')) => {
                    self.state = Errored;
                    return Elem(Err(ParseError {
                        msg: "Unexpected block comment end".to_string(),
                        start: self.pos,
                        end: self.pos + 1,
                    }))
                },
                (Some(buf), new) => {
                    ret = Elem(Ok(buf));
                    self.buf = new;
                },
                (None, new) => self.buf = new,
            },

            InBlock { ref mut depth, ref mut last_start } => match (self.buf, ch) {
                (Some('/'), Some('*')) => {
                    *depth += 1;
                    *last_start = self.pos - 1;
                    self.buf = None;
                },
                (Some('*'), Some('/')) => if *depth == 0 {
                    self.state = Normal;
                    self.buf = None;
                },
                (_, c@Some(_)) => self.buf = c,
                (_, None) => return Elem(Err(ParseError {
                    msg: format!("Unclosed comment block at depth {}", *depth + 1),
                    start: *last_start,
                    end: self.pos,
                })),
            },

            InLine => match (self.buf, ch) {
                // leaves a blank line instead of a line comment on the last line
                // tldr, could be perceived as adding a trailing newline, idk
                (None, Some('\n')) => self.state = Normal,
                (None, None) => ret = Finished,
                (None, _) => (),
                x@(Some(_), _) => unreachable!("{:?}", x),
            },

            Errored => panic!("Tried to continue WithoutComments iterator that was errored out"),
        }

        if let Some('\n') = ch {
            self.pos.line += 1;
            self.pos.pos = 0;
        } else {
            self.pos.pos += 1;
        }

        ret
    }
}

impl<I: Iterator<Item=char>> Iterator for WithoutComments<I> {
    type Item = ParseResult<char>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self._next() {
                IterRet::Finished => return None,
                IterRet::Wait => (),
                IterRet::Elem(e) => return Some(e),
            }
        }
    }
}

pub trait IntoWithoutComments where Self: Sized + Iterator<Item=char> {
    fn without_comments(self) -> WithoutComments<Self>;
}

impl<I: Iterator<Item=char>> IntoWithoutComments for I {
    fn without_comments(self) -> WithoutComments<Self> {
        WithoutComments::from_iter(self)
    }
}