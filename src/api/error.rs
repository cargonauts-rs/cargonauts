use router;

use self::Error::*;

pub enum Error {
    BadRequest,
    Forbidden,
    Conflict,
    NotFound,
    NoContent,
    InternalError,
}
