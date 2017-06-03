use std::any::Any;
use std::io;
use std::cell::RefCell;
use std::rc::Rc;

use anymap::AnyMap;
use core::reactor::Handle;
use c3po::{ConnFuture, Conn, Pool, Config};
use futures::{IntoFuture, Future, future, Stream, stream};
use futures_cpupool::{CpuPool, CpuFuture};
use num_cpus;
use tokio::NewService;
use r2d2::{ManageConnection, Pool as SyncPool, PooledConnection};

use Error;
use http::StatusCode;
use connections::{Client, Configure, ConnectClient};

#[derive(Clone)]
/// The Environment in which an API endpoint runs.
///
/// The resource method, format, and middlware of an endpoint all have access
/// to this environment. It is used for a few basic purposes:
/// * It controls access to the connections to other network services.
/// * It allows you to cache values in a type map to share them from the
/// format/middleware/resource.
pub struct Environment {
    cpu_pool: Rc<CpuPool>,
    pools: Rc<AnyMap>,
    store: Rc<RefCell<AnyMap>>,
}

pub type SyncConnFuture<T, E> = future::Either<CpuFuture<T, E>, future::FutureResult<T, E>>;

impl Environment {
    /// Store a new value in the environment.
    ///
    /// If a value of this type was already stored in the environment, that
    /// previous value is returned.
    pub fn store<T: Any>(&self, val: T) -> Option<T> {
        self.store.borrow_mut().insert(val)
    }

    /// Take a value of a type from the environment.
    pub fn take<T: Any>(&self) -> Option<T> {
        self.store.borrow_mut().remove()
    }

    /// Check if the environment contains a value of the given type.
    pub fn contains<T: Any>(&self) -> bool {
        self.store.borrow().contains::<T>()
    }

    /// Check out a connection to a service from the connection pool.
    pub fn conn<C: NewService>(&self) -> ConnFuture<Conn<C>, Error> {
        if let Some(pool) = self.pools.get::<Pool<C>>() {
            pool.connection()
        } else {
            future::Either::A(future::err(Error::with_msg(StatusCode::InternalServerError,
                                                          "Connection not registered.")))
        }
    }

    pub fn on_pool<T, R, F>(&self, f: F) -> CpuFuture<T, Error>
    where
        T: Send + 'static,
        F: FnOnce() -> R + Send + 'static,
        R: IntoFuture<Item = T, Error = Error> + Send + 'static,
        R::Future: Send + 'static,
    {
        self.cpu_pool.spawn_fn(f)
    }

    pub fn sync_conn<C, T, R, F>(&self, f: F) -> SyncConnFuture<T, Error>
    where
        C: ManageConnection,
        T: Send + 'static,
        F: FnOnce(PooledConnection<C>) -> R + Send + 'static,
        R: IntoFuture<Item = T, Error = Error> + Send + 'static,
        R::Future: Send + 'static,
    {
        if let Some(pool) = self.pools.get::<SyncPool<C>>() {
            let conn = match pool.get() {
                Ok(conn) => conn,
                Err(err) => {
                    return future::Either::B(future::err(Error::from_err(err, StatusCode::InternalServerError)))
                }
            };
            future::Either::A(self.on_pool(move || f(conn)))
        } else {
            return future::Either::B(future::err(Error::with_msg(StatusCode::InternalServerError,
                                                                 "Connection not registered.")))
        }
    }
    
    #[doc(hidden)]
    pub fn conn_for<C: NewService>(&self, name: &'static str) -> ConnFuture<Conn<C>, Error> {
        if let Some(pools) = self.pools.get::<Vec<(&'static str, Pool<C>)>>() {
            if let Some(&(_, ref pool)) = pools.iter().find(|&&(s, _)| s == name) {
                pool.connection()
            } else {
                future::Either::A(future::err(Error::with_msg(StatusCode::InternalServerError,
                                                              "Connection not registered.")))
            }
        } else {
            future::Either::A(future::err(Error::with_msg(StatusCode::InternalServerError,
                                                          "Connection not registered.")))
        }
    }

    /// Check out a high level client from the environment.
    pub fn client<C>(&self) -> Box<Future<Item = C, Error = Error>>
    where
        C: ConnectClient<<C as Client>::Connection>,
    {
        if let Some(pool) = self.pools.get::<Pool<C::Connection>>() {
            Box::new(pool.connection().map(|c| C::connect(c)))
        } else if let Some(pools) = self.pools.get::<Vec<(&'static str, Pool<C::Connection>)>>() {
            if let Some(&(_, ref pool)) = pools.iter().find(|&&(s, _)| s == C::CONNECTION_NAME) {
                Box::new(pool.connection().map(|c| C::connect(c)))
            } else {
                Box::new(future::err(Error::with_msg(StatusCode::InternalServerError,
                                                     format!("Connection not registered: {}", C::CONNECTION_NAME))))
            }
        } else {
            Box::new(future::err(Error::with_msg(StatusCode::InternalServerError,
                                                 format!("Connection not registered: {}", C::CONNECTION_NAME))))
        }
    }
}

#[derive(Clone)]
pub struct PreparedEnv {
    pools: Rc<AnyMap>,
    cpu_pool: Rc<CpuPool>,
}

impl PreparedEnv {
    pub fn new(&self) -> Environment {
        Environment {
            pools: self.pools.clone(),
            cpu_pool: self.cpu_pool.clone(),
            store: Rc::new(RefCell::new(AnyMap::new())),
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
        let client = C::config(client_cfg, handle.clone());
        Box::new(Pool::new(client, handle, cfg).map(move |pool| {
            self.pools.borrow_mut().insert(pool);
        }))
    }

    // For multiple connections of the same protocol
    pub fn new_pool_vec<C>(self, handle: Handle, cfgs: Vec<(&'static str, Config, C::Config)>)
        -> Box<Future<Item = (), Error = io::Error>>
    where C: NewService + Configure + 'static
    {
        Box::new(stream::futures_unordered(cfgs.into_iter().map(|(name, cfg, client_cfg)| {
            let client = C::config(client_cfg, handle.clone());
            Pool::new(client, handle.clone(), cfg).map(move |pool| (name, pool))
        })).collect().map(move |pools| {
            self.pools.borrow_mut().insert(pools);
        }))
    }

    pub fn build(self) -> PreparedEnv {
        PreparedEnv {
            pools: Rc::new(Rc::try_unwrap(self.pools).unwrap().into_inner()),
            cpu_pool: Rc::new(CpuPool::new(num_cpus::get() * 4)),
        }
    }
}
