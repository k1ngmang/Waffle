#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub(crate) enum Token {
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
    Assign,
    CompoundPlus,
    CompoundMinus,
    CompoundStar,
    CompoundSlash,
    CompoundMod,
    Let,
    LeftSquareBracket,
    RightSquareBracket,
    Star,
    Slash,
    Plus,
    Minus,
    Mod,
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
    Identifier(String),
    String(String),
    Boolean(bool),
    For,
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

#[derive(Debug, Clone)]
pub(crate) struct SpannedToken {
    pub(crate) token: Token,
    pub(crate) span: Span,
}

impl SpannedToken {
    pub(crate) fn new(token: Token, span: Span) -> SpannedToken {
        SpannedToken { token, span }
    }

    pub(crate) fn new_single_char(token: Token, span_start: usize) -> SpannedToken {
        SpannedToken {
            token,
            span: Span::new(span_start, span_start + 1),
        }
    }
}
