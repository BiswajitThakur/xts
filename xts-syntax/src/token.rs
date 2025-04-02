use scanny::{Scanny, WithPos};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
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
    Identifier,
    Int,
    Float,
    BigInt,
    String,
    Boolean,

    // Operator
    Plus,      // +
    PlusEq,    // +=
    Minus,     // -
    MinusEq,   // -=
    Star,      // *
    StarEq,    // *=
    Slash,     // /
    SlashEq,   // /=
    Percent,   // %
    PercentEq, // %=
    Eq,        // =
    EqEq,      // ==
    At,        // @
    Lt,        // <
    LtEq,      // <=
    Gt,        // >
    GtEq,      // >=
    And,       // &
    AndAnd,    // &&
    Pipe,      // | // TODO: rename me
    Or,        // ||
    Bang,      // !
    NotEq,     // !=

    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    DoubleColon, // ::
    Dot,         // .
    DoubleDot,   // ..
    Arrow,       // ->
    Underscore,  // _

    // Parentheses & Braces
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]

    InvalidToken,

    // End of File
    Eof,
    // TODO: add more tokens
}

pub struct Token<'a> {
    token_type: TokenType,
    value: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(t: TokenType, value: &'a str) -> Self {
        Self {
            token_type: t,
            value,
        }
    }
}

pub struct Tokenizer<'a> {
    sc: Scanny<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            sc: Scanny::from(value),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = WithPos<Token<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.sc.skeep_while(char::is_whitespace);
            let peek = self.sc.peek();
            if peek.is_none() {
                return None;
            }
            match peek.unwrap() {
                'a'..='z' | 'A'..='Z' | '_' => return consume_keywords(&self.sc),
                '0'..='9' => return consume_number(&self.sc),
                '(' => return consume_single_char_token(&self.sc, TokenType::OpenParen),
                ')' => return consume_single_char_token(&self.sc, TokenType::CloseParen),
                '{' => return consume_single_char_token(&self.sc, TokenType::OpenBrace),
                '}' => return consume_single_char_token(&self.sc, TokenType::CloseBrace),
                '[' => return consume_single_char_token(&self.sc, TokenType::OpenBracket),
                ']' => return consume_single_char_token(&self.sc, TokenType::CloseBracket),
                ',' => return consume_single_char_token(&self.sc, TokenType::Comma),
                _ => todo!(),
            }
        }
        todo!()
    }
}

fn consume_single_char_token<'a>(sc: &Scanny<'a>, t: TokenType) -> Option<WithPos<Token<'a>>> {
    sc.matcher()
        .match_char(|_| true)
        .finalize(|v| Token::new(t, v.value()))
}

fn consume_number<'a>(sc: &Scanny<'a>) -> Option<WithPos<Token<'a>>> {
    sc.matcher()
        .match_char(|v| v.is_ascii_digit())
        .consume_while(|v| v.is_ascii_digit() || v == &'_')
        .then_peek(|v| match v.peek() {
            Some('.') => {
                v.bump();
                v.consume_while(|v| v.is_ascii_digit() || v == &'_');
                true
            }
            Some(_) => false,
            None => true,
        })
        .finalize(|v| {
            if v.is_not_matched() {
                return Token::new(TokenType::InvalidToken, v.value());
            }
            if v.value().contains('.') {
                Token::new(TokenType::Float, v.value())
            } else {
                Token::new(TokenType::Int, v.value())
            }
        })
}

fn consume_keywords<'a>(sc: &Scanny<'a>) -> Option<WithPos<Token<'a>>> {
    sc.matcher()
        .match_char(|v| matches!(*v, 'a'..='z' | 'A'..='Z' | '_'))
        .consume_while(|v| v.is_ascii_alphabetic() || v.is_ascii_digit() || v == &'_')
        .finalize(|v| {
            if v.is_not_matched() {
                return Token::new(TokenType::InvalidToken, v.value());
            }
            let t = match v.value() {
                "_" => TokenType::Underscore,
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
            Token::new(t, v.value())
        })
}
