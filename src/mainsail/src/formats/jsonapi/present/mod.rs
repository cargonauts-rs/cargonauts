mod attributes;
mod traits;
mod document;
mod rels;

pub use self::traits::ApiSerialize;

use rigging::Error;
use rigging::resource::ResourceEndpoint;

use super::Fields;

use self::traits::{Object, ErrorObject};
use self::document::{Document, ErrorDocument};

pub trait JsonApiResponse {
    fn write(&self, fields: Option<&Fields>) -> Vec<u8>;
}

impl JsonApiResponse for () {
    fn write(&self, _: Option<&Fields>) -> Vec<u8> {
        vec![]
    }
}

impl<T: ApiSerialize + ResourceEndpoint> JsonApiResponse for T {
    fn write(&self, fields: Option<&Fields>) -> Vec<u8> {
        let doc = Document { member: Object { inner: self, fields } };
        doc.write().unwrap_or_else(|e| panic!("{:?}", e))
    }
}

impl<T: ApiSerialize + ResourceEndpoint> JsonApiResponse for Vec<T> {
    fn write(&self, fields: Option<&Fields>) -> Vec<u8> {
        let doc = Document { member: Object { inner: self, fields } };
        doc.write().unwrap_or_else(|e| panic!("{:?}", e))
    }
}

impl JsonApiResponse for Error {
    fn write(&self, _: Option<&Fields>) -> Vec<u8> {
        let doc = ErrorDocument { error: ErrorObject { error: self } };
        doc.write().unwrap_or_else(|e| panic!("{:?}", e))
    }
}
