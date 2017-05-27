# A very small application

To bring the core concepts we've just introduced together, let's look at very
small, single-file application.

This application does one thing: it has an endpoint at `/echo/$STRING`, and
will respond to GET requests at that endpoint by Debug printing an Echo object
with that string into its body. Here's the code:

```rust
#![feature(associated_consts)]

#[macro_use] extern crate cargonauts;

use cargonauts::resources::{Resource, Environment, Error};
use cargonauts::methods::Get;
use cargonauts::formats::Debug;
use cargonauts::futures::{future, Future};

#[derive(Debug)]
struct Echo {
    echo: String,
}

impl Resource for Echo {
    type Identifier = String;
}

impl Get for Echo {
    fn get(id: String, _: Environment) -> Box<Future<Item = Echo, Error = Error>> {
        future::ok(Echo { echo: id }).boxed()
    }
}

routes! {
    resource Echo {
        method Get in Debug;
    }
}

fn main() {
    cargonauts::serve(routes).unwrap();
}
```

Let's walk through this section by section.

### Top matter

```rust
#![feature(associated_consts)]
```

cargonauts depends on associated consts, which is why it is on nightly. When I
made this choice, I thought associated consts would be stable before I had
cargonauts ready to show people, but I was wrong.

```rust
#[macro_use] extern crate cargonauts;
```

We need to depend on cargonauts, and we have to apply macro_use to it or we
can't get the `routes!` macro.

```rust
use cargonauts::resources::{Resource, Environment, Error};
use cargonauts::methods::Get;
use cargonauts::formats::Debug;
use cargonauts::futures::{future, Future};
```

These are all the imports we need for this to work:

* Resource is needed so Echo can implement it; Environment and Error appear in
any method impl.
* Get is needed because its the method we'll be implemented.
* Debug is needed because its the format we'll be using.
* future and Future are used in our implementation of Get.

### Resource type

```rust
#[derive(Debug)]
struct Echo {
    echo: String,
}
```

This is our resource type in this case. Its a simple type; it only has one
field.

Echo derives Debug because the Debug format requires that.

```rust
impl Resource for Echo {
    type Identifier = String;
}
```

Echo needs to implement Resource. Its Identifier in this case is a String (the
string we'll be echoing back).

```
impl Get for Echo {
    fn get(id: String, _: Environment) -> Box<Future<Item = Echo, Error = Error>> {
        future::ok(Echo { echo: id }).boxed()
    }
}
```

This is our single endpoint: we receive a string and then construct an Echo
from it to return. This can never fail and does no IO, but it needs to be
wrapped in a future because cargonauts is an async framework.

### Routes and main

```rust
routes! {
    resource Echo {
        method Get in Debug;
    }
}
```

The routes DSL here has only one resource and one method. This constructs our
actual app, hooking up the endpoint we defined. This macro ultimately expands
to an item named `routes`.

```rust
fn main() {
    cargonauts::serve(routes).unwrap();
}
```

The main function serves our app uing the `serve` function from cargonauts,
which can take a `routes` object.
