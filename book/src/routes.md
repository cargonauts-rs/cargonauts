# The routes! DSL Reference

Resources, methods, and formats are just normal Rust code, but they are brought
together in the `routes!` macro, which implements a complicated domain specific
language for assembling your application in a declarative way.

Just implementing a method for a resource isn't enough to create an endpoint;
it also needs to be added to your `routes!` macro, which looks like this:

```rust
use cargonauts::methods::Get;
use cargonauts::formats::JsonApi;

use resources::User;

routes! {
    resource User {
        method Get in JsonApi;
    }
}
```

Having implemented Get for User (as well as the traits required to format a
User with JsonApi), this will create an endpoint for `GET /user/$id`. If you
haven't implemented all of the necessary traits, you will get a compiler error.

Every CamelCase name in the `routes!` macro is just a type imported into this
scope; there's no magic lookup for User, Get, or JsonApi, you have to have
imported them into this module in order to use them in the DSL.

The rest of this page will document the syntax for the `routes!` DSL.

### Resource blocks

A resource block looks like this:

```
resource $TYPE {
    ...
}
```

The $TYPE must implement the Resource trait.

By default, resources will be mounted at the kebab case form of the type
name - that is, all lowercase, joined by hyphens. For example, `User` will
be mounted at `user`, whereas a resource called `HappyDoggo` would be mounted
at `happy-doggo`.

If you don't like that route choice, you can use an `as` clause to rename it.
This takes a string literal. For example, you might prefer plural route names;
cargonauts will not automatically inflect your names for you, you'll have to
specify them:

```
resource Person as "people" {
    ...
}

resource Cat as "cats" {

}


resource Doggo as "puppers" {

}
```


### Method statements

Within a resource block, you can declare the methods that resource supports.
A method takes the syntax:

```
method $METHOD in $FORMAT;
```

Where $METHOD is a method trait, and $FORMAT is a type which implements Format.

You can have multiple methods using the same format on the same line, separated
with commas, or not:

```
method Get, Index, Post in JsonApi;
method Patch in Handlebars;
method Delete in Handlebars;
```

Currently each method can only be provided in one format, someday this will
change to support dispatching to different formats based on the Accepts
headers.

### Relationship blocks

Resources can also contain relationships, which can be either `has one` or
`has many`:

```
resource BlogPost {
    has one Author {
        ... 
    }

    has many Comment {
        ...
    }
}
```

The type of a relationship block must implement Relationship. Like resource
blocks, these support aliases with `as`, and they can contain method
statements. Their method statements must be relationship methods:

```
resource BlogPost {
    has many Comment as "comments" {
        method GetMany in JsonApi;
    }
}
```

### Modules

Resources can be put into inline modules; modules use the same syntax as in
regular Rust, and can be nested:

```
mod api {
    mod foo_bar {
        resource Baz {
            ...
        }
    }
}
```

These will create directories in the API; for example, Baz's endpoints would
be mounted at `api/foo-bar/baz`. Note that modules, like types, are kebab
cased in the actual API.

These are only used to control the shape of the API; there is no namespacing
or use statements inside the `routes!` DSL.

### Setup block

The `routes!` DSL can also begin with a `setup` block. This block is used to
set up connections to other services when the application starts.

The setup block looks like this:

```
setup {
    connection to $SERVICE;
}
```

$SERVICE must be a type which implements NewService and Configure. Connections
to this service will be managed by a connection pool which you can configure
with your Cargo.toml.

### Asset handler

By default, assets will be presented with a very simple asset handler,
providing few headers or other processing. If you wish to perform more complex
handling for your assets, you can define an asset hnadler function. This
function must have the signature:

```
fn(path: &'static Path, asset: &'static [u8], req: middleware::Request) -> middleware::http::BoxFuture
```

You can tell cargonauts to use this handler instead of the default with this
statement at the beginning of the `routes!` DSL:

```
use $ASSET_HANDLER for assets;
```
