use std::collections::BTreeMap;

use api::raw::Relationship;
use Serializer;
use Value;

pub mod mock;
mod include;
mod sort;

pub use self::include::IncludeQuery;
pub use self::sort::SortQuery;

pub trait Router {
    type Response: Response;
    fn attach_delete<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(String) -> Self::Response;
    fn attach_get<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(GetRequest) -> Self::Response;
    fn attach_index<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(IndexRequest) -> Self::Response;
    fn attach_patch<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(PatchRequest) -> Self::Response;
    fn attach_post<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(PostRequest) -> Self::Response;
    fn attach_fetch_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(GetRequest) -> Self::Response;
    fn attach_fetch_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(GetRequest) -> Self::Response;
    fn attach_fetch_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String) -> Self::Response;
    fn attach_delete_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String) -> Self::Response;
    fn attach_delete_one_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String) -> Self::Response;
    fn attach_remove_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Vec<String>) -> Self::Response;
    fn attach_remove_many_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Vec<String>) -> Self::Response;
    fn attach_clear_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String) -> Self::Response;
    fn attach_clear_many_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String) -> Self::Response;
    fn attach_patch_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(PatchRequest) -> Self::Response;
    fn attach_post_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, PostRequest) -> Self::Response;
    fn attach_append_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, PostRequest) -> Self::Response;
    fn attach_link_one<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
        where F: Fn(String, Relationship) -> Self::Response;
    fn attach_append_link_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Relationship) -> Self::Response;
    fn attach_replace_link_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Relationship) -> Self::Response;
}

pub enum Status {
    Ok = 200,
    NoContent = 203,
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    InternalError = 500,
}

pub trait Response: Default {
    type Serializer: Serializer;
    fn set_status(&mut self, status: Status);
    fn serializer(&mut self) -> &mut Self::Serializer;
}

pub struct GetRequest {
    pub id: String,
    pub includes: Vec<IncludeQuery>,
}

pub struct IndexRequest {
    pub includes: Vec<IncludeQuery>,
    pub sort: Vec<SortQuery>,
}

pub struct PatchRequest {
    pub attributes: Value,
    pub relationships: BTreeMap<String, Relationship>,
    pub id: String,
}

pub struct PostRequest {
    pub attributes: Value,
    pub relationships: BTreeMap<String, Relationship>,
}
