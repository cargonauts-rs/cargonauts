use futures::BoxFuture;
use futures::stream::BoxStream;

use tokio::{Service, NewService, NewMiddleware};
use tokio::stream::{StreamService, NewStreamService, NewStreamMiddleware};
use tokio as t;
use tokio::stream as s;

use mainsail::{ResourceEndpoint, Get, Index, Error};
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
    type Service;
    type WrappedService: NewService<Request = http::Request,
                                    Response = http::Response,
                                    Error = http::Error>;
    fn new_service() -> Self::WrappedService;
}

macro_rules! method {
    ($method:ident [ $($arg:ident),* ] : $request:ty => $response:ty as Resource in $service:ty; $($other_bounds:tt)*) => {
        impl<F, T, $($arg),*> Method for (T, F, $method<$($arg,)* Identifier = T::Identifier>)
        $($other_bounds)*, F: Format<T>,
        {
            type Request = $request;
            type Response = $response;
            type Service = $service;
            type WrappedService = t::NewServiceWrapper<
                Presenter<<F as Format<$response>>::Presenter, Self>,
                t::NewServiceWrapper<Receiver<<F as Format<T>>::Receiver, Self>, Self::Service>
            >;
            fn new_service() -> Self::WrappedService {
                let presenter = Presenter::<_, Self>::new(<F as Format<$response>>::Presenter::default());
                let receiver = Receiver::<_, Self>::new(<F as Format<T>>::Receiver::default());
                let service = <$service as Default>::default();
                NewService::wrap(service, receiver).wrap(presenter)
            }
        }

        impl<F, T, M, $($arg),*> Method for (M, T, F, $method<$($arg,)* Identifier = T::Identifier>)
        $($other_bounds)*,
            F: Format<T>,
            M: NewMiddleware<$service> + Default,
            M::WrappedService: Service<
                Request = $request,
                Response = $response,
                Error = Error,
                Future = BoxFuture<$response, Error>,
            >,
        {
            type Request = $request;
            type Response = $response;
            type Service = t::NewServiceWrapper<M, $service>;
            type WrappedService = t::NewServiceWrapper<
                Presenter<<F as Format<$response>>::Presenter, Self>,
                t::NewServiceWrapper<Receiver<<F as Format<T>>::Receiver, Self>, Self::Service>
            >;
            fn new_service() -> Self::WrappedService {
                let presenter = Presenter::<_, Self>::new(<F as Format<$response>>::Presenter::default());
                let receiver = Receiver::<_, Self>::new(<F as Format<T>>::Receiver::default());
                let service = M::default().wrap(<$service as Default>::default());
                NewService::wrap(service, receiver).wrap(presenter)
            }
        }
    };
    ($method:ident [ $($arg:ident),* ] : $request:ty => $response:ty as Collection in $service:ty; $($other_bounds:tt)*) => {
        impl<F, T, $($arg),*> Method for (T, F, $method<$($arg,)* Identifier = T::Identifier>)
        $($other_bounds)*, F: Format<T>,
        {
            type Request = $request;
            type Response = $response;
            type Service = $service;
            type WrappedService = s::NewStreamServiceReducer<
                Presenter<<F as Format<$response>>::Presenter, Self>,
                s::NewStreamServiceWrapper<Receiver<<F as Format<T>>::Receiver, Self>, Self::Service>
            >;
            fn new_service() -> Self::WrappedService {
                let presenter = Presenter::<_, Self>::new(<F as Format<$response>>::Presenter::default());
                let receiver = Receiver::<_, Self>::new(<F as Format<T>>::Receiver::default());
                let service = <$service as Default>::default();
                NewStreamService::wrap(service, receiver).reduce(presenter)
            }
        }

        impl<F, T, M, $($arg),*> Method for (M, T, F, $method<$($arg,)* Identifier = T::Identifier>)
        $($other_bounds)*,
            F: Format<T>,
            M: NewStreamMiddleware<$service> + Default,
            M::WrappedService: StreamService<
                Request = $request,
                Response = $response,
                Error = Error,
                Stream = BoxStream<$response, Error>,
            >,
        {
            type Request = $request;
            type Response = $response;
            type Service = s::NewStreamServiceWrapper<M, $service>;
            type WrappedService = s::NewStreamServiceReducer<
                Presenter<<F as Format<$response>>::Presenter, Self>,
                s::NewStreamServiceWrapper<Receiver<<F as Format<T>>::Receiver, Self>, Self::Service>
            >;
            fn new_service() -> Self::WrappedService {
                let presenter = Presenter::<_, Self>::new(<F as Format<$response>>::Presenter::default());
                let receiver = Receiver::<_, Self>::new(<F as Format<T>>::Receiver::default());
                let service = M::default().wrap(<$service as Default>::default());
                NewStreamService::wrap(service, receiver).reduce(presenter)
            }
        }
    };
}

method!(Get[]: GetRequest<T> => T as Resource in GetService<T>;
        where T: ResourceEndpoint + Get);

method!(Index[]: IndexRequest<T> => T as Collection in IndexService<T>;
        where T: ResourceEndpoint + Index);

method!(GetOne[R]: GetRequest<T> => R::Related as Resource in GetOneService<T, R>;
        where T: RelationEndpoint<R> + GetOne<R>,
              R: Relationship, R::Related: ResourceEndpoint,
              F: Format<R::Related>);

method!(GetMany[R]: GetRequest<T> => R::Related as Collection in GetManyService<T, R>;
        where T: RelationEndpoint<R> + GetMany<R>,
              R: Relationship, R::Related: ResourceEndpoint,
              F: Format<R::Related>);
