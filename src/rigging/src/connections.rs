use core::reactor::Handle;
use tokio::NewService;
use c3po::Conn;
use serde::de::DeserializeOwned;

pub trait Configure<Args>: Sized {
    type Config: Default + DeserializeOwned;
    fn new(cfg: Self::Config, args: Args) -> Self;
}

impl<T: Default, Args> Configure<Args> for T {
    type Config = ();
    fn new(_: Self::Config, _: Args) -> Self {
        T::default()
    }
}

pub trait Client: 'static {
    type Connection: Configure<Handle> + NewService + 'static;
    fn connect(conn: Conn<Self::Connection>) -> Self;
}
