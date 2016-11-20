#[macro_export]
/// Reduce the boilerplate in declaring a resource impl.
///
/// It supports two forms. In both forms, the "header" declares the
/// implementing type as well as its singular and plural API forms.
///
/// They are different in how they declare the ID. If the ID is just
/// a field, you can declare it like this:
/// ```
/// #[macro_use] extern crate cargonauts;
///
/// struct User {
///     id: u32,
///     username: String,
/// }
///
/// resource!(User as "user", "users" => {
///    type Id: u32 = self.id;
/// });
///
/// fn main() { }
/// ```
///
/// The other form allows you to define an `id` function, which returns
/// the id. This allows for a more complex Id lookup than field access:
///
/// ```
/// #[macro_use] extern crate cargonauts;
///
/// struct User {
///     id: u32,
///     username: String,
/// }
///
/// resource!(User as "user", "users" => {
///     fn id(&self) -> u32 {
///       self.id
///     }
/// });
///
/// fn main() { }
/// ```
macro_rules! resource {
    ($resource:ty as $singular:expr, $plural:expr => { fn id(&$this:ident) -> $id_ty:ty $id_fn:block }) => {
        impl $crate::api::Resource for $resource {
            type Id = $id_ty;
            fn id(&$this) -> Self::Id { $id_fn }
            fn resource() -> &'static str { $singular }
            fn resource_plural() -> &'static str { $plural }
        }
    };
    ($resource:ty as $singular:expr, $plural:expr => { type Id: $id_ty:ty = self.$id_field:ident; }) => {
        impl $crate::api::Resource for $resource {
            type Id = $id_ty;
            fn id(&self) -> Self::Id { self.$id_field }
            fn resource() -> &'static str { $singular }
            fn resource_plural() -> &'static str { $plural }
        }
    }
}

#[cfg(test)]
mod tests {

    use api::Resource;

    struct User {
        id: u32,
    }

    resource!(User as "user", "users" => {
        type Id: u32 = self.id;
    });

    struct Singleton;

    resource!(Singleton as "singleton", "singletons" => {
        fn id(&self) -> String {
            String::from("singleton")
        }
    });

    #[test]
    fn user_is_resource() {
        assert_eq!(User::resource(), "user");
        assert_eq!(User::resource_plural(), "users");
        assert_eq!(User { id: 0 }.id(), 0);
    }

    #[test]
    fn singleton_is_resource() {
        assert_eq!(Singleton::resource(), "singleton");
        assert_eq!(Singleton::resource_plural(), "singletons");
        assert_eq!(Singleton.id(), "singleton");
    }
}
