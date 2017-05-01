use serde::de::{Deserialize, Deserializer};
use rigging::Resource;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClientIdPolicy {
    Accepted,
    Required,
    NotAccepted,
}

pub trait ApiDeserialize<'d>: Resource + Sized {
    const CLIENT_ID_POLICY: ClientIdPolicy;
    type Attributes: Deserialize<'d>;
    fn from_parts(id: Option<Self::Identifier>, rest: Self::Attributes) -> Self;
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error>;
}
