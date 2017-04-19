use std::io;

use futures::Future;
use core::reactor::Handle;
use tokio::NewService;
use c3po::{Config, Pool};

mod client;

pub use self::client::{Client, ClientService, ClientConnector};

pub trait Configure: Sized {
    type Config;
    fn new(handle: &Handle, cfg: Self::Config) -> Self;
}

impl<T: Default> Configure for T {
    type Config = ();
    fn new(_: &Handle, _: Self::Config) -> Self {
        T::default()
    }
}

pub fn new_pool<C>(handle: Handle, cfg: Config, client_cfg: C::Config)
    -> Box<Future<Item = Pool<C>, Error = io::Error>>
where
    C: Configure + NewService + 'static
{
    let client = C::new(&handle, client_cfg);
    Pool::new(client, handle, cfg)
}
