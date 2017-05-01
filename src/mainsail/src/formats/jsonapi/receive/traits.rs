use serde::de::Deserializer;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClientIdPolicy {
    Accepted,
    Required,
    NotAccepted,
}

pub trait ApiDeserialize<'d>: Sized + 'static {
    const CLIENT_ID_POLICY: ClientIdPolicy;
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error>;
}
