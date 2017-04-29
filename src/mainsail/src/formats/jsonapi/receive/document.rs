use std::fmt;
use std::marker::PhantomData;

use serde::de::{Visitor, MapAccess, IgnoredAny};

use rigging::resource::ResourceEndpoint;

use super::{Bridge, ApiDeserialize};

pub struct DocumentVisitor<P>(pub PhantomData<P>);

impl<'d, P: ApiDeserialize<'d> + ResourceEndpoint> Visitor<'d> for DocumentVisitor<P> {
    type Value = Bridge<P>;

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
