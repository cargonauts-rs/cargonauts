use std::str::FromStr;

use Deserialize;
use Serialize;

pub trait Resource: Sized {
    type Id: ToString + FromStr;
    fn id(&self) -> Self::Id;
    fn resource() -> &'static str;
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

pub trait HasOne<T: Resource>: Resource {
    fn has_one(id: &Self::Id) -> Option<T>;
    fn link_one(id: &Self::Id, rel_id: &T::Id) -> Result<(), LinkError>;
}

pub trait HasMany<T: Resource>: Resource {
    fn has_many(id: &Self::Id) -> Vec<T>;
    fn link_many(id: &Self::Id, rel_ids: &[T::Id]) -> Result<(), LinkError>;
}

pub enum PostError {
    BadRequest,
    Forbidden,
    Conflict,
    InternalError,
}

pub enum PatchError {
    BadRequest,
    Forbidden,
    NotFound,
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
