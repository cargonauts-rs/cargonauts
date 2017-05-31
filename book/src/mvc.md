# Resources, methods and formats

cargonauts is a **resource-oriented** and **RESTful** framework. This means
that you will model the surface of your application in terms of resources
which have behaviors, rather than as "remote procedural call" (RPC) functions.

The three most important building blocks in cargonauts **resources**,
**methods**, and **formats**. Every endpoint in your application is constructed
by combining a resource, a method, and a format.

## Resources

Resources are the primary construct you'll be using to define your application.
A resource is a type which is used to collect the data yielded by a set of
endpoints. Every resource implements the `Resource` trait. A resource might
look like this:

```rust
struct User {
    user_id: Uuid,
    username: String,
    email: String,
    is_admin: bool,
    joined_at: DateTime<UTC>,
    last_logged_in: DateTime<UTC>,
}


impl Resource for User {
    type Identifier = Uuid;
}
```

## Methods

Resources define *data*; but to create an endpoint you also need some
*behavior* - the code that will be executed when the endpoint is accessed. This
is where methods come in.

A method is a trait, defining a behavior that could be implemented by any
resource. To define an endpoint, you will need to implement a method for a
resource.

For example, the `Index` method controls the behavior of `GET /$resource-type`.
You can define what happens when that request is hit for the user type by
implementing Index for User:

```rust
impl Index for User {
    fn index(env: Environment) -> Box<Future<Item = Vec<User>, Error = Error>> {
        // Note: panicking is not actually a good choice; it will crash your
        // server.
        panic!("Not implemented yet")
    }
}
```

## Formats

The last piece of this picture is the format. While the resource has defined
the data and the method has defined the behavior, you have not yet specified
how that data will be presented in terms of HTTP.

Formats specify presentation in a manner which is abstract over different
resources and methods, so that you can reuse that logic again and again. You
don't have to write code to bind your resource or method to a particular format
 - instead, you will declare the format to use for each method in the `routes!`
 DSL.

## Relationship to MVC

This tripartite division is essentially an application of the model, view,
controller pattern - it divides the work into data, logic, and presentation.
However, it is different from many MVC frameworks (like Ruby on Rails) because
it has more narrowly scoped the question.

In Rails, the model is responsible for persistence - in cargonauts, resources
do not necessarily correspond to any particular persistence form (that is, they
are not backed by a database table necessarily).

Similarly, in Rails the controller often encompasses both domain logic and HTTP
presentation logic. Here, the method and format clearly separate those two
responsibilities.

## Beyond MVC

Resources, methods, and formats aren't the only building block that cargonauts
gives you. It also comes with an API for wrapping your endpoints in middleware,
and provides ways to handle interacting with other networked services in a
way which is designed to be highly decoupled.
