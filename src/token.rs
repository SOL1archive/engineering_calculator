use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    EoF(String),
    
    Plus(String),
    Minus(String),
    Multiply(String),
    Divide(String),

    Lparen(String),
    Rparen(String),

    Ident(String),
    
    Log(String),
    Exp(String),
}
