use crate::lexer::Token;

pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            pos: 0,
            read_pos: 0,
            ch: 0,
        };
        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    pub fn next(&mut self) -> Token {
        self.eat_whitespace();

        let tok: Token = match self.ch {
            0 => Token::Eof,
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            },
            b'+' => Token::Plus,
            b'-' => {
                if self.peek() == b'>' {
                    self.read_char();
                    Token::ReturnOp
                } else {
                    Token::Dash
                }
            },
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            },
            b'?' => Token::QMark,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b':' => Token::Colon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return match ident.as_str() {
                    "fn" => Token::Function,
                    "var" => Token::Var,
                    "const" => Token::Const,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "true" => Token::True,
                    "false" => Token::False,
                    "i8" => Token::I8,
                    "i16" => Token::I16,
                    "i32" => Token::I32,
                    "i64" => Token::I64,
                    "f8" => Token::F8,
                    "f16" => Token::F16,
                    "f32" => Token::F32,
                    "f64" => Token::F64,
                    "u8" => Token::U8,
                    "bool" => Token::Bool,
                    "null" => Token::Null,
                    "String" => Token::StringTok,
                    _ => Token::Ident(ident),
                };
            },
            b'0'..=b'9' => {
                let num = self.read_num();
                if num.contains(".") {
                    return Token::Float(num)
                } else {
                    return Token::Int(num)
                }
            }
            _ => Token::Illegal(self.ch),
        };
        
        self.read_char();
        return tok;
    }

    fn read_ident(&mut self) -> String {
        let start = self.pos;
        while is_valid_var_char(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[start..self.pos]).to_string()
    }

    fn read_num(&mut self) -> String {
        let start = self.pos;
        while is_valid_num_char(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[start..self.pos]).to_string()
    }

    fn eat_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek(&self) -> u8 {
        if self.read_pos >= self.input.len() {
            0
        } else {
            self.input[self.read_pos]
        }
    }

    fn has_tokens(&self) -> bool {
        self.ch != 0
    }
}

fn is_valid_var_char(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'_'
}

fn is_valid_num_char(ch: u8) -> bool {
    ch.is_ascii_digit() || ch == b'.' || ch == b'_'
}

#[test]
fn text_next_token() -> Result<(), ()> {
    let input = r#"
        const x: i32 = 5;
        const y: f64 = 4.2;
        var z: ?u8 = null;
        ? ! != == :
        if (x != y) {
            x = y;
        }
        "#;
    let mut lex = Lexer::new(input.into());

    let tokens = vec![
        Token::Const,
        Token::Ident(String::from("x")),
        Token::Colon,
        Token::I32,
        Token::Assign,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Const,
        Token::Ident(String::from("y")),
        Token::Colon,
        Token::F64,
        Token::Assign,
        Token::Float(String::from("4.2")),
        Token::Semicolon,
        Token::Var,
        Token::Ident(String::from("z")),
        Token::Colon,
        Token::QMark,
        Token::U8,
        Token::Assign,
        Token::Null,
        Token::Semicolon,
        Token::QMark,
        Token::Bang,
        Token::NotEq,
        Token::Eq,
        Token::Colon,
        Token::If,
        Token::LParen,
        Token::Ident(String::from("x")),
        Token::NotEq,
        Token::Ident(String::from("y")),
        Token::RParen,
        Token::LSquirly,
        Token::Ident(String::from("x")),
        Token::Assign,
        Token::Ident(String::from("y")),
        Token::Semicolon,
        Token::RSquirly,
    ];

    for token in tokens {
        let next_token = lex.next();
        println!("expected: {}, received: {}", token, next_token);
        assert_eq!(token, next_token);
    }

    Ok(())
}
