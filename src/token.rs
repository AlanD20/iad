pub type Number = f64;
pub type Line = usize;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub word: String,
    pub value: TokenValue,
    pub line: Line,
}

impl Token {
    pub fn new(kind: TokenKind, word: String, value: TokenValue, line: Line) -> Token {
        Token {
            kind,
            word,
            value,
            line,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Bool,
    Identifier,
    Number,
    String,

    // Keywords/Identifiers
    And,
    Class,
    Const,
    Else,
    Extends,
    False,
    For,
    Func,
    If,
    Let,
    Null,
    Or,
    Print,
    Return,
    This,
    True,

    Eof,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    Null,
    Bool(bool),
    Identifier(String),
    Number(Number),
    String(String),
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenKind::LeftParen => f.write_str("("),
            TokenKind::RightParen => f.write_str(")"),
            TokenKind::LeftBrace => f.write_str("{"),
            TokenKind::RightBrace => f.write_str("}"),
            TokenKind::Comma => f.write_str(","),
            TokenKind::Dot => f.write_str("."),
            TokenKind::Minus => f.write_str("-"),
            TokenKind::Plus => f.write_str("+"),
            TokenKind::Semicolon => f.write_str(";"),
            TokenKind::Slash => f.write_str("/"),
            TokenKind::Star => f.write_str("*"),

            TokenKind::Bang => f.write_str("!"),
            TokenKind::BangEqual => f.write_str("!="),
            TokenKind::Equal => f.write_str("="),
            TokenKind::EqualEqual => f.write_str("=="),
            TokenKind::Greater => f.write_str(">"),
            TokenKind::GreaterEqual => f.write_str(">="),
            TokenKind::Less => f.write_str("<"),
            TokenKind::LessEqual => f.write_str("<="),

            TokenKind::Bool => f.write_str("<bool>"),
            TokenKind::Identifier => f.write_str("<identifier>"),
            TokenKind::Number => f.write_str("<number>"),
            TokenKind::String => f.write_str("<string>"),

            TokenKind::And => f.write_str("and"),
            TokenKind::Class => f.write_str("class"),
            TokenKind::Const => f.write_str("const"),
            TokenKind::Else => f.write_str("else"),
            TokenKind::Extends => f.write_str("extends"),
            TokenKind::False => f.write_str("false"),
            TokenKind::For => f.write_str("for"),
            TokenKind::Func => f.write_str("func"),
            TokenKind::If => f.write_str("if"),
            TokenKind::Let => f.write_str("let"),
            TokenKind::Null => f.write_str("null"),
            TokenKind::Or => f.write_str("or"),
            TokenKind::Print => f.write_str("print"),
            TokenKind::Return => f.write_str("return"),
            TokenKind::This => f.write_str("this"),
            TokenKind::True => f.write_str("true"),
            TokenKind::Eof => f.write_str("\\d"),
        }
    }
}

impl std::fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenValue::Identifier(s) => f.write_str(s),
            TokenValue::String(s) => s.fmt(f),
            TokenValue::Number(n) => n.fmt(f),
            TokenValue::Bool(n) => n.fmt(f),
            TokenValue::Null => f.write_str(""),
        }
    }
}
