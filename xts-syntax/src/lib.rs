#![allow(unused)]
use std::{ops::RangeInclusive, str::Chars};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // keywords
    Let,      // let
    Mut,      // mut
    Const,    // const
    Fn,       // fn
    If,       // if
    Else,     // else
    Struct,   // struct
    Loop,     // loop
    For,      // for
    While,    // while
    In,       // in
    Break,    // break
    Continue, // continue
    Return,   // return
    True,     // true
    False,    // false
    Match,    // match
    Impl,     // impl
    Trait,    // trait

    // Identifiers and literals
    Identifier(String),
    Int(i64),
    Float(f64),
    BigInt(String),
    String(String),
    Boolean(bool),

    // Operator
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Equals,  // =
    EqEq,    // ==
    NotEq,   // !=
    Lt,      // <
    LtEq,    // <=
    Gt,      // >
    GtEq,    // >=
    And,     // &&
    Or,      // ||
    Bang,    // !

    // Delimiters
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    DoubleColon, // ::
    Dot,         // .
    Arrow,       // ->
    Underscore,  // _

    // Parentheses & Braces
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]

    // End of File
    Eof,
}

struct WithPos<T> {
    value: T,
    start: usize,
    end: usize,
}

impl<T> WithPos<T> {
    pub(crate) fn new(value: T, pos: RangeInclusive<usize>) -> Self {
        Self {
            value,
            start: pos.start().clone(),
            end: pos.end().clone(),
        }
    }
}

struct Scanner<'a> {
    input: Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn bump(&mut self) -> Option<char> {
        self.input.next()
    }
}

impl Iterator for Scanner<'_> {
    type Item = WithPos<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.bump() {
            Some('+') => Some(WithPos::new(Token::Plus, 0..=1)),
            Some('-') => Some(WithPos::new(Token::Minus, 0..=1)),
            _ => todo!(),
        }
    }
}
