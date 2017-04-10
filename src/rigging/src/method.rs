use tokio::NewService;
use tokio::stream::NewStreamService;
use tokio as t;
use tokio::stream as s;

use mainsail::{ResourceEndpoint, Get, Index};
use mainsail::relations::{RelationEndpoint, Relationship, GetOne, GetMany};

use http;
use format::Format;
use request::{GetRequest, IndexRequest};
use service::{GetService, IndexService, GetOneService, GetManyService};
use present::middleware::Presenter;
use receive::middleware::Receiver;

pub trait Method {
    type Request;
    type Response: ResourceEndpoint;
    type Service: Default;
    type WrappedService: NewService<Request = http::Request,
                                    Response = http::Response,
                                    Error = http::Error>;
    fn new_service() -> Self::WrappedService;
}

impl<F, T> Method for (T, F, Get<Identifier = T::Identifier>)
where
    F: Format<T>,
    T: ResourceEndpoint + Get,
{
    type Request = GetRequest<T>;
    type Response = T;
    type Service = GetService<T>;
    type WrappedService = t::NewServiceWrapper<
        Presenter<F::Presenter, Self>,
        t::NewServiceWrapper<Receiver<F::Receiver, Self>, Self::Service>
    >;

    fn new_service() -> Self::WrappedService {
        let presenter = Presenter::<_, Self>::new(F::Presenter::default());
        let receiver = Receiver::<_, Self>::new(F::Receiver::default());
        let service = GetService::default();
        service.wrap(receiver).wrap(presenter)
    }
}

impl<F, T> Method for (T, F, Index<Identifier = T::Identifier>)
where
    F: Format<T>,
    T: ResourceEndpoint + Index,
{
    type Request = IndexRequest<T>;
    type Response = T;
    type Service = IndexService<T>;
    type WrappedService =  s::NewStreamServiceReducer<
        Presenter<F::Presenter, Self>,
        s::NewStreamServiceWrapper<Receiver<F::Receiver, Self>, Self::Service>
    >;

    fn new_service() -> Self::WrappedService {
        let presenter = Presenter::<_, Self>::new(F::Presenter::default());
        let receiver = Receiver::<_, Self>::new(F::Receiver::default());
        let service = IndexService::default();
        service.wrap(receiver).reduce(presenter)
    }
}

impl<F, T, R> Method for (T, F, GetOne<R, Identifier = T::Identifier>)
where
    F: Format<T> + Format<R::Related>,
    T: RelationEndpoint<R> + GetOne<R>,
    R: Relationship,
    R::Related: ResourceEndpoint,
{
    type Request = GetRequest<T>;
    type Response = R::Related;
    type Service = GetOneService<T, R>;
    type WrappedService = t::NewServiceWrapper<
        Presenter<<F as Format<R::Related>>::Presenter, Self>,
        t::NewServiceWrapper<Receiver<<F as Format<T>>::Receiver, Self>, Self::Service>
    >;

    fn new_service() -> Self::WrappedService {
        let presenter = Presenter::<_, Self>::new(<F as Format<R::Related>>::Presenter::default());
        let receiver = Receiver::<_, Self>::new(<F as Format<T>>::Receiver::default());
        let service = GetOneService::default();
        service.wrap(receiver).wrap(presenter)
    }
}

impl<F, T, R> Method for (T, F, GetMany<R, Identifier = T::Identifier>)
where
    F: Format<T> + Format<R::Related>,
    T: RelationEndpoint<R> + GetMany<R>,
    R: Relationship,
    R::Related: ResourceEndpoint,
{
    type Request = GetRequest<T>;
    type Response = R::Related;
    type Service = GetManyService<T, R>;
    type WrappedService =  s::NewStreamServiceReducer<
        Presenter<<F as Format<R::Related>>::Presenter, Self>,
        s::NewStreamServiceWrapper<Receiver<<F as Format<T>>::Receiver, Self>, Self::Service>
    >;

    fn new_service() -> Self::WrappedService {
        let presenter = Presenter::<_, Self>::new(<F as Format<R::Related>>::Presenter::default());
        let receiver = Receiver::<_, Self>::new(<F as Format<T>>::Receiver::default());
        let service = GetManyService::default();
        service.wrap(receiver).reduce(presenter)
    }
}
