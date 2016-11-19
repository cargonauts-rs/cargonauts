#[macro_export]
/// DSL for constructing a ToOne type.
macro_rules! to_one {
    ($rel:ident => $resource:ident as $to_one:expr) => {
        struct $rel;

        impl $crate::api::rel::Relation for $rel {
            type Resource = $resource;
        }

        impl $crate::api::rel::ToOne for $rel {
            fn to_one() -> &'static str { $to_one }
        }
    };
}

#[macro_export]
/// DSL for constructing a ToMany type.
macro_rules! to_many {
    ($rel:ident => $resource:ident as $to_many:expr) => {
        struct $rel;

        impl $crate::api::rel::Relation for $rel {
            type Resource = $resource;
        }

        impl $crate::api::rel::ToMany for $rel {
            fn to_many() -> &'static str { $to_many }
        }
    };
}

#[cfg(test)]
mod tests {
    use api::Resource;
    use api::rel::{Relation, ToOne, ToMany};
    use repr;

    #[derive(Copy, Clone)]
    struct User;

    impl repr::Represent for User {
        fn repr<S: ::Serializer>(&self, _: &mut S, _: Option<&[String]>) -> Result<(), S::Error> { unimplemented!() }
    }

    impl Resource for User {
        type Id = u32;
        fn id(&self) -> u32 { unimplemented!() }
        fn resource() -> &'static str { "user" }
        fn resource_plural() -> &'static str { "users" }
    }

    to_one!(Author => User as "author");

    to_many!(Authors => User as "authors");

    fn assert_rel_to_resource<T: Relation<Resource = U>, U: Resource>() { }

    #[test]
    fn author_is_rel_to_user() {
        assert_rel_to_resource::<Author, User>();
        assert_eq!(Author::to_one(), "author");
    }

    #[test]
    fn authors_is_rel_to_user() {
        assert_rel_to_resource::<Authors, User>();
        assert_eq!(Authors::to_many(), "authors");
    }
}

