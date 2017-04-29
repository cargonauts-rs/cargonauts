use std::fmt::{self, Display};
use std::marker::PhantomData;

use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

use rigging::resource::ResourceEndpoint;

pub struct Relationships<'a, T> {
    id: &'a str,
    _marker: PhantomData<T>,
}

impl<'a, T> Relationships<'a, T> {
    pub fn new(id: &'a str) -> Self { Relationships { id: id, _marker: PhantomData } }
}

impl<'a, T: ResourceEndpoint> Serialize for Relationships<'a, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(T::REL_LINKS.len()))?;
        for link in T::REL_LINKS {
            map.serialize_entry(link.relation, &RelObj {
                id: self.id,
                resource: T::ENDPOINT,
                rel: link.endpoint,
            })?;
        }
        map.end()
    }
}

struct RelObj<'a> {
    id: &'a str,
    resource: &'static str,
    rel: &'static str,
}

impl<'a> Serialize for RelObj<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("links", &RelLink { id: self.id, resource: self.resource, rel: self.rel, })?;
        map.end()
    }
}

struct RelLink<'a> {
    id: &'a str,
    resource: &'static str,
    rel: &'static str,
}

impl<'a> Serialize for RelLink<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("self", &SelfRoute { id: self.id, resource: self.resource, rel: self.rel, })?;
        map.serialize_entry("self", &RelatedRoute { id: self.id, resource: self.resource, rel: self.rel, })?;
        map.end()
    }
}

struct SelfRoute<'a> {
    id: &'a str,
    resource: &'static str,
    rel: &'static str,
}

impl<'a> Serialize for SelfRoute<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'a> Display for SelfRoute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("/")?;
        f.write_str(self.resource)?;
        f.write_str("/")?;
        f.write_str(self.id)?;
        f.write_str("/relationships/")?;
        f.write_str(self.rel)
    }
}

struct RelatedRoute<'a> {
    id: &'a str,
    resource: &'static str,
    rel: &'static str,
}

impl<'a> Serialize for RelatedRoute<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'a> Display for RelatedRoute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("/")?;
        f.write_str(self.resource)?;
        f.write_str("/")?;
        f.write_str(self.id)?;
        f.write_str("/")?;
        f.write_str(self.rel)
    }
}
