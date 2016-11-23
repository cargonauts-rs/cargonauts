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
    type Response: Response + 'static;
    type LinkMaker: MakeLinks;

    fn attach_resource(&mut self,
        resource: &'static str,
        route: ResourceRoute,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );

    fn attach_alias(&mut self,
        alias: &'static str,
        method: Method,
        handler: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    );
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Method {
    Get,
    Patch,
    Post,
    Delete,
    Index,
    Append,
    Replace,
    Clear,
    Remove,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ResourceRoute {
    pub method: Method,
    pub relation: Option<(&'static str, bool)>,
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
    fn endpoint(&self) -> &str;
    fn id(&self) -> &str;
    fn resource_options(&self) -> ResourceOptions;
    fn collection_options(&self) -> CollectionOptions;
    fn alias_info(&self) -> AliasRequest;
}
