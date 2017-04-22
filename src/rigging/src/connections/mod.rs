use core::reactor::Handle;

mod client;

pub use self::client::{Client, ClientService, ClientConnector};

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
