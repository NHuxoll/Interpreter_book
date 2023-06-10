use crate::lexer::lexer::Lexer;
use crate::token::token::Token;

#[allow(dead_code)]
pub fn Start(){
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut tokenizer = Lexer::new(line);
            
            while let Ok(token) = tokenizer.next_token() {
                println!("{:?}", token);
                if let Token::Eof = token {
                    break;
                }
            }
        }
    });
    return;
}


