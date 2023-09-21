use std::error::Error;
use std::fmt::{ Display, Formatter };

#[derive(Debug, Default)]
pub struct ParserErrors {
    pub errors: Vec<String>,
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

    pub fn push_err(&mut self, err: String) {
        self.errors.push(err);
    }

    pub fn append_errs(&mut self, mut errors: Vec<String>) {
        self.errors.append(&mut errors);
    }
}

