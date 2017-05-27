use std::fmt;
use std::io;
use std::error::Error as ErrorT;
use std::time::Duration;

use backtrace::Backtrace;

use http::StatusCode;

/// The Error type for your application.
pub struct Error {
    code: StatusCode,
    backtrace: Backtrace,
    cause: Cause,
}

enum Cause {
    Error(Box<ErrorT + Send + 'static>),
    Message(String),
    Timeout(Duration),
    Unknown,
}

impl Error {
    /// Construct a new error with an HTTP status code.
    ///
    /// This will create a backtrace and have no underlying cause.
    pub fn new(code: StatusCode) -> Self {
        Self::construct(code, Backtrace::new(), Cause::Unknown)
    }

    /// Construct a new error with a message to provide more context.
    ///
    /// This will create a backtrace and have no underlying cause.
    pub fn with_msg<S: Into<String>>(code: StatusCode, msg: S) -> Self {
        Self::construct(code, Backtrace::new(), Cause::Message(msg.into()))
    }

    /// Construct a new error from another error type.
    ///
    /// This will create a backtrace and have no underlying cause.
    pub fn from_err<E: ErrorT + Send + 'static>(err: E, code: StatusCode) -> Self {
        Self::with_backtrace(err, code, Backtrace::new())
    }

    /// Construct a new error from another error type, with a backtrace
    /// provided by you (probably generated closer to the source of the error).
    pub fn with_backtrace<E: ErrorT + Send + 'static>(err: E, code: StatusCode, backtrace: Backtrace) -> Self {
        Self::construct(code, backtrace, Cause::Error(Box::new(err)))
    }

    /// An error that was cause by a timeout.
    pub fn timeout(time: Duration) -> Self {
        Self::construct(StatusCode::InternalServerError, Backtrace::new(), Cause::Timeout(time))
    }

    fn construct(code: StatusCode, backtrace: Backtrace, cause: Cause) -> Self {
        Self { code, backtrace, cause }
    }
    
    /// The HTTP status code of this error.
    pub fn status_code(&self) -> StatusCode {
        self.code
    }

    /// The backtrace in this error.
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    /// The error which underlies this one, if this error was caused by an
    /// error of another type (that is, constructed by one of the constructors
    /// that take an error value).
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Cause::Error(ref cause) => write!(f, "{}\nINFO: {}\n\n{:?}", self.code, cause, self.backtrace),
            Cause::Message(ref msg) => write!(f, "{}\nINFO: {}\n\n{:?}", self.code, msg, self.backtrace),
            Cause::Timeout(time)    => write!(f, "{}\nINFO: Timed out after {} seconds.", self.code, time.as_secs()),
            Cause::Unknown          => write!(f, "{}\nINFO: No cause provided.\n\n{:?}", self.code, self.backtrace),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Cause::Error(ref cause) => write!(f, "{}\nINFO: {}", self.code, cause),
            Cause::Message(ref msg) => write!(f, "{}\nINFO: {}", self.code, msg),
            Cause::Timeout(time)    => write!(f, "{}\nINFO: Timed out after {} seconds.", self.code, time.as_secs()),
            Cause::Unknown          => write!(f, "{}\nINFO: No cause provided.", self.code),
        }
    }
}

impl ErrorT for Error {
    fn description(&self) -> &str {
        if let Cause::Timeout(_) = self.cause {
            "The request timed out."
        } else {
            self.code.canonical_reason().unwrap_or("An unknown error occurred")
        }
    }

    fn cause(&self) -> Option<&ErrorT> {
        self.cause()
    }
}
