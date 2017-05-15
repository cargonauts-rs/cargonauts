use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, IgnoredAny, Unexpected, Error};

use rigging::resource::{ResourceEndpoint, WithRels};

use super::{ClientIdPolicy, ApiDeserialize};
use super::rels::RelsBridge;

pub struct Object<T, P>(pub P, PhantomData<T>);

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Deserialize<'d> for Object<T, P> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        match P::CLIENT_ID_POLICY {
            ClientIdPolicy::Ignored     => deserializer.deserialize_map(ObjectVisitor::new(true)),
            ClientIdPolicy::NotAccepted => deserializer.deserialize_map(ObjectVisitor::new(false)),
            _                           => deserializer.deserialize_map(IdObjectVisitor::default())
        }
    }
}

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Deserialize<'d> for Object<T, WithRels<T, P>> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        match P::CLIENT_ID_POLICY {
            ClientIdPolicy::Ignored     => deserializer.deserialize_map(RelObjectVisitor::new(true)),
            ClientIdPolicy::NotAccepted => deserializer.deserialize_map(RelObjectVisitor::new(false)),
            _                           => deserializer.deserialize_map(RelIdObjectVisitor::default())
        }
    }
}

struct Attributes<T>(T);

impl<'d, T: ApiDeserialize<'d>> Deserialize<'d> for Attributes<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Attributes(T::deserialize(deserializer)?))
    }
}

struct ObjectVisitor<T, P> {
    allow_id: bool,
    checked_ty: bool,
    object: Option<Attributes<P>>,
    _marker: PhantomData<T>,
}

impl<T, P> ObjectVisitor<T, P> {
    fn new(allow_id: bool) -> Self {
        ObjectVisitor {
            allow_id,
            checked_ty: false,
            object: None,
            _marker: PhantomData,
        }
    }
}

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Visitor<'d> for ObjectVisitor<T, P> {
    type Value = Object<T, P>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON API resource object")
    }

    fn visit_map<A: MapAccess<'d>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(key) = map.next_key()? {
            match key {
                "type" if self.checked_ty   => Err(A::Error::duplicate_field("type")),
                "type"                      => {
                    let s = map.next_value()?;
                    if s == T::RESOURCE {
                        Ok(self.checked_ty = true)
                    } else {
                        Err(A::Error::invalid_value(Unexpected::Str(s), &T::RESOURCE))
                    }
                }
                "attributes" if self.object.is_none()   => Ok(self.object = Some(map.next_value()?)),
                "attributes"                            => Err(A::Error::duplicate_field("attributes")),
                "relationships"                         => Err(A::Error::custom("this endpoint does not support relationships")),
                "id" if !self.allow_id                  => Err(A::Error::custom("this endpoint does not support client ids")),
                _                                       => map.next_value::<IgnoredAny>().map(|_| ()),
            }?
        }
        if self.checked_ty {
            self.object.ok_or(A::Error::missing_field("attributes")).map(|attrs| Object(attrs.0, PhantomData))
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct IdObjectVisitor<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> {
    checked_ty: bool,
    attrs: Option<P::Attributes>,
    id: Option<P::Identifier>,
    _marker: PhantomData<T>,
}

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Default for IdObjectVisitor<'d, T, P> {
    fn default() -> Self {
        IdObjectVisitor {
            checked_ty: false,
            attrs: None,
            id: None,
            _marker: PhantomData,
        }
    }
}

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Visitor<'d> for IdObjectVisitor<'d, T, P> {
    type Value = Object<T, P>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON API resource object")
    }

    fn visit_map<A: MapAccess<'d>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(key) = map.next_key()? {
            match key {
                "type" if self.checked_ty   => Err(A::Error::duplicate_field("type")),
                "type"                      => {
                    let s = map.next_value()?;
                    if s == T::RESOURCE {
                        Ok(self.checked_ty = true)
                    } else {
                        Err(A::Error::invalid_value(Unexpected::Str(s), &T::RESOURCE))
                    }
                }
                "attributes" if self.attrs.is_none()    => Ok(self.attrs = Some(map.next_value()?)),
                "attributes"                            => Err(A::Error::duplicate_field("attributes")),
                "relationships"                         => Err(A::Error::custom("this endpoint does not support relationships")),
                "id" if self.id.is_none()               => {
                    let id: &str = map.next_value()?;
                    match id.parse() {
                        Ok(id)  => Ok(self.id = Some(id)),
                        Err(_)  => Err(A::Error::invalid_value(Unexpected::Str(id), &"a valid id")),
                    }
                }
                "id"                                    => Err(A::Error::duplicate_field("id")),
                _                                       => map.next_value::<IgnoredAny>().map(|_| ()),
            }?
        }
        if self.checked_ty {
            if P::CLIENT_ID_POLICY == ClientIdPolicy::Required && self.id.is_none() {
                Err(A::Error::missing_field("id"))
            } else {
                let IdObjectVisitor { attrs, id, .. } = self;
                attrs.map(|attrs| Object(P::from_parts(id, attrs), PhantomData)).ok_or(A::Error::missing_field("attributes"))
            }
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct RelObjectVisitor<T: ResourceEndpoint, P> {
    allow_id: bool,
    checked_ty: bool,
    object: Option<Attributes<P>>,
    rels: Option<T::RelIds>,
}

impl<T: ResourceEndpoint, P> RelObjectVisitor<T, P> {
    pub fn new(allow_id: bool) -> Self {
        RelObjectVisitor {
            allow_id,
            checked_ty: false,
            object: None,
            rels: None,
        }
    }
}

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Visitor<'d> for RelObjectVisitor<T, P> {
    type Value = Object<T, WithRels<T, P>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON API resource object")
    }

    fn visit_map<A: MapAccess<'d>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(key) = map.next_key()? {
            match key {
                "type" if self.checked_ty   => Err(A::Error::duplicate_field("type")),
                "type"                      => {
                    let s = map.next_value()?;
                    if s == T::RESOURCE {
                        Ok(self.checked_ty = true)
                    } else {
                        Err(A::Error::invalid_value(Unexpected::Str(s), &T::RESOURCE))
                    }
                }
                "attributes" if self.object.is_none()   => Ok(self.object = Some(map.next_value()?)),
                "attributes"                            => Err(A::Error::duplicate_field("attributes")),
                "relationships" if self.rels.is_none()  => {
                    let bridge: RelsBridge<T> = map.next_value()?;
                    Ok(self.rels = Some(bridge.0))
                }
                "relationships"                         => Err(A::Error::duplicate_field("relationships")),
                "id" if !self.allow_id                  => Err(A::Error::custom("this endpoint does not support client ids")),
                _                                       => map.next_value::<IgnoredAny>().map(|_| ()),
            }?
        }
        if self.checked_ty {
            let rels = self.rels.unwrap_or_else(|| Default::default());
            self.object.map(|attrs| Object(WithRels::from_parts(attrs.0, rels), PhantomData))
                .ok_or(A::Error::missing_field("attributes"))
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct RelIdObjectVisitor<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> {
    checked_ty: bool,
    attrs: Option<P::Attributes>,
    id: Option<P::Identifier>,
    rels: Option<T::RelIds>,
}

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Default for RelIdObjectVisitor<'d, T, P> {
    fn default() -> Self {
        RelIdObjectVisitor {
            checked_ty: false,
            attrs: None,
            id: None,
            rels: None,
        }
    }
}

