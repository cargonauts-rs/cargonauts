use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeSeq};

use rigging::Error;
use rigging::environment::Environment;

pub struct Fields;

impl Fields {
    pub fn get(_: &Environment) -> Option<Fields> {
        None
    }
}

pub trait ApiSerialize {
    fn serialize<S: Serializer>(&self, fields: Option<&Fields>, serializer: S) -> Result<S::Ok, S::Error>;
}

impl ApiSerialize for () {
    fn serialize<S: Serializer>(&self, _: Option<&Fields>, _: S) -> Result<S::Ok, S::Error> {
        panic!("ApiSerialize::serialize for () should never be called.")
    }
}

pub struct Bridge<'a, T: 'a> {
    pub fields: Option<&'a Fields>,
    pub inner: &'a T,
}

impl<'a, T: ApiSerialize> Serialize for Bridge<'a, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(self.fields, serializer)
    }
}

impl<'a, T: ApiSerialize> Serialize for Bridge<'a, Vec<T>> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.inner.len()))?;
        for elem in self.inner {
            seq.serialize_element(&Bridge { fields: self.fields, inner: elem })?;
        }
        seq.end()
    }
}

pub struct ErrorBridge {
    pub error: Error
}

impl Serialize for ErrorBridge {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_map(Some(0))?.end()
    }
}
