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
            Token::Function => write!(f, "Function"),
            Token::Var => write!(f, "Var"),
            Token::Const => write!(f, "Const"),
            Token::Return => write!(f, "Return"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Null => write!(f, "Null"),
            Token::Bool => write!(f, "Bool"),
            Token::I8 => write!(f, "I8"),
            Token::I16 => write!(f, "I16"),
            Token::I32 => write!(f, "I32"),
            Token::I64 => write!(f, "I64"),
            Token::F8 => write!(f, "F8"),
            Token::F16 => write!(f, "F16"),
            Token::F32 => write!(f, "F32"),
            Token::F64 => write!(f, "F64"),
            Token::U8 => write!(f, "U8"),
            Token::StringTok => write!(f, "String"),
        }
    }
}
