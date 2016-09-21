use api;
use Serialize;
use router;
use Value;

pub trait Wrapper<T: api::Resource>: Serialize {
    type Relationships: Serialize;
    fn get_related(&self, base_url: &str) -> Option<Self::Relationships>;
    fn put_related<'a, I>(id: &T::Id, rels: I) -> Result<(), api::LinkError>
        where I: IntoIterator<Item = &'a router::Relationship>;
    fn include(&self, params: &[String], base_url: &str) -> Vec<Value>;
}
