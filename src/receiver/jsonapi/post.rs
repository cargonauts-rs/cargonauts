use std::marker::PhantomData;

use serde::de::{Visitor, MapVisitor, SeqVisitor};
use serde::de::impls::VecVisitor;

use api::raw::RawResource;
use receiver::Post;
use receiver::jsonapi::ObjectVisitor;
use receiver::jsonapi::resource::{JsonApiResource, ResourceVisitor};
use Deserialize;
use Deserializer;

pub struct PostDocument<T: RawResource + Deserialize>(pub Post<T>);

impl<T: RawResource + Deserialize> Deserialize for PostDocument<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: JsonApiPost<T> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        Ok(PostDocument(inner.0))
    }
}

pub struct JsonApiPost<T: RawResource + Deserialize>(pub Post<T>);

impl<T: RawResource + Deserialize> Deserialize for JsonApiPost<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize(PostVisitor(PhantomData))
    }
}

struct PostVisitor<T: RawResource + Deserialize>(PhantomData<T>);

impl<T: RawResource + Deserialize> Visitor for PostVisitor<T> {
    type Value = JsonApiPost<T>;

    fn visit_seq<V: SeqVisitor>(&mut self, visitor: V) -> Result<JsonApiPost<T>, V::Error> {
        let mut vec_visitor = VecVisitor::new();
        let vector: Vec<JsonApiResource<T, T>> = vec_visitor.visit_seq(visitor)?;
        let vector = vector.into_iter().map(|x| x.0).collect();
        Ok(JsonApiPost(Post::Many(vector)))
    }

    fn visit_map<V: MapVisitor>(&mut self, visitor: V) -> Result<JsonApiPost<T>, V::Error> {
        let resource = ResourceVisitor::<T, T>(PhantomData).visit_map(visitor)?;
        Ok(JsonApiPost(Post::One(resource.0)))
    }
}
