# Resources, methods and formats

The core concepts you need to understand to use cargonauts are **resources**,
**methods**, and **formats**. These are the building blocks for the "front" of
your application; by combining a resource, a method, and a route, you can
construct an HTTP endpoint.

## Resources

cargonauts is REST-oriented web framework, so it is designed to enable you to
construct your application in terms of resources. Your application will have
many resources.

A resource is just a type that implements the `Resource` trait. For example:

```rust
pub struct User {
    username: String,
    email: String,
    joined_at: DateTime<UTC>,
}

impl Resource for User {
    type Identifier = Uuid;
}
```

The Resource trait is very small: it just specifies the type of Identifier
used to uniquely identify each value of this resource.

You can think of the resource as defining the data that will be included in an
HTTP response to certain endpoints. Once you've defined a resource, you will
need to implement methods on it to give it behavior.

cargonauts doesnot provide any resources for you. You define all of the
resources that are appropriate to your domain.

## Methods

Resources define data, whereas methods define behavior. All methods are traits,
and the file for a resource will contain not only that resource, but also
impl blocks for the methods implemented for it.

cargonauts comes with a large number of methods defined for you. You can see
the list of methods provided in the API docs.

```rust
impl Get for User {
    fn get(id: Uuid, env: Environment) -> Box<Future<Item = User, Error = Error>> {
        // ...
    }
}
```

### Resource and Collection methods

Some methods are resource methods, whereas some are collection methods. That is
to say that some define HTTP endpoints on specific resources, whereas some
define them on the collection of all resources.

```
# This is a resource endpoint:
HTTP /<resource-name>/<identifier>

# This is a collection endpoint::
HTTP /<resource-name>
```

For example, the `Get` method is a resource method, which defines what happens
when you `GET` a resource. On the other hand, the `Post` method is a collection
method, which deifnes what happens when you `POST` a collection endpoint.

Resource methods usually have an identifier as an argument, whereas collection
methods don't. The identifier is parsed from the route path.

## Formats

Resources define data & methods define behavior, but formats define 
presentation. Once a method has been called on a resource, the format
translates that into an HTTP response.

Formats are types which implement the Format trait. A format likely will
require your resource to implement additional traits for it to work.

Cargonauts comes with three formats provided for you:

* **Debug:** This format is not intended for serious use; the Debug format
debug prints the data returned by a method into the response body.

* **Handlebars:** This format is intended for server side rendering; it renders
templates using handlebars, and can receive POSTs from HTML forms.

* **JsonApi:** This format is intended for creating APIs in JSON, and is an
implementation of the JSON API spec. It does not implement everything from that
spec yet.

You do not directly hook your resources up to formats in the resource module;
instead, that is entirely handled as a part of the `routes!` macro.
