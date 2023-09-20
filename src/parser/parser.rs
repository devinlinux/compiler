use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::parser::ast;

pub struct Parser {
    lexer: Lexer,
    curr_tkn: Token,
    peek_tkn: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser: Parser = Parser {
            lexer,
            curr_tkn: Token::Illegal(0),
            peek_tkn: Token::Illegal(0),
        };

        parser.next();
        parser.next();

        parser
    }

    pub fn next(&mut self) {
        self.curr_tkn = self.peek_tkn.clone();
        self.peek_tkn = self.lexer.next();
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program: ast::Program = ast::Program::new(Vec::new());

        while self.curr_tkn != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        return match self.curr_tkn {
            Token::Var | Token::Const => self.parse_let_stmt().map(ast::Statement::Let),
            _ => None,
        };
    }

    fn parse_let_stmt(&mut self) -> Option<ast::LetStatement> {
        if !self.expect_peek(&Token::Ident(String::new())) {
            return None;
        }

        let name = match self.curr_tkn.clone() {
            Token::Ident(value) => ast::Identifier {
                token: self.curr_tkn.clone(),
                value,
            },
            _ => unreachable!(),
        };

        if !self.expect_peek(&Token::Assign) {
            return None;
        }

        // TODO: Do expressions, currently skipping until semicolon is encountered

        while !self.curr_tok_is(&Token::Semicolon) {
            self.next();
        }

        Some(ast::LetStatement::new(name))
    }

    fn curr_tok_is(&self, token: &Token) -> bool {
        match self.curr_tkn {
            Token::Ident(_) => matches!(token, Token::Ident(_)),
            Token::Int(_) => matches!(token, Token::Int(_)),
            Token::Float(_) => matches!(token, Token::Float(_)),
            _ => &self.curr_tkn == token
        }
    }

    fn peek_tok_is(&self, token: &Token) -> bool {
        match self.peek_tkn {
            Token::Ident(_) => matches!(token, Token::Ident(_)),
            Token::Int(_) => matches!(token, Token::Int(_)),
            Token::Float(_) => matches!(token, Token::Float(_)),
            _ => &self.peek_tkn == token
        }
    }

    fn expect_peek(&mut self, token: &Token) -> bool {
        if self.peek_tok_is(token) {
            self.next();
            true
        } else {
            false
        }
    }
}

