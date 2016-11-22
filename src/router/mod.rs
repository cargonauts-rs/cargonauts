use std::io::Read;
use std::io::Write;

use api::AliasRequest;

pub mod mock;
mod include;
mod page;
mod sort;

pub use self::include::IncludeQuery;
pub use self::page::Pagination;
pub use self::sort::SortQuery;

pub trait Router {
    type Request: Request;
    type Response: Response;
    type LinkMaker: MakeLinks;
    fn attach_get(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_index(&mut self,
        resource: &'static str, 
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_delete(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_clear(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_remove(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_patch(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_post(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_append(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_replace(&mut self,
        resource: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );

    fn attach_fetch_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_fetch_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_fetch_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );

    fn attach_delete_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_clear_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_remove_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_delete_one_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_clear_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_remove_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );

    fn attach_post_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_patch_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_append_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_replace_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_update_one_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_append_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
    fn attach_replace_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );

    fn attach_get_alias(&mut self,
        alias: &'static str,
        handler: fn(AliasRequest, Self::Request, Self::LinkMaker) -> Self::Response,
    );

    fn attach_index_alias(&mut self,
        alias: &'static str,
        handler: fn(AliasRequest, Self::Request, Self::LinkMaker) -> Self::Response,
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

pub trait Response: Default + Write {
    fn set_status(&mut self, status: Status);
    fn set_content(&mut self, content_type: &str);
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

pub struct ResourceOptions {
    pub includes: Vec<IncludeQuery>,
    pub field_set: Option<Vec<String>>,
}

pub struct CollectionOptions {
    pub includes: Vec<IncludeQuery>,
    pub sort: Vec<SortQuery>,
    pub page: Option<Pagination>,
    pub field_set: Option<Vec<String>>,
}

pub trait Request: Read {
    fn id(&self) -> &str;
    fn resource_options(&self) -> ResourceOptions;
    fn collection_options(&self) -> CollectionOptions;
}
