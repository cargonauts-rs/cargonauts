use std::collections::BTreeMap;
use std::io::Read;

use api::AliasRequest;
use api::raw::Relationship;
use Serializer;

pub mod mock;
mod include;
mod page;
mod sort;

pub use self::include::IncludeQuery;
pub use self::page::Pagination;
pub use self::sort::SortQuery;

pub trait Router {
    type Response: Response;
    type LinkMaker: MakeLinks;
    fn attach_delete<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(String, Self::LinkMaker) -> Self::Response;
    fn attach_get<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(GetRequest, Self::LinkMaker) -> Self::Response;
    fn attach_index<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(IndexRequest, Self::LinkMaker) -> Self::Response;
    fn attach_patch<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(PatchRequest, Self::LinkMaker) -> Self::Response;
    fn attach_post<F>(&mut self, resource: &'static str, f: F)
        where F: Fn(PostRequest, Self::LinkMaker) -> Self::Response;
    fn attach_fetch_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(GetRequest, Self::LinkMaker) -> Self::Response;
    fn attach_fetch_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(GetRequest, Self::LinkMaker) -> Self::Response;
    fn attach_fetch_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(FetchRelRequest, Self::LinkMaker) -> Self::Response;
    fn attach_delete_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Self::LinkMaker) -> Self::Response;
    fn attach_delete_one_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Self::LinkMaker) -> Self::Response;
    fn attach_remove_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Vec<String>, Self::LinkMaker) -> Self::Response;
    fn attach_remove_many_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Vec<String>, Self::LinkMaker) -> Self::Response;
    fn attach_clear_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Self::LinkMaker) -> Self::Response;
    fn attach_clear_many_rel<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Self::LinkMaker) -> Self::Response;
    fn attach_patch_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(PatchRequest, Self::LinkMaker) -> Self::Response;
    fn attach_post_one<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, PostRequest, Self::LinkMaker) -> Self::Response;
    fn attach_append_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, PostRequest, Self::LinkMaker) -> Self::Response;
    fn attach_link_one<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
        where F: Fn(String, Relationship, Self::LinkMaker) -> Self::Response;
    fn attach_append_link_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Relationship, Self::LinkMaker) -> Self::Response;
    fn attach_replace_link_many<F>(&mut self, resource: &'static str, relationship: &'static str, f: F)
        where F: Fn(String, Relationship, Self::LinkMaker) -> Self::Response;
    fn attach_get_alias<F>(&mut self, alias: &'static str, f: F)
        where F: Fn(AliasRequest, GetRequest, Self::LinkMaker) -> Self::Response;
}

pub enum Status {
    Ok = 200,
    Accepted,
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
    fn set_content(&mut self, content_type: &str);
    fn serializer(&mut self) -> &mut Self::Serializer;
}

pub trait MakeLinks {
    fn collection(&self, resource: &str) -> String;
    fn resource(&self, resource: &str, id: &str) -> String;
    fn relationship(&self, resource: &str, id: &str, relation: &str) -> String;
    fn related_resource(&self, resource: &str, id: &str, relation: &str) -> String;
}

impl<'a, L: MakeLinks> MakeLinks for &'a L {
    fn collection(&self, resource: &str) -> String {
        L::collection(self, resource)
    }
    fn resource(&self, resource: &str, id: &str) -> String {
        L::resource(self, resource, id)
    }
    fn relationship(&self, resource: &str, id: &str, relation: &str) -> String {
        L::relationship(self, resource, id, relation)
    }
    fn related_resource(&self, resource: &str, id: &str, relation: &str) -> String {
        L::related_resource(self, resource, id, relation)
    }
}

pub struct GetRequest {
    pub id: String,
    pub includes: Vec<IncludeQuery>,
    pub field_set: Option<Vec<String>>,
}

pub struct IndexRequest {
    pub includes: Vec<IncludeQuery>,
    pub sort: Vec<SortQuery>,
    pub page: Option<Pagination>,
    pub field_set: Option<Vec<String>>,
}

pub struct FetchRelRequest {
    pub id: String,
    pub includes: Vec<IncludeQuery>,
    pub relationship_route: String,
    pub related_resource_route: String,
}

pub struct PatchRequest {
    pub attributes: Box<Read>,
    pub relationships: BTreeMap<String, Relationship>,
    pub id: String,
    pub field_set: Option<Vec<String>>,
}

pub struct PostRequest {
    pub attributes: Box<Read>,
    pub relationships: BTreeMap<String, Relationship>,
    pub field_set: Option<Vec<String>>,
}
