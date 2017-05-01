use std::fmt;

use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, IgnoredAny, Unexpected, Error};

use rigging::resource::{ResourceEndpoint, RelIds, WithRels};

use super::{ClientIdPolicy, ApiDeserialize};

pub struct Object<T>(pub T);

impl<'d, T: ResourceEndpoint + ApiDeserialize<'d>> Deserialize<'d> for Object<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ObjectVisitor::new(T::RESOURCE))
    }
}

impl<'d, T: ResourceEndpoint + ApiDeserialize<'d>> Deserialize<'d> for Object<WithRels<T>> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(RelObjectVisitor::new(T::RESOURCE))
    }
}

struct ObjectVisitor<D> {
    ty: Option<&'static str>,
    object: Option<Object<D>>,
}

impl<D> ObjectVisitor<D> {
    pub fn new(ty: &'static str) -> ObjectVisitor<D> { ObjectVisitor {
            ty: Some(ty),
            object: None,
        }
    }
}

impl<'d, D: ResourceEndpoint + ApiDeserialize<'d>> Visitor<'d> for ObjectVisitor<D> {
    type Value = Object<D>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON API resource object")
    }
    fn visit_map<A: MapAccess<'d>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(key) = map.next_key()? {
            match key {
                "type" => {
                    let s = map.next_value()?;
                    match self.ty.take() {
                        Some(ty) if s != ty => Err(A::Error::invalid_value(Unexpected::Str(s), &ty)),
                        None                => Err(A::Error::duplicate_field("type")),
                        _                   => Ok(()),
                    }
                }
                "attributes" if self.object.is_none()   => Ok(self.object = Some(map.next_value()?)),
                "attributes"                            => Err(A::Error::duplicate_field("attributes")),
                "relationships"                         => Err(A::Error::custom("this endpoint does not support relationships")),
                "id"                                    => {
                    match D::CLIENT_ID_POLICY {
                        ClientIdPolicy::NotAccepted => Err(A::Error::custom("this endpoint does not support client ids")),
                        _                           => panic!(),
                    }
                }
                _                                       => map.next_value::<IgnoredAny>().map(|_| ()),
            }?
        }
        if self.ty.is_none() {
            self.object.ok_or(A::Error::missing_field("attributes"))
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct RelObjectVisitor<D: ResourceEndpoint> {
    ty: Option<&'static str>,
    object: Option<Object<D>>,
    rels: Option<D::RelIds>,
}

impl<D: ResourceEndpoint> RelObjectVisitor<D> {
    pub fn new(ty: &'static str) -> RelObjectVisitor<D> {
        RelObjectVisitor {
            ty: Some(ty),
            object: None,
            rels: None,
        }
    }
}

impl<'d, D: ResourceEndpoint + ApiDeserialize<'d>> Visitor<'d> for RelObjectVisitor<D> {
    type Value = Object<WithRels<D>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON API resource object")
    }

    fn visit_map<A: MapAccess<'d>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(key) = map.next_key()? {
            match key {
                "type" => {
                    let s = map.next_value()?;
                    match self.ty.take() {
                        Some(ty) if s != ty => Err(A::Error::invalid_value(Unexpected::Str(s), &ty)),
                        None                => Err(A::Error::duplicate_field("type")),
                        _                   => Ok(()),
                    }
                }
                "attributes" if self.object.is_none()   => Ok(self.object = Some(map.next_value()?)),
                "attributes"                            => Err(A::Error::duplicate_field("attributes")),
                "relationships" if self.rels.is_none()  => {
                    let bridge: RelsBridge<D> = map.next_value()?;
                    Ok(self.rels = Some(bridge.0))
                }
                "relationships"                         => Err(A::Error::duplicate_field("relationships")),
                "id"                                    => {
                    match D::CLIENT_ID_POLICY {
                        ClientIdPolicy::NotAccepted => Err(A::Error::custom("this endpoint does not support client ids")),
                        _                           => panic!(),
                    }
                }
                _                                       => map.next_value::<IgnoredAny>().map(|_| ()),
            }?
        }
        if self.ty.is_none() {
            let rels = self.rels.unwrap_or_else(|| Default::default());
            self.object.map(|obj| Object(WithRels::from_parts(obj.0, rels)))
                .ok_or(A::Error::missing_field("attributes"))
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct RelsBridge<T: ResourceEndpoint>(T::RelIds);

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
