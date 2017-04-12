# cargonauts - a Rust web framework

**cargonauts** is a Rust web framework intended for building long lasting,
maintainable web services.

**This project is an incomplete work in progress. Please do not attempt to use
it yet.**


```rust
#[macro_use]
extern crate cargonauts;

use cargonauts::api::{Resource, Get, Environment, Error};
use cargonauts::api::relations::GetOne;
use cargonauts::format::Debug;
use cargonauts::futures::{Future, BoxFuture, future};

#[derive(Debug)]
pub struct MyResource { 
    slug: String,
}

impl Resource for MyResource {
    type Identifier = String;
    fn identifier(&self) -> String { self.slug.clone() }
}

impl Get for MyResource {
    fn get(slug: String, _: Environment) -> BoxFuture<MyResource, Error> {
        future::ok(MyResource { slug }).boxed()
    }
}

relation!(AllCaps => MyResource);

impl GetOne<AllCaps> for MyResource {
    fn get_one(slug: String, _: Environment) -> BoxFuture<MyResource, Error> {
        future::ok(MyResource { slug: slug.to_uppercase() }).boxed()
    }
}

routes! {
    resource MyResource {
        method Get in Debug;

        relation AllCaps {
            method GetOne in Debug;
        }
    }
}

fn main() {
    let socket_addr = "127.0.0.1:8000".parse().unwrap();
    cargonauts::server::Http::new().bind(&socket_addr, routes).unwrap().run().unwrap();
}
```

## License

Cargonauts is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

[json-api]: http://jsonapi.org
