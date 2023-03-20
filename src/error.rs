#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub description: String,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedCharacter,
    InvalidString,
    InvalidNumber,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "Line {}: '{}' at {} ends at {}. More Context: {}",
                self.line, self.kind, self.start, self.end, self.description
            )
            .as_str(),
        )
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ErrorKind::InvalidNumber => f.write_str("Invalid Number"),
            ErrorKind::InvalidString => f.write_str("Invalid String"),
            ErrorKind::UnexpectedCharacter => f.write_str("Unexpected Character"),
        }
    }
}
