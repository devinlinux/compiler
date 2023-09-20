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

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(stmt) => write!(f, "{}", stmt),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} value: {}", self.name, self.value)
    }
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

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Expression")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token: {} value: {}", self.token, self.value)
    }
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier {
            token,
            value,
        }
    }
}

