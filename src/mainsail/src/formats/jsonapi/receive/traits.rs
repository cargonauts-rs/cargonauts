use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClientIdPolicy {
    Accepted,
    Required,
    NotAccepted,
    Ignored,
}

pub trait ApiDeserialize<'d>: Sized + 'static {
    const CLIENT_ID_POLICY: ClientIdPolicy;
    type Identifier: FromStr;
    type Attributes: Deserialize<'d>;
    fn from_parts(id: Option<Self::Identifier>, rest: Self::Attributes) -> Self;
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error>;
}
