This is a todomvc demo.

The frontend is implemented with ember and stored in the `frontend` directory.

There are two recommended ways to run the app:

* Using the ember hot reloading proxy: `cargo run server --features proxy`
    - The server will be available at `localhost:4200`
* Building into a single binary (with optimizations): `cargo run server --release`
    - The server will be available at `localhost:7878`
