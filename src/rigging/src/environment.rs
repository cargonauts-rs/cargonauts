use std::any::Any;
use std::io;
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

use Error;

use anymap::AnyMap;
use core::reactor::Handle;
use c3po::{ConnFuture, Conn, Pool, Config};
use futures::{Future, future, Stream, stream};
use ref_filter_map::*;
use tokio::NewService;

use http::StatusCode;
use connections::{Client, Configure, ConnectClient};

#[derive(Clone)]
pub struct Environment {
    pools: Rc<AnyMap>,
    store: Rc<RefCell<AnyMap>>,
}

impl Environment {
    pub fn store<T: Any>(&mut self, val: T) {
        self.store.borrow_mut().insert(val);
    }

    pub fn get<T: Any>(&self) -> Option<Ref<T>> {
        ref_filter_map(self.store.borrow(), |map| map.get())
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<RefMut<T>> {
        ref_mut_filter_map(self.store.borrow_mut(), |map| map.get_mut())
    }

    pub fn take<T: Any>(&mut self) -> Option<T> {
        self.store.borrow_mut().remove()
    }

    pub fn conn<C: NewService>(&self) -> ConnFuture<Conn<C>, Error> {
        if let Some(pool) = self.pools.get::<Pool<C>>() {
            pool.connection()
        } else {
            future::Either::A(future::err(Error::with_msg(StatusCode::InternalServerError,
                                                          "Connection not registered.")))
        }
    }

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
}

impl PreparedEnv {
    pub fn new(&self) -> Environment {
        Environment {
            pools: self.pools.clone(),
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
        }
    }
}
