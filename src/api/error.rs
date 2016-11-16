use std::error::Error as ErrorTrait;
use std::fmt::{self, Display};

use router::Status;
use self::Error::*;

#[derive(Debug)]
pub enum Error {
    BadRequest,
    Forbidden,
    Conflict,
    NotFound,
    NoContent,
    InternalError,
}

impl Error {
    pub fn status(&self) -> Status {
        match *self {
            BadRequest      => Status::BadRequest,
            Forbidden       => Status::Forbidden,
            Conflict        => Status::Conflict,
            NotFound        => Status::NotFound,
            NoContent       => Status::NoContent,
            InternalError   => Status::InternalError,
        }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            BadRequest      => "Bad Request",
            Forbidden       => "Forbidden",
            Conflict        => "Conflict",
            NotFound        => "Not Found",
            NoContent       => "No Content",
            InternalError   => "Internal Server Error",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}
