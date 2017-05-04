use Resource;
use environment::Environment;
use routes::Route;

pub trait Method<T: Resource> {
    const ROUTE: Route<'static>;

    type Request;
    type Response;
    type Outcome: 'static;
}

pub trait ResourceMethod<T: Resource>: Method<T> {
    fn call(id: T::Identifier, req: Self::Request, env: &mut Environment) -> Self::Outcome;
}

pub trait CollectionMethod<T: Resource>: Method<T> {
    fn call(req: Self::Request, env: &mut Environment) -> Self::Outcome;
}
