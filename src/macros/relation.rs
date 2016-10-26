#[macro_export]
/// DSL for constructing a Relation type.
macro_rules! relation {
    ($rel:ident: $resource:ident as $to_one:expr, $to_many:expr) => {
        struct $rel;

        impl $crate::api::rel::Relation for $rel {
            type Resource = $resource;
            fn to_one() -> &'static str { $to_one }
            fn to_many() -> &'static str { $to_many }
        }
    };
}

#[cfg(test)]
mod tests {
    use api::Resource;
    use api::rel::Relation;
    use Serialize;
    use Serializer;

    #[derive(Copy, Clone)]
    struct User;

    impl Serialize for User {
        fn serialize<S: Serializer>(&self, _: &mut S) -> Result<(), S::Error> { unimplemented!() }
    }

    impl Resource for User {
        type Id = u32;
        type Repr = User;
        fn id(&self) -> u32 { unimplemented!() }
        fn repr(self) -> User { self }
        fn resource() -> &'static str { "user" }
        fn resource_plural() -> &'static str { "users" }
    }

    relation!(Author: User as "author", "authors");

    fn assert_rel_to_resource<T: Relation<Resource = U>, U: Resource>() { }

    #[test]
    fn author_is_rel_to_user() {
        assert_rel_to_resource::<Author, User>();
        assert_eq!(Author::to_one(), "author");
        assert_eq!(Author::to_many(), "authors");
    }
}

