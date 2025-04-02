#![allow(unused)]
use std::{cell::RefCell, ops::RangeInclusive, rc::Rc, str::Chars, usize};

use pos::WithPos;
use token::TokenType;

mod pos;
mod token;

struct TokenMatcher<'a> {
    start: Option<(usize, usize)>,
    tokenizer: Scanner<'a>,
    value: Option<TokenType>,
    is_prev_match: bool,
}

impl<'a> TokenMatcher<'a> {
    fn new(tokenizer: Scanner<'a>) -> Self {
        Self {
            start: None,
            tokenizer,
            value: None,
            is_prev_match: true,
        }
    }
    pub fn and_then(&mut self, ch: char, matched: TokenType) -> &mut Self {
        if !self.is_prev_match {
            return self;
        }
        match self.tokenizer.peek() {
            Some(c) if c == ch => {
                if self.start.is_none() {
                    self.start = Some(self.tokenizer.pos());
                }
                self.tokenizer.bump();
                self.value = Some(matched);
                self
            }
            _ => {
                self.is_prev_match = false;
                self
            }
        }
    }
    pub fn then<F: Fn(char) -> Option<TokenType>>(&mut self, f: F) -> &mut Self {
        todo!()
    }
    pub fn finalized(&self) -> Option<WithPos<Token>> {
        let start = self.start?;
        let end_pos = self.tokenizer.pos();
        let line_pos = start.0..end_pos.0;
        let byte_pos = start.1..end_pos.1;
        let value = self.value.clone()?;
        Some(
            WithPos::new(Token::new(value))
                .set_byte_pos(byte_pos)
                .set_line_pos(line_pos),
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    _value: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type,
            _value: None,
        }
    }
    pub fn value(mut self, value: String) -> Self {
        self._value = Some(value);
        self
    }
}

pub struct Scanner<'a> {
    input: Rc<RefCell<Chars<'a>>>,
    line: Rc<RefCell<usize>>,
    pos: Rc<RefCell<usize>>,
}

impl Clone for Scanner<'_> {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
            line: self.line.clone(),
            pos: self.pos.clone(),
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            input: Rc::new(RefCell::new(value.chars())),
            line: Rc::new(RefCell::new(0)),
            pos: Rc::new(RefCell::new(0)),
        }
    }
    #[inline]
    pub(crate) fn bump(&mut self) -> Option<char> {
        match self.input.borrow_mut().next() {
            ch @ Some('\n') => {
                *self.pos.borrow_mut() += 1; // utf-8 length of new line is 1
                *self.line.borrow_mut() += 1;
                ch
            }
            ch @ Some(c) => {
                *self.pos.borrow_mut() += c.len_utf8();
                ch
            }
            none => none,
        }
    }
    #[inline]
    pub(crate) fn peek(&self) -> Option<char> {
        self.input.borrow().clone().next()
    }
    pub(crate) fn pos(&self) -> (usize, usize) {
        (self.line.borrow().clone(), self.pos.borrow().clone())
    }
    pub(crate) fn consume_while<F: Fn(char) -> bool>(&mut self, f: F) -> String {
        let mut v = String::new();
        loop {
            match self.peek() {
                Some(ch) if f(ch) => {
                    self.bump();
                    v.push(ch);
                }
                _ => break,
            }
        }
        v
    }
    pub(crate) fn consume_identifier(&mut self) -> Option<WithPos<Token>> {
        match self.peek() {
            Some(ch) if !matches!(ch, 'a'..='z' | 'A'..='Z' | '_') => return None,
            _ => {}
        }
        let line_pos = self.line.borrow().clone();
        let byte_pos_start = self.pos.borrow().clone();
        let identifier =
            self.consume_while(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
        let byte_pos_end = self.pos.borrow().clone();
        if byte_pos_start == byte_pos_end {
            None
        } else {
            let token_kind = match identifier.as_str() {
                "let" => TokenType::Let,
                "mut" => TokenType::Mut,
                "const" => TokenType::Const,
                "fn" => TokenType::Fn,
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "struct" => TokenType::Struct,
                "loop" => TokenType::Loop,
                "for" => TokenType::For,
                "while" => TokenType::While,
                "in" => TokenType::In,
                "break" => TokenType::Break,
                "continue" => TokenType::Continue,
                "return" => TokenType::Return,
                "true" => TokenType::True,
                "false" => TokenType::False,
                "match" => TokenType::Match,
                "impl" => TokenType::Impl,
                "trait" => TokenType::Trait,
                _ => TokenType::Identifier,
            };
            if token_kind == TokenType::Identifier {
                let token = WithPos::new(Token::new(token_kind).value(identifier))
                    .set_byte_pos(byte_pos_start..byte_pos_end)
                    .set_line_pos(line_pos..line_pos);
                Some(token)
            } else {
                let token = WithPos::new(Token::new(token_kind))
                    .set_byte_pos(byte_pos_start..byte_pos_end)
                    .set_line_pos(line_pos..line_pos);
                Some(token)
            }
        }
    }
    pub(crate) fn consume_number(&mut self) -> Option<WithPos<Token>> {
        todo!()
    }
    pub(crate) fn consume_string(&mut self) -> Option<WithPos<Token>> {
        todo!()
    }
    pub(crate) fn token_matcher(&mut self) -> TokenMatcher<'a> {
        TokenMatcher::new(self.clone())
    }
}

