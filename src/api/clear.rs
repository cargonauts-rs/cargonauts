use api::{Resource, Error};
use IntoFuture;

pub trait Clear: Resource {
    type ClearFut: IntoFuture<Item = (), Error = Error>;
    fn clear() -> Self::ClearFut;
}
