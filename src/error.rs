#[derive(Debug)]
pub enum Error {
    ParsingError(String),
    FormattingError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParsingError(e) => write!(f, "Parsing error: {}", e),
            Error::FormattingError(e) => write!(f, "Formatting error error: {}", e)
        }
    }
}

impl std::error::Error for Error {}