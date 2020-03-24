use super::Enumerator;
use regex::Regex;
use std::ops::{Add, Sub};
use std::fmt::{Display, Formatter, Error};
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct ParsePos {
    pub line: usize, // 0-indexed
    pub pos: usize, // in line, 0-indexed
}

impl Add<usize> for ParsePos {
    type Output = ParsePos;

    fn add(mut self, rhs: usize) -> Self::Output {
        self.pos += rhs;
        self
    }
}

impl Sub<usize> for ParsePos {
    type Output = ParsePos;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self.pos -= rhs; // panic if <0
        self
    }
}

impl Display for ParsePos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}

impl Default for ParsePos {
    fn default() -> Self {
        Self { line: 0, pos: 0 }
    }
}


#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub start: ParsePos,
    pub end: ParsePos,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at {} .. {}: {}", self.start, self.end, self.msg)
    }
}

pub type ParseResult<T> = Result<T, ParseError>;


impl Enumerator {
    /* private */ fn from_rules_str(rules: String) -> ParseResult<Self> {
        todo!()
    }
}