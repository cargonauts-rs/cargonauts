use Serializer;

pub trait Router {
    type Response: Response;
    fn attach_index<F: FnOnce(&[String]) -> Self::Response>(&mut self, route: &'static str, f: F);
    fn attach_get<F>(&mut self, route: &'static str, f: F)
        where F: FnOnce(&str, &[String]) -> Self::Response;
    fn attach_put<F: FnOnce(())>(&mut self, route: &'static str, f: F);
    fn attach_patch<F: FnOnce(())>(&mut self, route: &'static str, f: F);
}

pub enum Status {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalError = 500,
}

pub trait GetRequest {
    fn id(&self) -> &str;
}

pub trait Response: Default {
    type Serializer: Serializer;
    fn set_status(&mut self, status: Status);
    fn serializer(&mut self) -> &mut Self::Serializer;
}

