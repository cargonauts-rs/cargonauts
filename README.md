# cargonauts - easy REST API in Rust

**cargonauts** is a fun and easy way to write a REST API in Rust. With a macro
& some trait impls, you can quickly construct a RESTful HTTP/JSON API for your
Rust network service. Your API will be guaranteed to conform to the
[JSON API][json-api] spec, and you will get many features of this spec out of
the box for free. Instead of dealing with Request & Response types and
worrying about the shape of your API, just write code.


**This project is an incomplete work in progress. Please do not attempt to use
it yet.**

## `routes!` macro - define your endpoints

To use cargonauts, the first thing you need do is define your API endpoints
using the `routes!` macro, like this:

```rust
routes! {
    resource User => ["get", "put", "patch"] {
        related Post: "has-many";
        related Comment: "has-many";
    }
    resource Post => ["get", "index", "put", "patch"] {
        related User: "has-one";
        related Comment: "has-many";
    }
    resource Comment => ["index", "put"] {
        related User: "has-one";
        related Post: "has-one";
    }
}
```

**TODO** explain the macro syntax.

This macro will expand to a function called `attach_routes`, which will be
applied to your server router in the `boot.rs` setup, attaching all of the
routes for all of the endpoints you described.

## Implement API traits

In order for this macro to expand successfully, you must have implemented all
of the methods for each of your end point types. Every resource type must
implement the `Resource` trait. Then, for each method you declare that resource
as providing, you must implement the corresponding trait (that is, if you
say resource has a `"get"` method, you must implement the `Get` trait). Lastly,
for every relation of that resource, you must implement `HasOne` or `HasMany`,
parameterized by the type of that relation.

Thus, the macro in the previous section corresponds to these trait impls:

* `User` - `Resource`, `Get`, `Put`, `Patch`, `HasMany<Post>`, `HasMany<Comment>`
* `Post` - `Resource`, `Get`, `Index`, `Put`, `Patch`, `HasOne<User>`, `HasMany<Comment>`
* `Comment` - `Resource`, `Index`, `Put`, `HasOne<User>`, `HasOne<Post>`

## `boot.rs` - set up your server

**TODO**

## TODO List

Some of many things still left todo:

 - [X] GET resource
 - [X] Relationships
 - [X] Includes
 - [ ] Recursive includes
 - [X] INDEX resource
 - [ ] Filter
 - [X] POST resource
 - [ ] POST resource without object
 - [ ] POST resource with UUID
 - [X] POST resource returning NoContent
 - [ ] POST resource with included relationship objects??
 - [X] PATCH resource
 - [ ] POST-ASYNC resource
 - [ ] PATCH-ASYNC resource
 - [ ] DELETE requests
 - [ ] HEAD requests
 - [X] GET relationships
 - [ ] POST/PATCH relationships
 - [ ] DELETE relationships
 - [ ] Raw methods
 - [ ] Sparse fieldsets
 - [ ] Pagination
 - [ ] Custom relation names & pluralization issues.
 - [ ] ...much more

## License

Cargonauts is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

[json-api]: http://jsonapi.org
