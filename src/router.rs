use Serializer;

pub trait Router {
    type Response: Response;
    fn attach_index<F: FnOnce(&[String]) -> Self::Response>(&mut self, route: &'static str, f: F);
    fn attach_get<F>(&mut self, route: &'static str, f: F)
        where F: FnOnce(&str, &[String]) -> Self::Response;
    fn base_url(&self) -> &'static str;
}

pub enum Status {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalError = 500,
}

pub trait Response: Default {
    type Serializer: Serializer;
    fn set_status(&mut self, status: Status);
    fn serializer(&mut self) -> &mut Self::Serializer;
}

