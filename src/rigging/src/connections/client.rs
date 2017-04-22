use std::io;
use std::ops::Deref;

use futures::Future;
use core::reactor::Handle;
use tokio::{Service, NewService};

use connections::Configure;

pub trait Client: {
    type Connector: NewService<Instance = Self::Connection>;
    type Connection: Service<Request = <Self::Connector as NewService>::Request,
                             Response = <Self::Connector as NewService>::Response,
                             Error = <Self::Connector as NewService>::Error>;
    fn connect(conn: Self::Connection) -> Self;
    fn conn(&self) -> &Self::Connection;
}

pub struct ClientService<C>(C);

impl<C: Client> Service for ClientService<C> {
    type Request = <C::Connection as Service>::Request;
    type Response = <C::Connection as Service>::Response;
    type Error = <C::Connection as Service>::Error;
    type Future = <C::Connection as Service>::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.0.conn().call(req)
    }
}

impl<C: Client> Deref for ClientService<C> {
    type Target = C;
    fn deref(&self) -> &C {
        &self.0
    }
}


pub struct ClientConnector<C: Client> {
    connector: C::Connector,
}

impl<C, Conn> NewService for ClientConnector<C>
where
    C: Client<Connector = Conn>,
    Conn: NewService<Instance = C::Connection>,
    Conn::Future: 'static,
{
    type Request = Conn::Request;
    type Response = Conn::Response;
    type Error = Conn::Error;
    type Instance = ClientService<C>;
    type Future = Box<Future<Item = ClientService<C>, Error = io::Error>>;

    fn new_service(&self) -> Self::Future {
        Box::new(self.connector.new_service().map(|conn| ClientService(C::connect(conn))))
    }
}

impl<C, Conn> Configure for ClientConnector<C>
where
    C: Client<Connector = Conn>,
    Conn: NewService<Instance = C::Connection> + Configure,
{
    type Config = Conn::Config;
    
    fn new(handle: &Handle, cfg: Self::Config) -> Self {
        ClientConnector {
            connector: Conn::new(handle, cfg),
        }
    }
}
