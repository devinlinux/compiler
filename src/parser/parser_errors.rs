use std::error::Error;
use std::fmt::{ Display, Formatter };

use crate::lexer::Token;

#[derive(Debug, Default)]
pub struct ParserErrors {
    pub errors: Vec<ParserError>,
}

impl Error for ParserErrors {
}

impl Display for ParserErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { 
        writeln!(f, "Parser errors:")?;
        for err in &self.errors {
            writeln!(f, "\t{err}")?;
        }
        Ok(())
    }
}

impl ParserErrors {
    pub fn new() -> ParserErrors {
        ParserErrors {
            errors: Vec::new(),
        }
    }

    pub fn push_err(&mut self, err: ParserError) {
        self.errors.push(err);
    }

    pub fn append_errs(&mut self, mut errors: Vec<ParserError>) {
        self.errors.append(&mut errors);
    }
}

#[derive(Debug)]
pub enum ParserError {
    PeekError(Token, Token),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::PeekError(expected, got) => write!(f, "Expected: {}, Got: {} instead", expected, got),
        }
    }
}

