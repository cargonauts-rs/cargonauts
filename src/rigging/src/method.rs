use Resource;
use environment::Environment;
use request::Request;
use routes::Route;

pub trait Method<T: Resource> {
    const ROUTE: Route<'static>;

    type Request: Request<T>;
    type Response;
    type Outcome: 'static;

    fn call(req: Self::Request, env: &mut Environment) -> Self::Outcome;
}
