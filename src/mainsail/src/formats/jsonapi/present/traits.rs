use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeSeq};

use rigging::Error;
use rigging::resource::ResourceEndpoint;
use rigging::environment::Environment;

use super::attributes::Attributes;
use super::rels::Relationships;

pub struct Fields;

impl Fields {
    pub fn get(_: &Environment) -> Option<Fields> {
        None
    }

    pub fn len(&self) -> usize {
        panic!()
    }

    pub fn contains(&self, _field: &str) -> bool {
        panic!()
    }
}

pub trait ApiSerialize {
    fn identifier(&self) -> String;

    fn serialize<S: Serializer>(&self, fields: Option<&Fields>, serializer: S) -> Result<S::Ok, S::Error>;
}

impl ApiSerialize for () {
    fn identifier(&self) -> String {
        panic!("ApiSerialize::identifier for () should never be called.")
    }

    fn serialize<S: Serializer>(&self, _: Option<&Fields>, _: S) -> Result<S::Ok, S::Error> {
        panic!("ApiSerialize::serialize for () should never be called.")
    }
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

pub struct ErrorObject {
    pub error: Error
}

impl Serialize for ErrorObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_map(Some(0))?.end()
    }
}
