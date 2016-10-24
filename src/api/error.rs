use router;

use self::Error::*;

pub enum Error {
    BadRequest,
    Forbidden,
    Conflict,
    NotFound,
    InternalError,
}

impl<'a> From<&'a Error> for router::Status {
    fn from(error: &'a Error) -> router::Status {
        match *error {
            BadRequest      => router::Status::BadRequest,
            Forbidden       => router::Status::Forbidden,
            Conflict        => router::Status::Conflict,
            NotFound        => router::Status::NotFound,
            InternalError   => router::Status::InternalError,
        }
    }
}
