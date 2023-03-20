use crate::{
    error::Error,
    error::ErrorKind,
    token::{self, Line, Number, Token, TokenKind, TokenValue},
};

pub type Index = usize;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a [u8], // The consumed line
    start: Index,     // Start of each token
    current: Index,   // Current char position in the line
    line: Line,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = vec![];
        let mut errors = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => {}
                Err(error) => errors.push(error),
            }
        }

        tokens.push(token::Token::new(
            token::TokenKind::Eof,
            String::new(),
            token::TokenValue::String(String::new()),
            self.line,
        ));

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        let c = self.advance();

        let empty_value = TokenValue::String(String::new());

        match c {
            // Skip unnecessary lines
            b' ' | b'\r' | b'\t' => Ok(None),
            b'\n' => {
                self.advance_line();
                Ok(None)
            }
            // Basic operations & chars
            b'(' => Ok(Some(self.make_token(TokenKind::LeftParen, empty_value))),
            b')' => Ok(Some(self.make_token(TokenKind::RightParen, empty_value))),
            b'{' => Ok(Some(self.make_token(TokenKind::LeftBrace, empty_value))),
            b'}' => Ok(Some(self.make_token(TokenKind::RightBrace, empty_value))),
            b',' => Ok(Some(self.make_token(TokenKind::Comma, empty_value))),
            b'.' => Ok(Some(self.make_token(TokenKind::Dot, empty_value))),
            b';' => Ok(Some(self.make_token(TokenKind::Semicolon, empty_value))),
            b'+' => Ok(Some(self.make_token(TokenKind::Plus, empty_value))),
            b'-' => Ok(Some(self.make_token(TokenKind::Minus, empty_value))),
            b'*' => Ok(Some(self.make_token(TokenKind::Star, empty_value))),
            // Multiple Characters
            b'!' => {
                let mut token = TokenKind::Bang;
                if self.matches(b'=') {
                    token = TokenKind::BangEqual;
                }
                Ok(Some(self.make_token(token, empty_value)))
            }
            b'=' => {
                let mut token = TokenKind::Equal;
                if self.matches(b'=') {
                    token = TokenKind::EqualEqual;
                }
                Ok(Some(self.make_token(token, empty_value)))
            }
            b'>' => {
                let mut token = TokenKind::Greater;
                if self.matches(b'=') {
                    token = TokenKind::GreaterEqual;
                }
                Ok(Some(self.make_token(token, empty_value)))
            }
            b'<' => {
                let mut token = TokenKind::Less;
                if self.matches(b'=') {
                    token = TokenKind::LessEqual;
                }
                Ok(Some(self.make_token(token, empty_value)))
            }
            // Slash and Comments
            b'/' => {
                if self.matches(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(self.make_token(TokenKind::Slash, empty_value)))
                }
            }
            // String Literals
            b'"' => Ok(Some(self.string())),
            _ => {
                // Number Literal
                if c.is_ascii_digit() {
                    Ok(Some(self.number()))
                } else if c.is_ascii_alphanumeric() {
                    Ok(Some(self.identifier()))
                } else {
                    Err(Error {
                        kind: ErrorKind::UnexpectedCharacter,
                        line: self.line,
                        description: String::from(format!("Unexpected '{}' Character", c)),
                        start: self.start,
                        end: self.current,
                    })
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn advance_line(&mut self) {
        self.line += 1;
    }

    fn peek_offset(&self, offset: Index) -> u8 {
        *self.source.get(self.current + offset).unwrap_or(&b'\0')
    }

    fn peek(&self) -> u8 {
        self.peek_offset(0)
    }

    fn peek_next(&self) -> u8 {
        self.peek_offset(1)
    }

    fn make_token(&mut self, kind: TokenKind, value: TokenValue) -> Token {
        let word = self.substring(self.start, self.current);

        Token::new(kind, word, value, self.line)
    }

    fn substring(&self, start: Index, end: Index) -> String {
        String::from_utf8(self.source[start..end].to_vec()).unwrap_or_else(|source| {
            println!("Error: {}", start + source.utf8_error().valid_up_to());
            String::new()
        })
    }

    fn matches(&mut self, c: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != c {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            // Supports multi-line string values
            // We have to advance when we hit new line
            if self.peek() == b'\n' {
                self.advance_line();
            }

            self.advance();
        }

        // Must be closed in one line
        if self.is_at_end() {
            println!("Line {}: Missing closing quotation", self.line);
        }

        // Closing quote
        self.advance();
        let value = self.substring(self.start + 1, self.current - 1);
        // Add token with value
        self.make_token(TokenKind::String, TokenValue::String(value))
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            //consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: f64 = self
            .substring(self.start, self.current)
            .parse::<Number>()
            .map_err(|_| println!("Line {}: Invalid number", self.line))
            .unwrap();

        self.make_token(TokenKind::Number, TokenValue::Number(value))
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = self.substring(self.start, self.current);

        self.make_token(TokenKind::Identifier, TokenValue::String(text))
    }
}
