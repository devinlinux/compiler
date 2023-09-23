use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::parser::{ ast, parser_errors::{ ParserErrors, ParserError } };

pub struct Parser {
    lexer: Lexer,
    curr_tkn: Token,
    peek_tkn: Token,
    errors: ParserErrors
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser: Parser = Parser {
            lexer,
            curr_tkn: Token::Illegal(0),
            peek_tkn: Token::Illegal(0),
            errors: ParserErrors::default(),
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
            Token::Ident(value) => ast::Identifier::new(self.curr_tkn.clone(), value),
            _ => unreachable!(),
        };

        if !self.expect_peek(&Token::Assign) {
            self.errors.push_err(ParserError::PeekError(Token::Assign, self.peek_tkn.clone()));
            println!("{}", self.errors);
            std::process::exit(1);
        }

        //  TODO: Do expressions, currently skipping until semicolon is encountered
        //  TODO: types
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

#[test]
fn parse_let_statements_test() {
    let input = r#"
        const x = 5;
        var y = 5.4;
        "#;
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    if program.statements.len() != 2 {
        panic!("Incorrect number of statements");
    }

    let statements: Vec<ast::Statement> = vec![
        ast::Statement::Let(ast::LetStatement::new(ast::Identifier::new(Token::Ident(String::from("x")), String::from("x")))),
        ast::Statement::Let(ast::LetStatement::new(ast::Identifier::new(Token::Ident(String::from("y")), String::from("y")))),
    ];

    let mut i: usize = 0;
    for statement in statements {
        let got = program.statements.get(i);
        i += 1;

        println!("expected: {}, got: {}", statement, got.unwrap());
        assert_eq!(&statement, got.unwrap());
    }
}

