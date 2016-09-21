use Serializer;
use Value;

pub trait Router {
    type Response: Response;
    fn attach_get<F>(&mut self, route: &'static str, f: F)
        where F: FnMut(GetObject) -> Self::Response;
    fn attach_index<F: FnMut(IndexObject) -> Self::Response>(&mut self, route: &'static str, f: F);
    fn attach_patch<F>(&mut self, route: &'static str, f: F)
        where F: FnMut(PatchObject) -> Self::Response;
    fn attach_post<F>(&mut self, route: &'static str, f: F)
        where F: FnMut(PostObject) -> Self::Response;
    fn base_url(&self) -> &'static str;
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

pub struct GetObject {
    pub id: String,
    pub includes: Vec<String>,
}

pub struct IndexObject {
    pub includes: Vec<String>,
}

pub struct PatchObject {
    pub attributes: Value,
    pub relationships: Vec<Relationship>,
    pub resource_type: String,
    pub id: String,
}

pub struct PostObject {
    pub attributes: Value,
    pub relationships: Vec<Relationship>,
    pub resource_type: String,
    pub id: Option<String>,
}

pub struct Relationship {
    pub resource: String,
    pub member: RelationshipId,
}

pub enum RelationshipId {
    One(String),
    Many(Vec<String>),
}
