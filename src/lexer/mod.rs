use crate::lexer::cursor::Cursor;
use crate::lexer::errors::LexError;
use crate::lexer::token::{Span, SpannedToken, Token};
use std::collections::HashMap;

pub(crate) mod cursor;
pub(crate) mod errors;
pub(crate) mod tests;
pub(crate) mod token;

#[allow(dead_code)]
pub(crate) struct Lexer<'cursor> {
    line: usize,
    column: usize,
    cursor: Cursor<'cursor>,
    tokens: Vec<SpannedToken>,
    keywords: HashMap<&'static str, Token>,
}

#[allow(unused)]
#[allow(dead_code)]
impl<'cursor> Lexer<'cursor> {
    pub(crate) fn new(code: &'cursor [char]) -> Self {
        Lexer {
            line: 0,
            column: 0,
            cursor: Cursor::new(code),
            tokens: Vec::new(),
            keywords: HashMap::from([
                ("fun", Token::Fun),
                ("for", Token::For),
                ("match", Token::Match),
                ("use", Token::Use),
                ("struct", Token::Struct),
                ("enum", Token::Enum),
                ("let", Token::Let),
                ("pub", Token::Pub),
                ("priv", Token::Priv),
                ("module", Token::Mod),
            ]),
        }
    }

    pub(crate) fn tokenize(&mut self) -> Result<Vec<SpannedToken>, LexError> {
        while !self.cursor.is_at_end() {
            let current = self.advance();
            match current? {
                ':' => self.add_tk(Token::Colon),
                ';' => self.add_tk(Token::Semicolon),
                '+' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::CompoundPlus)
                    } else {
                        self.add_tk(Token::Plus)
                    }
                }
                '-' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::CompoundMinus)
                    } else {
                        self.add_tk(Token::Minus)
                    }
                }
                '*' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::CompoundStar)
                    } else {
                        self.add_tk(Token::Star)
                    }
                }
                '/' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::CompoundSlash)
                    } else if next.is_some() && next.unwrap() == '/' {
                        todo!()
                    } else {
                        self.add_tk(Token::Slash)
                    }
                }
                '&' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '&' {
                        self.add_tk(Token::And)
                    } else {
                        todo!()
                    }
                }
                '|' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '|' {
                        self.add_tk(Token::Or)
                    } else {
                        todo!()
                    }
                }
                '^' => self.add_tk(Token::Xor),
                '[' => self.add_tk(Token::LeftSquareBracket),
                ']' => self.add_tk(Token::RightSquareBracket),
                '(' => self.add_tk(Token::LeftParen),
                ')' => self.add_tk(Token::RightParen),
                '{' => self.add_tk(Token::LeftBrace),
                '}' => self.add_tk(Token::RightBrace),
                '.' => self.add_tk(Token::Dot),
                '!' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::NotEquals)
                    } else {
                        self.add_tk(Token::Not)
                    }
                }
                '=' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::Equals)
                    } else {
                        self.add_tk(Token::Assign)
                    }
                }
                '>' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::GreaterEquals)
                    } else {
                        self.add_tk(Token::Greater)
                    }
                }
                '<' => {
                    let next = self.cursor.peek();
                    if next.is_some() && next.unwrap() == '=' {
                        self.add_tk(Token::LessEquals)
                    } else {
                        self.add_tk(Token::Less)
                    }
                }
                ',' => self.add_tk(Token::Comma),
                '"' => {
                    let string = self.tokenize_string()?;
                    self.add_tk(string)
                }
                ' ' => self.column += 1,
                '\t' => self.column += 1,
                '\n' => self.line += 1,
                '\0' => self.column += 1,
                ch => {
                    if ch.is_digit(10) {
                        let number;
                        if ch == '0' && self.cursor.offset_result(0)? == 'x' {
                            number = self.tokenize_hexadecimal_number()?;
                        } else if ch == '0' && self.cursor.offset_result(0)? == 'b' {
                            number = self.tokenize_binary_number()?;
                        } else if ch == '0' && self.cursor.offset_result(0)? == 'o' {
                            number = self.tokenize_octal_number()?;
                        } else {
                            number = self.tokenize_number(ch)?;
                        }
                        self.add_spanned_tk(number);
                    } else if ch.is_alphabetic() || ch == '_' {
                        let id = self.tokenize_id(ch)?;
                        self.add_spanned_tk(id);
                    }
                }
            }
        }
        Ok(self.tokens.drain(..).collect())
    }

    pub(crate) fn cleanup(&mut self) {
        self.cursor.current = 0;
        self.tokens.clear();
    }

    fn tokenize_string(&mut self) -> Result<Token, LexError> {
        // todo add utf-8 sequences tokenizing
        let mut text = String::new();
        while !self.cursor.is_at_end() && self.cursor.offset_result(0)? != '"' {
            text.push(self.advance()?);
        }
        self.advance()?;
        Ok(Token::String(text))
    }

    fn advance(&mut self) -> Result<char, LexError> {
        let ch: Option<char> = self.cursor.peek();
        match ch {
            None => Err(LexError::UnexpectedEof),
            Some(char) => {
                self.cursor.current += 1;
                self.column += 1;
                Ok(char)
            }
        }
    }

    fn add_tk(&mut self, tk: Token) {
        self.tokens
            .push(SpannedToken::new_single_char(tk, self.cursor.current));
    }

    fn add_spanned_tk(&mut self, tk: SpannedToken) {
        self.tokens.push(tk);
    }

    fn tokenize_hexadecimal_number(&mut self) -> Result<SpannedToken, LexError> {
        let start = self.cursor.current - 1;
        self.advance()?; // x
        let mut number = String::new();
        while !self.cursor.is_at_end() && self.cursor.offset_result(0)?.is_digit(16) {
            number.push(self.advance()?);
        }
        let end = self.cursor.current;
        Ok(SpannedToken::new(
            Token::Integer(i64::from_str_radix(&number, 16)?),
            Span::new(start, end),
        ))
    }

    fn tokenize_binary_number(&mut self) -> Result<SpannedToken, LexError> {
        let start = self.cursor.current - 1;
        self.advance()?; // b
        let mut number = String::new();
        while !self.cursor.is_at_end() && self.cursor.offset_result(0)?.is_digit(2) {
            number.push(self.advance()?);
        }
        let end = self.cursor.current;
        Ok(SpannedToken::new(
            Token::Integer(i64::from_str_radix(&number, 2)?),
            Span::new(start, end),
        ))
    }

    fn tokenize_octal_number(&mut self) -> Result<SpannedToken, LexError> {
        let start = self.cursor.current - 1;
        self.advance()?; // o
        let mut number = String::new();
        while !self.cursor.is_at_end() && self.cursor.offset_result(0)?.is_digit(8) {
            number.push(self.advance()?);
        }
        let end = self.cursor.current;
        Ok(SpannedToken::new(
            Token::Integer(i64::from_str_radix(&number, 8)?),
            Span::new(start, end),
        ))
    }

    fn tokenize_number(&mut self, ch: char) -> Result<SpannedToken, LexError> {
        let start = ch.len_utf8();
        let mut number = String::from(ch);
        let mut is_decimal = false;
        loop {
            if self.cursor.is_at_end() { break }
            let char = self.advance()?;
            if char.is_digit(10) || char == '.' {
                if char == '.' {
                    if is_decimal {
                        return Err(LexError::FailedToParseNumber);
                    }
                    is_decimal = true;
                }
                number.push(char);
            } else {
                break;
            }
        }
        let end = self.cursor.current;
        if is_decimal {
            Ok(SpannedToken::new(
                Token::Float(number.parse::<f64>()?),
                Span::new(start, end),
            ))
        } else {
            Ok(SpannedToken::new(
                Token::Integer(number.parse::<i64>()?),
                Span::new(start, end),
            ))
        }
    }

    fn tokenize_id(&mut self, ch: char) -> Result<SpannedToken, LexError> {
        let start = ch.len_utf8();
        let mut id = String::from(ch);
        loop {
            if self.cursor.is_at_end() { break }
            else {
                match self.cursor.peek() {
                    None => return Err(LexError::UnexpectedEof),
                    Some(char) => {
                        if char.is_alphabetic() || char == '_' {
                            id.push(self.advance()?);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        let end = self.cursor.current;
        if let Some(keyword) = self.keywords.get(id.as_str()) {
            Ok(SpannedToken::new(keyword.clone(), Span::new(start, end)))
        } else {
            Ok(SpannedToken::new(
                Token::Identifier(id),
                Span::new(start, end),
            ))
        }
    }
}
