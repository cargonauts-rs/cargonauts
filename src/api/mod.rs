use std::str::FromStr;

mod rel;

pub use self::rel::*;

use Deserialize;
use Serialize;

pub trait Resource: Sized {
    type Id: ToString + FromStr;
    fn id(&self) -> Self::Id;
    fn resource() -> &'static str;
}

pub trait Delete: Resource {
    fn delete(id: Self::Id) -> Result<(), DeleteError>;
}

pub trait Get: Resource + Serialize {
    fn get(id: Self::Id) -> Option<Self>;
}

pub trait Index: Resource + Serialize {
    fn index() -> Vec<Self>;
}

pub trait Patch: Resource + Serialize {
    type Patch: Deserialize;
    fn patch(id: Self::Id, patch: Self::Patch) -> Result<Option<Self>, PatchError>;
}

pub trait Post: Resource + Serialize + Deserialize {
    fn post(self) -> Result<Option<Self>, PostError>;
}

pub enum DeleteError {
    BadRequest,
    Forbidden,
    NotFound,
    InternalError,
}

pub enum PatchError {
    BadRequest,
    Forbidden,
    NotFound,
    Conflict,
    InternalError,
}

pub enum PostError {
    BadRequest,
    Forbidden,
    Conflict,
    InternalError,
}


pub enum LinkError {
    BadRequest,
    Forbidden,
    NotFound,
    Conflict,
    InternalError,
}
