use api;
use Serialize;
use Value;

pub trait Wrapper<T: api::Resource>: Serialize {
    type Relationships: Serialize;
    fn related(&self, base_url: &str) -> Option<Self::Relationships>;
    fn include(&self, params: &[String], base_url: &str) -> Vec<Value>;
}
