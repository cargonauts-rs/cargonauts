use core::reactor::Handle;
use tokio::NewService;
use c3po::Conn;
use serde::de::DeserializeOwned;

mod redis;
pub mod mock;

pub trait Configure: Sized {
    type Config: Default + DeserializeOwned;
    fn config(cfg: Self::Config, handle: Handle) -> Self;
}

pub trait NewServiceLike<S: NewService + 'static>: 'static
    where Self: NewService<Request = S::Request, Response = S::Response, Error = S::Error>
{}

impl<S1, S2> NewServiceLike<S1> for S2
where
    S1: NewService + 'static,
    S2: NewService<Request = S1::Request, Response = S1::Response, Error = S1::Error> + 'static,
{}

pub trait Client: 'static {
    const CONNECTION_NAME: &'static str;
    type Connection: Configure + NewService + 'static;
}

pub trait ConnectClient<C: NewServiceLike<Self::Connection>>: Client {
    fn connect(conn: Conn<C>) -> Self;
}
