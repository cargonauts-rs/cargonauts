use std::io;
use std::net::SocketAddr;

use futures::{Future, Stream};
use hyper::server::Http;
use core::net::TcpListener;
use core::reactor::Core;

pub use hyper::header as headers;
pub use hyper::{StatusCode, Error, Method, Body};
pub use hyper::server::{Request, Response};
pub use core::reactor::Handle;
pub use tokio::{Service, NewService};

pub fn serve<T, Fut, F>(routes: F) -> io::Result<()>
where
    T: NewService<Request = Request, Response = Response, Error = Error> + 'static,
    T::Instance: 'static,
    Fut: Future<Item = T, Error = io::Error>,
    F: FnOnce(&Handle) -> (SocketAddr, Fut),
{
    // TODO shutdown
    let mut core = Core::new()?;
    let protocol = Http::new();
    let handle = core.handle();
    let (addr, routes) = routes(&handle);
    let listener = TcpListener::bind(&addr, &handle)?;
    let srv = routes.and_then(|routes| {
        let handle = &handle;
        let protocol = &protocol;
        listener.incoming().for_each(move |(socket, remote_addr)| {
            routes.new_service().map(move |service| {
                protocol.bind_connection(handle, socket, remote_addr, service);
            })
        })
    });
    core.run(srv)
}

pub type BoxFuture = Box<Future<Item = Response, Error = Error>>;
