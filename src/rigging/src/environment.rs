use std::io;
use std::rc::Rc;

use anymap::AnyMap;
use core::net::TcpStream;
use proto::BindClient;
use tokio::{Service, NewService};
use c3po::{ConnFuture, Conn, Pool};

use connections::{Client, ClientConnector};

#[derive(Clone)]
pub struct Environment {
    pools: Rc<AnyMap>,
}

impl Environment {
    pub fn conn<C: NewService>(&self) -> ConnFuture<Conn<C>, io::Error> {
        if let Some(pool) = self.pools.get::<Pool<C>>() {
            pool.connection()
        } else {
            panic!()
        }
    }

    pub fn client<C, K>(&self) -> ConnFuture<Conn<ClientConnector<C, K>>, io::Error>
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
            panic!()
        }
    }
}

pub struct EnvBuilder {
    pools: AnyMap,
}

impl EnvBuilder {
    pub fn new() -> EnvBuilder {
        EnvBuilder {
            pools: AnyMap::new(),
        }
    }

    pub fn insert_pool<C: NewService + 'static>(&mut self, pool: Pool<C>) {
        self.pools.insert(pool);
    }

    pub fn build(self) -> Environment {
        Environment {
            pools: Rc::new(self.pools),
        }
    }
}