impl<'d, T: ResourceEndpoint, P: ApiDeserialize<'d>> Visitor<'d> for RelIdObjectVisitor<'d, T, P> {
    type Value = Object<T, WithRels<T, P>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a JSON API resource object")
    }

    fn visit_map<A: MapAccess<'d>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some(key) = map.next_key()? {
            match key {
                "type" if self.checked_ty   => Err(A::Error::duplicate_field("type")),
                "type"                      => {
                    let s = map.next_value()?;
                    if s == T::RESOURCE {
                        Ok(self.checked_ty = true)
                    } else {
                        Err(A::Error::invalid_value(Unexpected::Str(s), &T::RESOURCE))
                    }
                }
                "attributes" if self.attrs.is_none()    => Ok(self.attrs = Some(map.next_value()?)),
                "attributes"                            => Err(A::Error::duplicate_field("attributes")),
                "relationships" if self.rels.is_none()  => {
                    let bridge: RelsBridge<T> = map.next_value()?;
                    Ok(self.rels = Some(bridge.0))
                }
                "relationships"                         => Err(A::Error::duplicate_field("relationships")),
                "id" if self.id.is_none()               => {
                    let id: &str = map.next_value()?;
                    match id.parse() {
                        Ok(id)  => Ok(self.id = Some(id)),
                        Err(_)  => Err(A::Error::invalid_value(Unexpected::Str(id), &"a valid id")),
                    }
                }
                "id"                                    => Err(A::Error::duplicate_field("id")),
                _                                       => map.next_value::<IgnoredAny>().map(|_| ()),
            }?
        }
        if self.checked_ty {
            if P::CLIENT_ID_POLICY == ClientIdPolicy::Required && self.id.is_none() {
                Err(A::Error::missing_field("id"))
            } else {
                let RelIdObjectVisitor { attrs, id, rels, .. } = self;
                let rels = rels.unwrap_or_else(T::RelIds::default);
                attrs.map(|attrs| {
                    let obj = P::from_parts(id, attrs);
                    Object(WithRels::from_parts(obj, rels), PhantomData)
                }).ok_or(A::Error::missing_field("attributes"))
            }
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}
