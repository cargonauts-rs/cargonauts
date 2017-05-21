use std::fmt;
use std::io;
use std::error::Error as ErrorT;

use backtrace::Backtrace;

use http::StatusCode;

pub trait HasStatusCode {
    fn status_code(&self) -> StatusCode;
}

pub struct Error {
    code: StatusCode,
    backtrace: Backtrace,
    cause: Cause,
}

enum Cause {
    Error(Box<ErrorT + Send + 'static>),
    Message(String),
    Unknown,
}

impl Error {
    pub fn new(code: StatusCode) -> Self {
        Self::construct(code, Backtrace::new(), Cause::Unknown)
    }

    pub fn with_msg<S: Into<String>>(code: StatusCode, msg: S) -> Self {
        Self::construct(code, Backtrace::new(), Cause::Message(msg.into()))
    }

    pub fn from_err<E: ErrorT + Send + 'static>(err: E, code: StatusCode) -> Self {
        Self::with_backtrace(err, code, Backtrace::new())
    }

    pub fn with_backtrace<E: ErrorT + Send + 'static>(err: E, code: StatusCode, backtrace: Backtrace) -> Self {
        Self::construct(code, backtrace, Cause::Error(Box::new(err)))
    }

    fn construct(code: StatusCode, backtrace: Backtrace, cause: Cause) -> Self {
        Self { code, backtrace, cause }
    }
    
    pub fn status_code(&self) -> StatusCode {
        self.code
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    pub fn cause(&self) -> Option<&ErrorT> {
        match self.cause {
            Cause::Error(ref err)   => Some(&**err as &ErrorT),
            _                       => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::from_err(err, StatusCode::InternalServerError)
    }
}

impl HasStatusCode for Error {
    fn status_code(&self) -> StatusCode {
        self.code
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Cause::Error(ref cause) => write!(f, "{}\nINFO: {}\n\n{:?}", self.code, cause, self.backtrace),
            Cause::Message(ref msg) => write!(f, "{}\nINFO: {}\n\n{:?}", self.code, msg, self.backtrace),
            Cause::Unknown          => write!(f, "{}\n\n{:?}", self.code, self.backtrace),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Cause::Error(ref cause) => write!(f, "{}\nINFO: {}", self.code, cause),
            Cause::Message(ref msg) => write!(f, "{}\nINFO: {}", self.code, msg),
            Cause::Unknown          => write!(f, "{}", self.code),
        }
    }
}

impl ErrorT for Error {
    fn description(&self) -> &str {
        self.code.canonical_reason().unwrap_or("An unknown error occurred")
    }

    fn cause(&self) -> Option<&ErrorT> {
        self.cause()
    }
}
