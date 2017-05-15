This is a todomvc demo.

The frontend is implemented with ember and stored in the `frontend` directory.

There are two recommended ways to run the app:

* Using the ember hot reloading proxy: `cargo run server --features proxy`
    - The server will be available at `localhost:4200`
* Building into a single binary (with optimizations): `cargo run server --release`
    - The server will be available at `localhost:7878`

## Dependencies

You must have ember-cli installed to build cargonauts in either proxy or release mode.

You also must have a redis instance running on your machine (this app persists to redis,
a sort of bad idea but w/e, its a demo). The app eneds to have the environment variable
`REDIS_HOST` set to the URL for redis (probably this is `tcp://127.0.0.01:6379`). The
app will panic at start up if you don't do this.
