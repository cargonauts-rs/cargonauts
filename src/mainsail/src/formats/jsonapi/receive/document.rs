use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserialize, Visitor, MapAccess, IgnoredAny};

pub struct DocumentVisitor<D>(pub PhantomData<D>);

impl<'d, D: Deserialize<'d>> Visitor<'d> for DocumentVisitor<D> {
    type Value = D;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an object with a \"data\" key")
    }

    fn visit_map<A: MapAccess<'d>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        loop {
            if let Some("data") = map.next_key()? {
                return map.next_value()
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
    }
}

