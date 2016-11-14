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
    fn attach_get(&mut self,
        resource: &'static str,
        handler: fn(GetRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_index(&mut self,
        resource: &'static str, 
        handler: fn(IndexRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_delete(&mut self,
        resource: &'static str,
        handler: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_clear(&mut self,
        resource: &'static str,
        handler: fn(Self::LinkMaker) -> Self::Response,
    );
    fn attach_remove(&mut self,
        resource: &'static str,
        handler: fn(RemoveRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_patch(&mut self,
        resource: &'static str,
        handler: fn(PatchRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_post(&mut self,
        resource: &'static str,
        handler: fn(PostRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_append(&mut self,
        resource: &'static str,
        handler: fn(MultiPostRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_replace(&mut self,
        resource: &'static str,
        handler: fn(MultiPostRequest, Self::LinkMaker) -> Self::Response,
    );

    fn attach_fetch_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(GetRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_fetch_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(GetRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_fetch_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(FetchRelRequest, Self::LinkMaker) -> Self::Response,
    );

    fn attach_delete_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_clear_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_remove_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(RemoveManyRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_delete_one_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_clear_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_remove_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(RemoveManyRequest, Self::LinkMaker) -> Self::Response,
    );

    fn attach_post_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(PostOneRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_patch_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(PatchRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_append_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(PostManyRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_replace_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(PostManyRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_update_one_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(UpdateRelRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_append_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(UpdateRelRequest, Self::LinkMaker) -> Self::Response,
    );
    fn attach_replace_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(UpdateRelRequest, Self::LinkMaker) -> Self::Response,
    );

    fn attach_get_alias(&mut self,
        alias: &'static str,
        handler: fn(AliasRequest, GetRequest, Self::LinkMaker) -> Self::Response,
    );
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

pub struct DeleteRequest {
    pub id: String,
}

pub struct RemoveRequest {
    pub ids: Vec<String>,
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

pub struct MultiPostRequest {
    pub attributes: Vec<Box<Read>>,
    pub relationships: Vec<BTreeMap<String, Relationship>>,
    pub field_set: Option<Vec<String>>,
}

pub struct FetchRelRequest {
    pub id: String,
    pub includes: Vec<IncludeQuery>,
    pub relationship_route: String,
    pub related_resource_route: String,
}

pub struct RemoveManyRequest {
    pub id: String,
    pub rel_ids: Vec<String>,
}

pub struct PostOneRequest {
    pub id: String,
    pub attributes: Box<Read>,
    pub relationships: BTreeMap<String, Relationship>,
    pub field_set: Option<Vec<String>>,
}

pub struct PostManyRequest {
    pub id: String,
    pub attributes: Box<Read>,
    pub relationships: BTreeMap<String, Relationship>,
    pub field_set: Option<Vec<String>>,
}

pub struct UpdateRelRequest {
    pub id: String,
    pub field_set: Option<Vec<String>>,
    pub rel: Relationship,
}
