use api::{Resource, Error};
use IntoFuture;

pub trait Remove: Resource {
    type RemoveFut: IntoFuture<Item = (), Error = Error>;
    fn remove(ids: &[Self::Id]) -> Self::RemoveFut;
}
