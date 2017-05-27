# Structure of your application

If you've just created your application with `cargonauts new`, you may notice
that it created a lot of files for you. We'll walk through what each module
in your application is intended to be used for.

### `bin/server.rs`

This is the server that you're going to be building, running, and deploying.
Your main application is a library, and this is a binary that depends on it.
Right now it just servers your application, but you can add any code you want
here to do additional setup and teardown aroud your running application.

### `lib.rs`

This is the root of your application; it doesn't contain any code by default;
just extern crate and module declarations. It also re-exports the `routes` item
from your routing module; this is the public entrance to your entire
application.

### `routing.rs`

The routing file contains the `routes!` macro, which is how we stitch all of
the code in your application together. You'll edit this whenever you want to
add new endpoints to your application.

### The assets directory

This directory is for your static assets. By default, any assets here will be
built into your application at their path relative to the asset directory root.

Files starting with `index` will be served as the index for the directory they
are in, rather than at their filename.

This is not a submodule of your application; it does not contain Rust code.

### The clients module

This module is for defining clients, high level API wrappers around connections
to other running services.

### The formats module

This module is for defining custom formats. cargonauts comes with several
useful formats out of the box, but if you need to define your own, you can do
so here.

### The methods module

This module is for defining custom methods. cargonauts comes with many methods
covering most use case, but if you do need to define your own, this is the
module for them.

### The middleware module

This is for defining middleware to wrap around your endpoint. If you need to
deal directly with the HTTP requests and responses in a way that doesn't
integrate into the resource/method/format structure of cargonauts, you just
need to write a middleware for that.

You can use middleware for instrumenting endpoints, for example.

### The resources module

This is where you define your resources, and implement methods for them. This
is probably the module you'll turn to most often, at least at first.

### The templates directory

This directory is for your templates. When formatting your resources into HTTP
responses, the format has access to the templates you define here in order to
render the response. Some formats use templates and some do not.

Templates are located at `templates/$resource/$method`, for example, something
like: `templates/user/get.html.hbs` (all file extensions are optional).

Like the assets directory, this is not a part of your application, and you
shouldn't put any Rust code here.
