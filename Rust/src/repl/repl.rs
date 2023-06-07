use crate::lexer::lexer::Lexer;
use crate::token::token::Token;
use std::io;

#[allow(dead_code)]
pub fn Start() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    while stdin.read_line(&mut buffer).is_ok() {
        let trimmed = buffer.trim_end();
        let mut lex = Lexer::new(trimmed.to_string());

        while let Ok(tok) = lex.next_token() {
            if tok != Token::Eof {
            println!("{:?}", tok);
            } else {
                break;
            }
        }
     }
}

