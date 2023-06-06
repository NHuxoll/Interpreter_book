
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),

    Illegal,
    Eof,
    Assign,

    Bang,
    Dash,
    ForwardSlash,
    Asterisk,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    Comma,
    Semicolon,
    Plus,
    Minus,
    LParen,
    RParen,
    LSquirly,
    RSquirly,

    Function,
    Let,
    
    If,
    Else,
    Return,
    True,
    False
}
