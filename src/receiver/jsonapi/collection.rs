use std::marker::PhantomData;
use serde::de::impls::VecVisitor;

use api::raw::{RawResource, RawReceived};
use receiver::jsonapi::ObjectVisitor;
use receiver::jsonapi::resource::JsonApiResource;
use Deserialize;
use Deserializer;

pub struct CollectionDocument<T: RawResource, A: Deserialize>(pub Vec<RawReceived<T, A>>);

impl<T, A> Deserialize for CollectionDocument<T, A>
where
    T: RawResource,
    A: Deserialize,
{
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: JsonApiCollection<T, A> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        Ok(CollectionDocument(inner.0))
    }
}

struct JsonApiCollection<T: RawResource, A: Deserialize>(Vec<RawReceived<T, A>>);

impl<T, A> Deserialize for JsonApiCollection<T, A>
where
    T: RawResource,
    A: Deserialize,
{
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let visitor = VecVisitor::new();
        let vector: Vec<JsonApiResource<T, A>> = deserializer.deserialize_seq(visitor)?;
        Ok(JsonApiCollection(vector.into_iter().map(|x| x.0).collect()))
    }
}
