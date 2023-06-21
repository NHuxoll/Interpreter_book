use crate::token::token::Token;

trait Node  {
    fn TokenLiteral(&self) -> String;
}

trait Statement: Node {
    fn statementNode(&self);
}

trait Expression: Node {
    fn expressionNode(&self);
}


struct Program {
    statements: Vec<Box<dyn Statement>>,
}


impl Node for Program {
    fn TokenLiteral(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].TokenLiteral().to_owned();
        } else {
            return "".to_owned();
        }
    }
}

struct LetStatement {
    Token: Token,
    Name: Identifier,
    Value: dyn Expression,
}

//impl Statement for LetStatement {
//    fn statementNode(&self) {
        
//    }
//}

//impl Node for LetStatement {
    //fn TokenLiteral(&self) -> String {
 ////       return Token::Let;
    //}
//}

struct Identifier {
    Token: Token,
    Value: String,
}
