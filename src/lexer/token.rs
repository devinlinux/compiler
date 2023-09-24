use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(u8),
    Eof,

    //  identifiers + literals
    Ident(String),
    Int(String),
    Float(String),

    //  operators
    Assign,
    Plus,
    Dash,
    Asterisk,
    Slash,
    Bang,
    QMark,
    Eq,
    NotEq,
    ReturnOp,

    //  delimiters
    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LSquirly,
    RSquirly,

    //  keywords
    Function,
    Var,
    Const,
    Return,
    If,
    Else,
    True,
    False,

    //  types
    Null,
    Bool,
    I8,
    I16,
    I32,
    I64,
    F8,
    F16,
    F32,
    F64,
    U8,
    StringTok,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Illegal(tok) => write!(f, "Illegal({})", tok),
            Token::Eof => write!(f, "Eof"),
            Token::Ident(ident) => write!(f, "Ident({})", ident),
            Token::Int(int) => write!(f, "Int({})", int),
            Token::Float(float) => write!(f, "Float({})", float),
            Token::Assign => write!(f, "Assign"),
            Token::Plus => write!(f, "Plus"),
            Token::Dash => write!(f, "Dash"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::Bang => write!(f, "Bang"),
            Token::QMark => write!(f, "QMark"),
            Token::Eq => write!(f, "Eq"),
            Token::NotEq => write!(f, "Not Eq"),
            Token::ReturnOp => write!(f, "Return Op"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Colon => write!(f, "Colon"),
            Token::LParen => write!(f, "LParen"),
            Token::RParen => write!(f, "RParen"),
            Token::LBracket => write!(f, "LBracket"),
            Token::RBracket => write!(f, "RBracket"),
            Token::LSquirly => write!(f, "LSquirly"),
            Token::RSquirly => write!(f, "RSquirly"),
            Token::Function => write!(f, "fn"),
            Token::Var => write!(f, "var"),
            Token::Const => write!(f, "const"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            Token::Bool => write!(f, "bool"),
            Token::I8 => write!(f, "i8"),
            Token::I16 => write!(f, "i16"),
            Token::I32 => write!(f, "i32"),
            Token::I64 => write!(f, "i64"),
            Token::F8 => write!(f, "f8"),
            Token::F16 => write!(f, "f16"),
            Token::F32 => write!(f, "f32"),
            Token::F64 => write!(f, "f64"),
            Token::U8 => write!(f, "u8"),
            Token::StringTok => write!(f, "String"),
        }
    }
}
