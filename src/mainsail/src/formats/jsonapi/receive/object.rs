use std::fmt;

use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, IgnoredAny, Unexpected, Error};

use rigging::resource::{ResourceEndpoint, WithRels};

use super::{ClientIdPolicy, ApiDeserialize};
use super::rels::RelsBridge;

pub struct Object<T>(pub T);

impl<'d, T: ResourceEndpoint + ApiDeserialize<'d>> Deserialize<'d> for Object<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        if T::CLIENT_ID_POLICY == ClientIdPolicy::NotAccepted {
            deserializer.deserialize_map(ObjectVisitor::new(T::RESOURCE))
        } else {
            deserializer.deserialize_map(IdObjectVisitor::new(T::RESOURCE))
        }
    }
}

impl<'d, T: ResourceEndpoint + ApiDeserialize<'d>> Deserialize<'d> for Object<WithRels<T>> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        if T::CLIENT_ID_POLICY == ClientIdPolicy::NotAccepted {
            deserializer.deserialize_map(RelObjectVisitor::new(T::RESOURCE))
        } else {
            deserializer.deserialize_map(RelIdObjectVisitor::new(T::RESOURCE))
        }
    }
}

struct Attributes<T>(T);

impl<'d, T: ApiDeserialize<'d>> Deserialize<'d> for Attributes<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Attributes(T::deserialize(deserializer)?))
    }
}

struct ObjectVisitor<D> {
    ty: Option<&'static str>,
    object: Option<Attributes<D>>,
}

impl<D> ObjectVisitor<D> {
    pub fn new(ty: &'static str) -> ObjectVisitor<D> { ObjectVisitor {
            ty: Some(ty),
            object: None,
        }
    }
}

impl<'d, D: ApiDeserialize<'d>> Visitor<'d> for ObjectVisitor<D> {
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
                "id"                                    => Err(A::Error::custom("this endpoint does not support client ids")),
                _                                       => map.next_value::<IgnoredAny>().map(|_| ()),
            }?
        }
        if self.ty.is_none() {
            self.object.ok_or(A::Error::missing_field("attributes")).map(|attrs| Object(attrs.0))
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct IdObjectVisitor<'d, D: ApiDeserialize<'d>> {
    ty: Option<&'static str>,
    attrs: Option<D::Attributes>,
    id: Option<D::Identifier>,
}

impl<'d, D: ApiDeserialize<'d>> IdObjectVisitor<'d, D> {
    pub fn new(ty: &'static str) -> IdObjectVisitor<'d, D> {
        IdObjectVisitor {
            ty: Some(ty),
            attrs: None,
            id: None,
        }
    }
}

impl<'d, D: ApiDeserialize<'d>> Visitor<'d> for IdObjectVisitor<'d, D> {
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
        if self.ty.is_none() {
            if D::CLIENT_ID_POLICY == ClientIdPolicy::Required && self.id.is_none() {
                Err(A::Error::missing_field("id"))
            } else {
                let IdObjectVisitor { attrs, id, .. } = self;
                attrs.map(|attrs| Object(D::from_parts(id, attrs))).ok_or(A::Error::missing_field("attributes"))
            }
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct RelObjectVisitor<D: ResourceEndpoint> {
    ty: Option<&'static str>,
    object: Option<Attributes<D>>,
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
            self.object.map(|attrs| Object(WithRels::from_parts(attrs.0, rels)))
                .ok_or(A::Error::missing_field("attributes"))
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}

struct RelIdObjectVisitor<'d, D: ResourceEndpoint + ApiDeserialize<'d>> {
    ty: Option<&'static str>,
    attrs: Option<D::Attributes>,
    id: Option<D::Identifier>,
    rels: Option<D::RelIds>,
}

impl<'d, D: ResourceEndpoint + ApiDeserialize<'d>> RelIdObjectVisitor<'d, D> {
    pub fn new(ty: &'static str) -> RelIdObjectVisitor<'d, D> {
        RelIdObjectVisitor {
            ty: Some(ty),
            attrs: None,
            id: None,
            rels: None,
        }
    }
}

impl<'d, D: ResourceEndpoint + ApiDeserialize<'d>> Visitor<'d> for RelIdObjectVisitor<'d, D> {
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
                "attributes" if self.attrs.is_none()    => Ok(self.attrs = Some(map.next_value()?)),
                "attributes"                            => Err(A::Error::duplicate_field("attributes")),
                "relationships" if self.rels.is_none()  => {
                    let bridge: RelsBridge<D> = map.next_value()?;
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
        if self.ty.is_none() {
            if D::CLIENT_ID_POLICY == ClientIdPolicy::Required && self.id.is_none() {
                Err(A::Error::missing_field("id"))
            } else {
                let RelIdObjectVisitor { attrs, id, rels, .. } = self;
                let rels = rels.unwrap_or_else(D::RelIds::default);
                attrs.map(|attrs| {
                    let obj = D::from_parts(id, attrs);
                    Object(WithRels::from_parts(obj, rels))
                }).ok_or(A::Error::missing_field("attributes"))
            }
        } else {
            Err(A::Error::missing_field("type"))
        }
    }
}
