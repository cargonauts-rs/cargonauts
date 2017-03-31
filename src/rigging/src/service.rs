macro_rules! service {
    ($method:ident as $service:ident : $request:ident -> Resource |$req:ident| $call:block) => {
        pub struct $service<T: ::mainsail::$method> {
            _spoopy: ::std::marker::PhantomData<T>,
        }

        impl<T: ::mainsail::$method> Default for $service<T> {
            fn default() -> Self {
                Self { _spoopy: ::std::marker::PhantomData, }
            }
        }

        impl<T: ::mainsail::$method> ::tokio::Service for $service<T> {
            type Request = ::request::$request<T>;
            type Response = T;
            type Error = ::mainsail::Error;
            type Future = ::futures::BoxFuture<Self::Response, Self::Error>;

            fn call(&self, $req: Self::Request) -> Self::Future {
                $call
            }
        }

        impl<T: ::mainsail::$method> ::tokio::NewService for $service<T> {
            type Request = ::request::$request<T>;
            type Response = T;
            type Error = ::mainsail::Error;
            type Instance = Self;

            fn new_service(&self) -> ::std::io::Result<Self::Instance> {
                Ok(Self {
                    _spoopy: ::std::marker::PhantomData,
                })
            }
        }
    };
    ($method:ident as $service:ident : $request:ident -> Collection |$req:ident| $call:block) => {
        pub struct $service<T: ::mainsail::$method> {
            _spoopy: ::std::marker::PhantomData<T>,
        }

        impl<T: ::mainsail::$method> Default for $service<T> {
            fn default() -> Self {
                Self { _spoopy: ::std::marker::PhantomData, }
            }
        }

        impl<T> ::tokio::stream::StreamService for $service<T> where
            T: ::mainsail::$method,
        {
            type Request = ::request::$request<T>;
            type Response = T;
            type Error = ::mainsail::Error;
            type Stream = ::futures::stream::BoxStream<Self::Response, Self::Error>;

            fn call(&self, $req: Self::Request) -> Self::Stream {
                $call
            }
        }

        impl<T> ::tokio::stream::NewStreamService for $service<T> where
            T: ::mainsail::$method,
        {
            type Request = ::request::$request<T>;
            type Response = T;
            type Error = ::mainsail::Error;
            type Instance = Self;

            fn new_service(&self) -> ::std::io::Result<Self::Instance> {
                Ok(Self {
                    _spoopy: ::std::marker::PhantomData,
                })
            }
        }
    };
}

service!(Get as GetService: GetRequest -> Resource |req| {
    T::get(req.identifier, req.env)
});
service!(Index as IndexService: IndexRequest -> Collection |req| {
    T::index(req.env)
});
