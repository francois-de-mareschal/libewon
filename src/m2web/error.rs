use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    /// HTTP error code returned by the M2Web API.
    code: u16,
    /// Store the error kind associated to the HTTP status code and the message returned by the M2Web API.
    kind: ErrorKind,
}

impl error::Error for Error {}

/// Display the HTTP status code and the error message returned by the M2Web API.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidCredentials(ref error_message) => {
                write!(f, "HTTP {}: {}", self.code, error_message)
            }
        }
    }
}

/// Enumerate all kinds of error that could occur.
#[derive(Debug)]
pub enum ErrorKind {
    /// This error occurs when one of the authentication parameters provided to the M2Web API is wrong.
    InvalidCredentials(String),
}
