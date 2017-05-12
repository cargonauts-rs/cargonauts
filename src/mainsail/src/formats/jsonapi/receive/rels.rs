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
            match map.next_value()? {
                RelData::Single(RelObject { id, ..}) => {
                    if !self.0.try_set_rel_id(key, id) {
                        return Err(A::Error::invalid_value(Unexpected::Str(key), &"a relationship of this resource"))
                    }
                }
                RelData::Many(objects) => {
                    if !self.0.try_set_rel_ids(key, objects.into_iter().map(|RelObject { id, .. }| id).collect()) {
                        return Err(A::Error::invalid_value(Unexpected::Str(key), &"a relationship of this resource"))
                    }
                }
            }
        }
        Ok(RelsBridge(self.0))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RelData<'a> {
    Single(#[serde(borrow)] RelObject<'a>),
    Many(#[serde(borrow)] Vec<RelObject<'a>>),
}

#[derive(Deserialize)]
struct RelObject<'a> {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    ty: &'a str,
    id: String,
}
