# Structure of your application

If you look at the src directory you've created with `cargonauts new`, you'll
notice that it has created quite a few files for you. We're going to walk
through some of the most important ones now, and leave the rest of them for
later.

Here's the files and directories we'll be walking through in this section of
the book:

```
src
├── bin
│   └── server.rs
├── lib.rs
├── resources
│   └── mod.rs
├── routing.rs
└── templates
```

(If you look at the source yourself, you'll see that there are some additional
modules, but we won't be looking at them just yet.)

### `src/bin/server.rs`

The application created by `cargonauts new` has a split library/binary 
structure, like many Rust applications. The bulk of your application exists in
a library, but it is wrapped by a smaller binary that depends on it. This file
is the binary for your server.

The default server binary is very brief: it just runs your application using
`cargonauts::serve`. If you want to perform additional set up or tear down
around your application, you can do so by editing this file.

### `src/lib.rs`

This is the root of your application. It is also fairly brief: it activates the
`associated_consts` feature flag, declares a dependency on cargonauts, lists
your top level modules, and contains one re-export.

The re-export is worth noticing: we re-export `routing::routes`; this is the
item created by the `routes!` macro, which is a high level description of your
application. The `routes` item is the "entry-point" to your application which
the server binary uses to serve it.

### `src/routing.rs`

This file contains the `routes!` macro. At the top, it imports many of the
things you'll need to make your `routes!` macro work as you add to it, and then
it contains an empty macro, waiting for you to add routes to your application.

The `routes!` macro is the magic that binds cargonauts together. The rest of
your application is normal Rust code: you define types and implement traits for
them. But the `routes!` macro has its own language, which takes a high level
description of all of your endpoints and constructs your application from it.

One important aspect of the `routes!` macro is that every *CamelCase* name
inside it is just a type that has been imported into this module. All of the
imports at the top of this file bring in types that you'll be using in
describing in your routes - mainly the names of resources, methods, and
formats.

### The resources module

This is where you define your resources, and implement methods for them. This
is probably the module you'll turn to most often, at least at first.

In the next section, we're going to create a new resource and implement a
method for it, so we'll be editing code in this module quite a bit.

### The templates directory

This directory is for your templates. When formatting your resources into HTTP
responses, the format has access to the templates you define here in order to
render the response. Some formats use templates and some do not.

Templates are located at `templates/$resource/$method`, for example, something
like: `templates/user/get.html.hbs` (all file extensions are optional).

This is **not** a submodule of your Rust application; you should not put
Rust code in this directory, just templates.

## Running your application

As a final note about the structure of your application, you can run your
server any time with this command:

```
cargo run server
```

By default, your server will serve on port `7878`. Right now it should be
an empty application, so any request to `localhost:7878` should return a 404,
file not found error.
