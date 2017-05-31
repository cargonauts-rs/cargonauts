# Setting up a new cargonauts project

## Install nightly Rust

This guide assumes you have both Rust and cargo. Specifically, cargonauts
currently depends on the **nightly** version of Rust. You need to get the
nightly Rust compiler on your machine in order to use cargonauts.

If you don't have Rust yet, I recommend installing it using
[rustup.rs](https://rustup.rs/), which will help you handle having multiple
versions of Rust installed at the same time. Once you've installed rustup,
you can get the nightly version of Rust with this command:

```
rustup install nightly
```

## Install `cargonauts-cli`

To quickly get started with cargonauts, you should install the `cargonauts-cli`
crate. This is just a tiny app which helps you generate the source code for
your app, it isn't involved in running it.


You can install the CLI with cargo:

```
cargo install cargonauts-cli
```

This gives you a new command, `cargonauts`.

## Create the echo project

Now that we have the CLI, we'll use it to create our application, which we're
going to call "echo". The command to create our app is this:

```
cargonauts new echo
```

This will create a new Rust application in the `echo` directory. Move into that
directory and make sure that you'll be using the nightly compiler with rustup:

```
cd ./echo
rustup override set nightly
```

### Build your application

Now, to make sure that everything is in order, let's build the application with
`cargo build`.

Unfortunately, and despite our best intentions, building all of cargonauts'
dependencies for the first time will take several minutes. Feel free to treat
yourself to a refreshment, or do something you like to do, while your waiting
for all of our dependencies to build.

If you're looking for something to do, feel free to advance to the
[next section](./structure.md) of this book, where we'll explore some of the
files that we've created so far. You can read through this section before your
app has finished building.

If your app doesn't build correctly, please visit the
[gitter channel](https://gitter.im/cargonauts-rs/cargonauts) for help.
