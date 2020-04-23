// might be useful later

// use super::Enumerator;
// use regex::Regex;
// use std::ops::{Add, Sub};
// use std::fmt::{Display, Formatter, Error};
// use std::fmt;
// use crate::turing_machine::Dir;
//
// #[derive(Copy, Clone, Debug)]
// pub struct ParsePos {
//     pub line: usize, // 0-indexed
//     pub pos: usize, // in line, 0-indexed
// }
//
// impl Add<usize> for ParsePos {
//     type Output = ParsePos;
//
//     fn add(mut self, rhs: usize) -> Self::Output {
//         self.pos += rhs;
//         self
//     }
// }
//
// impl Sub<usize> for ParsePos {
//     type Output = ParsePos;
//
//     fn sub(mut self, rhs: usize) -> Self::Output {
//         self.pos -= rhs; // panic if <0
//         self
//     }
// }
//
// impl Display for ParsePos {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}:{}", self.line, self.pos)
//     }
// }
//
// impl Default for ParsePos {
//     fn default() -> Self {
//         Self { line: 0, pos: 0 }
//     }
// }
//
//
// // note debug prints 0-indexed positions while display prints 1-indexed positions
// #[derive(Debug)]
// pub struct ParseError {
//     pub msg: String,
//     pub start: ParsePos,
//     pub end: ParsePos,
// }
//
// impl Display for ParseError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         // convert to 1-indexed positions
//         let mut start = self.start;
//         start.line += 1;
//         start.pos += 1;
//         let mut end = self.end;
//         end.line += 1;
//         end.pos += 1;
//         write!(f, "Parse error at {} .. {}: {}", start, end, self.msg)
//     }
// }
//
// pub type ParseResult<T> = Result<T, ParseError>;
//
//
// struct PartialRule {
//     from_state: Option<String>,
//     to_state: Option<String>,
//     read: Option<String>,
//     write: Option<String>,
//     go: Option<char>,
// }
//
// #[derive(Default)]
// struct Rule {
//     from_state: String,
//     to_state: String,
//     read: String,
//     write: String,
//     go: char,
// }
//
// impl PartialRule {
//     fn started(&self) -> bool { self.from_state.is_some() }
//     fn finished(&self) -> bool { self.go.is_some() }
//
//     fn compile(self) -> Option<Rule> {
//         Some(Rule {
//             from_state: self.from_state?,
//             to_state: self.to_state?,
//             read: self.read?,
//             write: self.write?,
//             go: self.go?,
//         })
//     }
// }
//
// struct Parser<I: Iterator<Item=char>> {
//     iter: I,
//     partial: PartialRule,
// }
//
// impl<I: Iterator<Item=char>> Parser<I> {
//     pub fn from_iter(iter: I) -> Self {
//         Self {
//             iter,
//             partial: Default::default(),
//         }
//     }
//
//     fn next(&mut self) -> Option<Rule> {
//
//     }
// }