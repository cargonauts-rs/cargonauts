use std::collections::HashMap;
use std::str::FromStr;

use Deserialize;
use Serialize;

pub trait Resource: Sized {
    type Id: ToString + FromStr;
    fn id(&self) -> Self::Id;
    fn resource() -> &'static str;
}

pub trait Get: Resource + Serialize {
    fn get(Self::Id) -> Option<Self>;
}

pub trait Index: Resource + Serialize {
    fn index() -> Vec<Self>;
}

pub trait Post: Resource + Serialize + Deserialize {
    fn post(self, relationship: HashMap<String, Relationship>) -> Result<Option<Self>, PostError>;
}

pub trait HasOne<T: Resource>: Resource {
    fn has_one(id: &Self::Id) -> Option<T>;
}

pub trait HasMany<T: Resource>: Resource {
    fn has_many(id: &Self::Id) -> Vec<T>;
}

pub struct Relationship {
    pub resource: String,
    pub id: String,
}

pub enum PostError {
    Forbidden,
    Conflict,
    BadRequest,
    InternalError,
}
