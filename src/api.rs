use std::str::FromStr;
use Serialize;

pub trait Resource: Serialize + Sized {
    type Id: ToString + FromStr;
    fn id(&self) -> Self::Id;
    fn resource() -> &'static str;
}

pub trait Get: Resource {
    fn get(Self::Id) -> Option<Self>;
}

pub trait Index: Resource {
    fn index() -> Vec<Self>;
}

pub trait HasOne<T: Resource>: Resource {
    fn has_one(id: &Self::Id) -> Option<T>;
}

pub trait HasMany<T: Resource>: Resource {
    fn has_many(id: &Self::Id) -> Vec<T>;
}
