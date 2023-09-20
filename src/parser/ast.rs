use std::fmt::Display;

use crate::lexer::Token;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program {
            statements,
        }
    }
}

pub enum Statement {
    Let(LetStatement),
}

pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl LetStatement {
    pub fn new(name: Identifier) -> LetStatement {
        LetStatement {
            name,
            value: Expression::Blank,
        }
    }
}

//  FIXME: add values
pub enum Expression {
    Blank, 
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

