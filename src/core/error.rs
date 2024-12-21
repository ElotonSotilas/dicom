use thiserror::Error;

pub type DicomResult<T> = Result<T, DicomError>;

#[derive(Error, Debug)]
pub enum DicomError {
    #[error("Invalid tag: {0}")]
    InvalidTag(String),
    #[error("Invalid value representation: {0}")]
    InvalidVR(String),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Invalid length: {0}")]
    InvalidLength(String),
    #[error("Invalid file: {0}")]
    InvalidFile(String),
    #[error("Invalid dataset: {0}")]
    InvalidDataset(String),
    #[error("Obsolete element: {0}. Use a Force flag to allow legacy properties.")]
    ObsoleteElement(String),
    #[error("Syntax error: {0}")]
    SyntaxError(SyntaxErrorKind),
    #[error("IO error: {0}")]
    IOError(String),
    #[error("Unknown error: {0}")]
    Error(String),
}

#[derive(Error, Debug)]
pub enum SyntaxErrorKind {
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    #[error("Invalid string: {0}")]
    InvalidString(String),
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Invalid time: {0}")]
    InvalidTime(String),
    #[error("Invalid datetime: {0}")]
    InvalidDateTime(String),
    #[error("Unknown syntax error: {0}")]
    Error(String),
}

impl From<std::io::Error> for DicomError {
    fn from(error: std::io::Error) -> Self {
        DicomError::IOError(error.to_string())
    }
}

impl From<&(dyn std::error::Error + 'static)> for DicomError {
    fn from(error: &(dyn std::error::Error + 'static)) -> Self {
        DicomError::Error(error.to_string())
    }
}

pub trait OptionExt<T> {
    fn ok_or_err(self) -> Result<T, DicomError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_err(self) -> Result<T, DicomError> {
        self.ok_or(DicomError::Error("Unwrapped empty value.".to_string()))
    }
}