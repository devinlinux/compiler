use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl PartialEq for LetStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
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
#[derive(Debug, PartialEq)]
pub enum Expression {
    Blank, 
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier {
            token,
            value,
        }
    }
}

