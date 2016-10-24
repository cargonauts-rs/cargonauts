use api::{Resource, Error};

pub trait Delete: Resource {
    fn delete(id: &Self::Id) -> Result<(), Error>;
}
