use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeSeq};

use rigging::Error;
use rigging::resource::ResourceEndpoint;

use super::attributes::Attributes;
use super::rels::Relationships;
use super::super::Fields;

pub trait ApiSerialize: Sized {
    fn identifier(&self) -> String;

    fn serialize<S: Serializer>(&self, fields: Option<&Fields>, serializer: S) -> Result<S::Ok, S::Error>;
}

pub struct Object<'a, T: 'a> {
    pub fields: Option<&'a Fields>,
    pub inner: &'a T,
}

impl<'a, T: ApiSerialize + ResourceEndpoint> Serialize for Object<'a, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let id = self.inner.identifier();
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("type", T::RESOURCE)?;
        map.serialize_entry("id", &id)?;
        map.serialize_entry("attributes", &Attributes { fields: self.fields, attributes: self.inner })?;
        map.serialize_entry("relationships", &Relationships::<T>::new(&id))?;
        map.end()
    }
}

impl<'a, T: ApiSerialize + ResourceEndpoint> Serialize for Object<'a, Vec<T>> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.inner.len()))?;
        for elem in self.inner {
            seq.serialize_element(&Object { fields: self.fields, inner: elem })?;
        }
        seq.end()
    }
}

pub struct ErrorObject<'a> {
    pub error: &'a Error
}

impl<'a> Serialize for ErrorObject<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_map(Some(0))?.end()
    }
}
