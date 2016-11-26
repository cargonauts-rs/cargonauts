use std::borrow::Cow;

use api::Resource;
use api::raw::{ResourceObject, RawResource};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Identifier {
    pub resource: Cow<'static, str>,
    pub id: String,
}

impl Identifier {
    pub fn new<T: Resource>(id: &T::Id) -> Identifier {
        Identifier {
            resource: Cow::Borrowed(T::resource()),
            id: id.to_string(),
        }
    }
}

impl<'a, T: Resource + 'a> From<&'a T> for Identifier {
    fn from(resource: &'a T) -> Identifier {
        Identifier {
            resource: Cow::Borrowed(T::resource()),
            id: resource.id().to_string(),
        }
    }
}

impl<'a, T: RawResource> From<&'a ResourceObject<T>> for Identifier {
    fn from(resource: &'a ResourceObject<T>) -> Identifier {
        Identifier {
            resource: Cow::Borrowed(T::resource()),
            id: resource.id.to_string(),
        }
    }
}
