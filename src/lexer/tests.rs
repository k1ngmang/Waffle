#[allow(unused)]
use crate::lexer::Lexer;
use crate::lexer::token::{SpannedToken, Token};

#[allow(dead_code)]
fn to_token_list(vector: Vec<SpannedToken>) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    for element in vector {
        result.push(element.token);
    }
    result
}

#[test]
pub fn number_tokenize() {
    let code = &"123456.7890".chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(code);
    let result = to_token_list(lexer.tokenize().unwrap());
    assert_eq!(result, vec![Token::Float(123456.7890)]);
}

#[test]
pub fn binary_number_tokenize() {
    let code = &"0b1010101".chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(code);
    let result = to_token_list(lexer.tokenize().unwrap());
    assert_eq!(result, vec![Token::Integer(85)]);
}

#[test]
pub fn octal_number_tokenize() {
    let code = &"0o7777".chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(code);
    let result = to_token_list(lexer.tokenize().unwrap());
    assert_eq!(result, vec![Token::Integer(4095)]);
}

#[test]
pub fn string_tokenize() {
    let code = &"\"hello, world!\"".chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(code);
    let result = to_token_list(lexer.tokenize().unwrap());
    assert_eq!(result, vec![Token::String("hello, world!".to_string())]);
}

#[test]
pub fn keywords_tokenize() {
    let code = &"match a _ for".chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(code);
    let result = to_token_list(lexer.tokenize().unwrap());
    assert_eq!(
        result,
        vec![
            Token::Match, Token::Identifier("a".to_string()),
            Token::Identifier("_".to_string()), Token::For
        ]
    );
}

#[test]
pub fn keywords_tokenize_2() {
    let code = &"fun a(): int {}".chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(code);
    let result = to_token_list(lexer.tokenize().unwrap());
    assert_eq!(
        result,
        vec![
            Token::Fun, Token::Identifier("a".to_string()),
            Token::LeftParen, Token::RightParen, Token::Colon,
            Token::Identifier("int".to_string()), Token::LeftBrace,
            Token::RightBrace
        ]
    );
}