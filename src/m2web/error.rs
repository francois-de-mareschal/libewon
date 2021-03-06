use std::convert;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    /// HTTP error code returned by the M2Web API.
    pub(in crate::m2web) code: u16,
    /// Store the error kind associated to the HTTP status code and the message returned by the M2Web API.
    pub(in crate::m2web) kind: ErrorKind,
}

/// Enumerate all kinds of error that could occur.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    /// This error occurs when the API returns and empty response.
    EmptyResponse(String),
    /// This error occurs when an internal call goes wrong and return an error.
    InternalError(String),
    /// This error occurs when one of the authentication parameters provided to the M2Web API is wrong.
    InvalidCredentials(String),
    /// This error occurs when a mandatory parameter or endpoint is missing or wrong
    MissingOrWrongParameter(String),
    /// This error occurs when the API returns an empty response.
    NoContent(String),
    /// This error occurs when the API client is unable to parse and deserialize the JSON response from the API.
    ResponseParsing(String),
    /// This error occurs when the client tries to auth statefully without the stateful_auth field set.
    StatelessAuthSet(String),
    /// This is a generic error when an unknown error occurred.
    UnknownError(String),
}

impl error::Error for Error {}

/// Display the HTTP status code and the error message returned by the M2Web API.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::EmptyResponse(ref error_message) => {
                write!(f, "HTTP {}: {}", self.code, error_message)
            }
            ErrorKind::InternalError(ref error_message) => {
                write!(f, "Internal error: {}", error_message)
            }
            ErrorKind::InvalidCredentials(ref error_message) => {
                write!(f, "HTTP {}: {}", self.code, error_message)
            }
            ErrorKind::MissingOrWrongParameter(ref error_message) => {
                write!(f, "HTTP {}: {}", self.code, error_message)
            }
            ErrorKind::NoContent(ref error_message) => {
                write!(f, "HTTP {}: {}", self.code, error_message)
            }
            ErrorKind::ResponseParsing(ref error_message) => {
                write!(f, "Unable to parse JSON response: {}", error_message)
            }
            ErrorKind::StatelessAuthSet(ref error_message) => {
                write!(
                    f,
                    "Client set to authenticate statelessly: {}",
                    error_message
                )
            }
            ErrorKind::UnknownError(ref error_message) => {
                write!(f, "Unknown error: {}", error_message)
            }
        }
    }
}

/// Allow to transform reqwest::Error to m2web::Error.
impl convert::From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        match error.status() {
            Some(reqwest::StatusCode::FORBIDDEN) => Error {
                code: 403,
                kind: ErrorKind::InvalidCredentials(format!("{}", error)),
            },
            Some(_) | None => Error {
                code: 500,
                kind: ErrorKind::UnknownError(format!(
                    "Unknown error while requesting API: {}",
                    error
                )),
            },
        }
    }
}

/// Allow to transform serde_json::Error to m2web::Error.
impl convert::From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        if error.is_syntax() {
            Error {
                code: 500,
                kind: ErrorKind::ResponseParsing(format!("JSON response syntax error: {}", error)),
            }
        } else if error.is_data() {
            Error {
                code: 500,
                kind: ErrorKind::ResponseParsing(format!(
                    "JSON response data format does not match the expected one: {}",
                    error
                )),
            }
        } else if error.is_eof() {
            Error {
                code: 500,
                kind: ErrorKind::ResponseParsing(format!(
                    "An empty or incomplete response were received: {}",
                    error
                )),
            }
        } else {
            Error {
                code: 500,
                kind: ErrorKind::ResponseParsing(format!(
                    "Unknown error while parsing JSON response: {}",
                    error
                )),
            }
        }
    }
}
