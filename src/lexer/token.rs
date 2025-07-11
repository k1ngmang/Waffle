#[derive(Debug, Clone)]
pub(crate) enum Token<'input> {
    Fun,
    Module,
    Use,
    Struct,
    LeftParen,
    RightParen,
    Priv,
    Pub,
    Colon,
    Semicolon,
    LeftBrace,
    RightBrace,
    DoubleQuote,
    Assign,
    Let,
    LeftSquareBracket,
    RightSquateBracket,
    Star,
    Slash,
    Plus,
    Minus,
    Match,
    And,
    Or,
    Xor,
    Enum,
    LeftArrow,
    RightArrow,
    Greater,
    Less,
    Equals,
    NotEquals,
    LessEquals,
    GreaterEquals,
    Comma,
    Dot,
    Not,
    Integer(i64),
    Float(f64),
    Identifier(&'input str),
    String(&'input str),
    Boolean(bool),
    Undefined
}   

#[derive(Debug, Clone)]
pub(crate) struct SpannedToken<'input> {
    pub(crate) token: Token<'input>,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}

impl<'input> SpannedToken<'input> {
    pub(crate) fn new(token: Token<'input>, span: Span) -> SpannedToken<'input> {
        SpannedToken { token, span }
    }

    pub(crate) fn new_single_char(token: Token<'input>, span_start: usize) -> SpannedToken<'input> {
        SpannedToken {
            token,
            span: Span::new(span_start, span_start + 1),
        }
    }
}
