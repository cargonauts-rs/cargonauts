use std::io;
use std::cell::RefCell;
use std::rc::Rc;

use Error;

use anymap::AnyMap;
use futures::{Future, future};
use core::reactor::Handle;
use tokio::NewService;
use c3po::{ConnFuture, Conn, Pool, Config};

use connections::{Client, Configure};

#[derive(Clone)]
pub struct Environment {
    pools: Rc<AnyMap>,
}

impl Environment {
    pub fn conn<C: NewService>(&self) -> ConnFuture<Conn<C>, Error> {
        if let Some(pool) = self.pools.get::<Pool<C>>() {
            pool.connection()
        } else {
            future::Either::A(future::err(Error))
        }
    }

    pub fn client<C: Client>(&self) -> Box<Future<Item = C, Error = Error>> {
        if let Some(pool) = self.pools.get::<Pool<C::Connection>>() {
            Box::new(pool.connection().map(|c| C::connect(c)))
        } else {
            Box::new(future::err(Error))
        }
    }
}

#[derive(Clone)]
pub struct EnvBuilder {
    pools: Rc<RefCell<AnyMap>>,
}

impl EnvBuilder {
    pub fn new() -> EnvBuilder {
        EnvBuilder {
            pools: Rc::new(RefCell::new(AnyMap::new())),
        }
    }

    pub fn new_pool<C>(self, handle: Handle, cfg: Config, client_cfg: C::Config)
        -> Box<Future<Item = (), Error = io::Error>>
    where C: NewService + Configure<Handle> + 'static
    {
        let client = C::new(client_cfg, handle.clone());
        Box::new(Pool::new(client, handle, cfg).map(move |pool| {
            self.pools.borrow_mut().insert(pool);
        }))
    }

    pub fn build(self) -> Environment {
        Environment {
            pools: Rc::new(Rc::try_unwrap(self.pools).unwrap().into_inner()),
        }
    }
}
