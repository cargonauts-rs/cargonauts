# Resources, methods and formats

cargonauts is a **resource-oriented** and **RESTful** framework. This means
that you will model the surface of your application in terms of resources
which have behaviors, rather than as "remote procedural call" (RPC) functions.

The three most important building blocks in cargonauts **resources**,
**methods**, and **formats**. Every endpoint in your application is constructed
by combining a resource, a method, and a format.

[TODO summary of the concept of each of these]

[TODO you can define your own formats and methods but we give you several]

These aren't the only building blocks that cargonauts gives you, but they are
the most important. Most of the code you will write for cargonauts will be in
implementing methods for resources.
