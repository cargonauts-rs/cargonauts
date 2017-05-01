use std::fmt;

use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, Unexpected, Error};

use rigging::resource::{ResourceEndpoint, RelIds};

pub struct RelsBridge<T: ResourceEndpoint>(pub T::RelIds);

impl<'d, T: ResourceEndpoint> Deserialize<'d> for RelsBridge<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        let visitor: RelsVisitor<T> = RelsVisitor(Default::default());
        deserializer.deserialize_map(visitor)
    }
}

struct RelsVisitor<T: ResourceEndpoint>(T::RelIds);

impl<'d, T: ResourceEndpoint> Visitor<'d> for RelsVisitor<T> {
    type Value = RelsBridge<T>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON API relationships object")
    }

    fn visit_map<A: MapAccess<'d>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(key) = map.next_key::<&'d str>()? {
            let RelObject { ty, id } = map.next_value()?;
            if ty != key {
                return Err(A::Error::invalid_value(Unexpected::Str(ty), &key))
            }
            if !self.0.try_set_rel_id(ty, id) {
                return Err(A::Error::invalid_value(Unexpected::Str(ty), &"a relationship of this resource"))
            }
        }
        Ok(RelsBridge(self.0))
    }
}

#[derive(Deserialize)]
struct RelObject<'a> {
    #[serde(rename = "type")]
    ty: &'a str,
    id: String,
}
