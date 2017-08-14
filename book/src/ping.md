# Creating our first endpoint

Now that our application is set up, its time to actually implement some
behavior. We're going to write the single endpoint we described earlier:

```
GET /ping/$slug
```

### Committing

Before we get to that though, its a good idea to commit your repository so
that you can revert to it if things go bad. You'll also be able to use
`git diff` to view the changes you've made at each step of the documentation.

```
git add --all
git commit -m "An empty cargonauts app, ripe with potential."
```

## Creating our resource

The first thing we need to create an endpoint is a resource. Because creating
resources is a very common thing to do, the cargonauts command has a subcommand
which will create a new resource for you. This command is aptly named
`generate resource`. Let's create a resource called ping:

```
cargonauts generate resource ping
```

Feel free to use git to view the changes, the generator makes these three
changes:

* We add a new module to the resource module. This contains the beginnings of
the definition of our new resource.
* We re-export that module from the top-level resource module.
* We use that resource in the `routes!` macro.

There's one important problem though: the definition of a resource generated
by the scaffolding is incomplete. For this reason, if you try to build your
application right now, you will get some errors.

We're going to fix that, but before we go any further, let's look at the
change to the routes! macro.

### Changes to `routing.rs`

Our `routes!` DSL now has its first declaration:

```rust
routes! {
    resource Ping {
    }
}
```

The `routes!` DSL consists mostly of "resource objects" like this - there will
be one for each resource in your application, and it will contain various
additional items, mainly `method` statements, which we'll see in a bit.

### Finishing the Resource impl

If you open `src/resources/ping.rs`, you'll see that it contains this impl:

```rust
impl Resource for Ping {
    type Identifier = (); // TODO: Select the identifier type for this resource
}
```

This impl contains an error: `()` is not a valid Identifier type. This is
because identifiers have to implement both `ToString` and `FromStr`, and `()`
does not.

The identifier type is parsed from the URL of the resource. When users request
`/ping/$slug`, it is `$slug` that is parsed into the identifier. If the slug
is not a valid string representation of that identifier type, users receive an
error response.

For our purposes, in which the slug can be anything, a `String` is a fine
representation of the slug, so let's make that change:

```rust
impl Resource for Ping {
    type Identifier = String;
}
```

### Adding data to Ping

We said earlier that the response from a ping request would have two things:

* The slug requested
* A timestamp

The `Ping` type will be used to represent this data, so it needs to have a
field for each of these.

We've already decided that the slug is represented as a String. To represent
the timestamp, we can use the `chrono` crate.

Add a dependency on [chrono](https://crates.io/crates/chrono),
and define your struct:

```rust
use chrono::{DateTime, Utc};

pub struct Ping {
    slug: String,
    timestamp: DateTime<Utc>,
}
```

## Adding a method to Ping

Even though your app has a resource now, it *still* doesn't do anything! This
is because we haven't implemented any methods for that resource. A resource
without methods is like an object without methods, it doesn't actually do
anything.

We're going to make the `Ping` type implement the `Get` method. This is the
method that corresponds to `GET /$resource/$identifer`, so its exactly what
we need to implement what we said.

We don't have a generator for this, so we'll do it all by hand.

### Adding the impl

The `Get` trait can be found in `cargonauts::methods`; we need to import it
and implement it for `Ping`. We also need to import from the futures crate
(which cargonauts re-exports), so that we can create a future. We'll start by
making our endpoint panic:

```rust
use cargonauts::methods::Get;
use cargonauts::futures::*;

impl Get for Ping {
    fn get(slug: String, _: Environment) -> Box<Future<Item = Ping, Error = Error>> {
        panic!()
    }
}
```

Creating our resource doesn't involve any IO or complex method calls; all we
need to do is construct our resource and wrap `future::ok().boxed()` around it,
like so:

```rust
future::ok(Ping {
    slug: slug,
    timestamp: Utc::now(),
}).boxed()
```

That's it! Ping implements Get.

### Deciding what format to use

The other thing we need to do before our endpoint is complete is to decide
which format we will use to display this method. cargonauts comes with three
formats out of the box:

* **Debug:** This format prints your type into the response using the `Debug`
trait from the standard library. As its name implies, its intended for
debugging, not as much for production code.
* **JsonApi:** An implementation of the [JSON API](http://jsonapi.org/) spec,
this provides a JSON exchange format for machine consumption.
* **Handlebars:** This renders your response from a template using the
handlebars templating language; this is intended for server side rendering of
HTML.

For our purposes, we're going to use `Debug`, because its the easiest to use
for examples like this. Each format will have its own additional requirements
for resources to be formatted with it. In the case of `Debug`, the resource
type needs to implement the `Debug` trait from the standard library.

We can achieve that by deriving `Debug` for `Ping`:

```rust
#[derive(Debug)]
struct Ping {
    slug: String,
    timestamp: DateTime<Utc>,
}
```

### Adding the method to the `routes!` macro

Though Ping implements Get, the endpoint still doesn't exist. The last thing
you need to do is declare your method in the routes! DSL. This check keeps all
of your routes in one place, so you don't have to trace impls all over your
code to figure out what routes your application has.

The syntax for creating a new route is like this:

```rust
resource Ping {
    method Get in Debug;
}
```

Every method you add to a resource will look like the same, a method name and
a format the method is to be displayed in. Like the resource, these are just
types that need to be in scope.
