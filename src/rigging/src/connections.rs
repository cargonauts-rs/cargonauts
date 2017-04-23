use core::reactor::Handle;
use tokio::NewService;
use c3po::Conn;

pub trait Configure: Sized {
    type Config: Default;
    fn new(handle: &Handle, cfg: Self::Config) -> Self;
}

impl<T: Default> Configure for T {
    type Config = ();
    fn new(_: &Handle, _: Self::Config) -> Self {
        T::default()
    }
}

pub trait Client: 'static {
    type Connection: Configure + NewService + 'static;
    fn connect(conn: Conn<Self::Connection>) -> Self;
}
