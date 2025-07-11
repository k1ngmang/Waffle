use std::{fs::File, io::Read};

use crate::lexer::token::SpannedToken;

pub(crate) mod token;

#[derive(Debug)]
pub enum LexError {
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    UnexpectedEof,
    IndexOutOfRange,
    ParseInt,
}

impl From<std::io::Error> for LexError {
    fn from(e: std::io::Error) -> Self {
        LexError::Io(e)
    }
}

impl From<std::str::Utf8Error> for LexError {
    fn from(e: std::str::Utf8Error) -> Self {
        LexError::Utf8(e)
    }
}

pub(crate) struct Lexer {
    pos: usize,
    
}

impl Lexer {
    pub(crate) fn tokenize<'lex>(file_path: &str) -> Result<Vec<SpannedToken<'lex>>, LexError> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let mut tokens = Vec::new();


        Ok(tokens)
    }
}
