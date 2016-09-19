use api;
use Serialize;
use Value;

pub trait Wrapper<T: api::Resource>: Serialize {
    type Relationships: Serialize;
    fn related(&self) -> Option<Self::Relationships>;
    fn include(&self, params: &[String]) -> Vec<Value>;
}
