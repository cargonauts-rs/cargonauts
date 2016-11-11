use api::{Resource, Error};
use IntoFuture;

pub trait Delete: Resource {
    type DeleteFut: IntoFuture<Item = (), Error = Error>;
    fn delete(id: &Self::Id) -> Self::DeleteFut;
}
