use std::time::{Instant};
use cargonauts::futures::Future;
use cargonauts::middleware::*;

#[derive(Default)]
pub struct Logging;

impl<S> Middleware<S> for Logging
where
    S: Service<Request = Request, Response = http::Response, Error = http::Error, Future = http::BoxFuture>
{
    type WrappedService = LoggedService<S>;
    fn wrap(self, service: S) -> Self::WrappedService {
        LoggedService(service)
    }
}

pub struct LoggedService<S>(S);

impl<S> Service for LoggedService<S>
where
    S: Service<Request = Request, Response = http::Response, Error = http::Error, Future = http::BoxFuture>
{
    type Request = Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Request) -> Self::Future {
        let time = Instant::now();
        println!("{} {}", req.req.method(), req.req.path());
        Box::new(self.0.call(req).map(move |response| {
            let elapsed = time.elapsed();
            let ms = elapsed.as_secs() * 1000 + (elapsed.subsec_nanos() as u64) / 1000;
            println!("Finished in {} ms.", ms);
            response
        }))
    }
}
