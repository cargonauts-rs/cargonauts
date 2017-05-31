# Creating our first endpoint

[TODO intro]

### Committing

Its a good idea to commit your repository now that you know you have a
successfully initiated cargonauts app. That way you can easily restore your
state if anything bad happens, and you can get a quick diff to view the changes
you make at each step of this documentation. Just run a commit like so:

```
git add --all
git commit -m "An empty cargonauts app, ripe with potential."
```

## Creating our resource

Right now our application doesn't actually do anything - any request will
simply return a `404`. Let's start by creating a resource. Because this is a
very common thing to do, the cargonauts command has a subcommand which will
create a new resource, aptly titled `generate resource`. Let's create a
resource called Ping:

```
cargonauts generate resource ping
```

### Changes to `routing.rs`

We've added our first declaration to the `routes!` DSL:

```rust
routes! {
    resource Ping {
    }
}
```

The `routes!` DSL consists mostly of "resource objects" like this - there will
be one for each resource in your application, and it will contain various
additional items, mainly `method` statements, which we'll see in a bit.

In other words, your routes consist of a number of resource objects like this
one.

### `src/resources/ping.rs`

### Finishing the Resource impl

[TODO]

### Adding data to Ping

[TODO]

## Adding a method to Ping

Even though your app has a resource now, it *still* doesn't do anything! This
is because we haven't implemented any methods for that resource. A resource
without methods is like an object without methods, it doesn't actually do
anything.

We're going to make the `Ping` type implement the `Get` method. [describe the
Get method]

We don't have a generator for this, so we'll do it all by hand.

### Adding the impl

### Adding the method to the `routes!` macro

#### Deciding on a format