impl Iterator for Scanner<'_> {
    type Item = WithPos<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        let peek = self.peek();
        if peek.is_none() {
            return None;
        }
        loop {
            match self.peek().unwrap() {
                ch if ch.is_whitespace() => {
                    self.bump();
                    continue;
                }
                'a'..='z' | 'A'..='Z' | '_' => return self.consume_identifier(),
                '0'..='9' => return self.consume_number(),
                '\'' | '"' => return self.consume_string(),
                ch @ '=' => {
                    return self
                        .token_matcher()
                        .and_then(ch, TokenType::Eq)
                        .and_then('=', TokenType::EqEq)
                        .finalized();
                }
                ch @ '!' => {
                    return self
                        .token_matcher()
                        .and_then(ch, TokenType::Bang)
                        .and_then('=', TokenType::NotEq)
                        .finalized();
                }
                ch @ '<' => {
                    return self
                        .token_matcher()
                        .and_then(ch, TokenType::Lt)
                        .and_then('=', TokenType::LtEq)
                        .finalized();
                }
                ch @ '>' => {
                    return self
                        .token_matcher()
                        .and_then(ch, TokenType::Gt)
                        .and_then('=', TokenType::GtEq)
                        .finalized();
                }
                ch @ '&' => {
                    return self
                        .token_matcher()
                        .and_then(ch, TokenType::And)
                        .and_then('&', TokenType::AndAnd)
                        .finalized();
                }
                ch @ '|' => {
                    return self
                        .token_matcher()
                        .and_then(ch, TokenType::Pipe)
                        .and_then('|', TokenType::Or)
                        .finalized();
                }
                ch @ '/' => {
                    return self
                        .token_matcher()
                        .and_then(ch, TokenType::Slash)
                        .finalized();
                }
                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Scanner, Token, pos::WithPos, token::TokenType};

    #[test]
    fn match_token() {
        let input = "===let abc";
        let mut sc = Scanner::new(input);
        assert_eq!(
            sc.next().unwrap(),
            WithPos::new(Token::new(TokenType::EqEq))
                .set_byte_pos(0..2)
                .set_line_pos(0..0)
        );
        assert_eq!(
            sc.next().unwrap(),
            WithPos::new(Token::new(TokenType::Eq))
                .set_byte_pos(2..3)
                .set_line_pos(0..0)
        );
        assert_eq!(
            sc.next().unwrap(),
            WithPos::new(Token::new(TokenType::Let))
                .set_byte_pos(3..6)
                .set_line_pos(0..0)
        );
        assert_eq!(
            sc.next().unwrap(),
            WithPos::new(Token::new(TokenType::Identifier).value("abc".to_owned()))
                .set_byte_pos(7..10)
                .set_line_pos(0..0)
        );
        assert_eq!(sc.next(), None);
    }
}
