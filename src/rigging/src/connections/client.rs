use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;

use proto::{TcpClient, BindClient, BoundTcpClient};

use futures::Future;
use core::reactor::Handle;
use core::net::TcpStream;
use tokio::{Service, NewService};

use connections::Configure;

pub trait Client {
    type Connection: Service;
    fn connect(conn: Self::Connection) -> Self;
    fn conn(&self) -> &Self::Connection;
}

pub struct ClientService<C>(C);

pub struct ClientConnector<C, P, Kind> {
    tcp: BoundTcpClient<Kind, P>,
    _marker: PhantomData<C>,
}


impl<C: Client> Service for ClientService<C> {
    type Request = <C::Connection as Service>::Request;
    type Response = <C::Connection as Service>::Response;
    type Error = <C::Connection as Service>::Error;
    type Future = <C::Connection as Service>::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.0.conn().call(req)
    }
}

impl<C, P, Conn, Kind> NewService for ClientConnector<C, P, Kind>
where
    C: Client<Connection = Conn>,
    P: BindClient<Kind, TcpStream, BindClient = Conn>,
    Conn: Service<Request = P::ServiceRequest, Response = P::ServiceResponse, Error = P::ServiceError>,
    Kind: 'static
{
    type Request = Conn::Request;
    type Response = Conn::Response;
    type Error = Conn::Error;
    type Instance = ClientService<C>;
    type Future = Box<Future<Item = ClientService<C>, Error = io::Error>>;

    fn new_service(&self) -> Self::Future {
        Box::new(self.tcp.new_service().map(|conn| ClientService(C::connect(conn))))
    }
}

impl<C, Kind, P> Configure for ClientConnector<C, P, Kind>
where
    C: Client,
    P: BindClient<Kind, TcpStream> + Configure,
{
    type Config = (P::Config, SocketAddr);
    
    fn new(handle: &Handle, cfg: Self::Config) -> Self {
        let (cfg, addr) = cfg;
        let proto = P::new(handle, cfg);
        ClientConnector {
            tcp: TcpClient::new(proto).bind(addr, handle.clone()),
            _marker: PhantomData,
        }
    }
}
