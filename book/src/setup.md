# Starting a new cargonauts app

## Install Rust

This guide assumes that you have both Rust and cargo installed on your
computer. I recommend using [rustup.rs](https://rustup.rs/), which makes it
very easy to manage multiple installations of Rust.

cargonauts currently requires the **nightly** version of Rust. If you don't
have that installed, you can use rustup to install it:

```
rustup install nightly
```

## Install `cargonauts-cli`

To quickly get started with cargonauts, you should install the `cargonauts-cli`
crate. This is just a tiny app which helps you create a cargonauts app with
your source directory set up correctly.

You can install it with cargo:

```
cargo install cargonauts-cli
```

This gives you a new command, `cargonauts`, which currently only has one
subcommand, which we'll be using in a second to create your app.

## Create your project

To create your project, just run this command (where `<my-app>` is the name of
your app):

```
cargonauts new <my-app>
```

This will initiate an app in a directory with the same name, which will have
a source directory set up to make cargonauts easy to use.

## Build your dependencies

Now, you should build all the dependencies of cargonauts. Unfortunately, this
takes quite a while right now.

First, make sure you are using the nightly compiler:

```
rustup override set nightly
```

Then, do a fresh build of your cargonauts dependency:

```
cargo build -p cargonauts
```

## Running your application

You're all set. If you want to run your application, you can do so with this
command:

```
cargo run server
```
