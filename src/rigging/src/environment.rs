use std::io;
use std::cell::RefCell;
use std::rc::Rc;

use Error;

use anymap::AnyMap;
use futures::{Future, future};
use core::net::TcpStream;
use core::reactor::Handle;
use proto::BindClient;
use tokio::{Service, NewService};
use c3po::{ConnFuture, Conn, Pool, Config};

use connections::{Client, ClientConnector, Configure};

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

    pub fn client<C, K>(&self) -> ConnFuture<Conn<ClientConnector<C, K>>, Error>
    where
        C: Client,
        C::Protocol: BindClient<K, TcpStream, BindClient = C::Connection>,
        C::Connection: Service<Request  = <C::Protocol as BindClient<K, TcpStream>>::ServiceRequest,
                               Response = <C::Protocol as BindClient<K, TcpStream>>::ServiceResponse,
                               Error    = <C::Protocol as BindClient<K, TcpStream>>::ServiceError>,
        K: 'static,
    {
        if let Some(pool) = self.pools.get::<Pool<ClientConnector<C, K>>>() {
            pool.connection()
        } else {
            future::Either::A(future::err(Error))
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
    where C: NewService + Configure + 'static
    {
        let client = C::new(&handle, client_cfg);
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
