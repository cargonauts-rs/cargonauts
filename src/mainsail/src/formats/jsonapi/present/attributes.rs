use serde::{Serialize, Serializer};

use super::ApiSerialize;
use super::super::Fields;

pub struct Attributes<'a, T: 'a> {
    pub fields: Option<&'a Fields<T>>,
    pub attributes: &'a T,
}

impl<'a, T: ApiSerialize> Serialize for Attributes<'a, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.attributes.serialize(self.fields, serializer)
    }
}
