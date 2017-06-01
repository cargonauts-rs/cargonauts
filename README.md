# cargonauts - a Rust web framework

[![Join the chat at https://gitter.im/cargonauts-rs/cargonauts](https://badges.gitter.im/cargonauts-rs/cargonauts.svg)](https://gitter.im/cargonauts-rs/cargonauts?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[Documentation](cargonauts-rs.github.io/cargonauts)

**cargonauts** is a Rust web framework intended for building maintainable,
well-factored web apps.

**This project is a work in progress. It has not been hardened or audited for
security. Expect frequent breaking changes. Use at your own risk.**

## Core concepts

* **REST-first & resource oriented:** cargonauts is designed for constructing
applications as networks of related resources which support methods. It comes
with a JSON API hypermedia API format.
* **Cleanly decoupled & well-factored:** cargonauts provides many different
building blocks for your application, with clear interfaces between them. Used
correctly, it can help you keep your app from turning into a pile of spaghetti.
* **Async always:** cargonauts is built on top of tokio; your application will
use asynchronous IO out of the box.

## License

Cargonauts is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

## Code of Conduct

This project is governed by the Code of Conduct, found in the root of this
repository. The Code of Conduct has been adapted from the Contributor
Covenant.
