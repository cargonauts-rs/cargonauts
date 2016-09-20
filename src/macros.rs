/// The entry point for the routes DSL, which defines the endpoints of your API.
#[macro_export]
macro_rules! routes {
    {$(resource $resource:ty => $methods:tt { $(related $rel:ty : $count:tt; )* } )*} => {
        pub fn attach_routes<T: ::cargonauts::router::Router>(router: &mut T) {
            let mut router = ::cargonauts::_internal::Router::new(router);
            $({ _resource!(router, $resource => $methods { $($rel: $count;)* }); })*
        }
    }
}

/// Do not call this macro, it is an implementation detail of the routes! macro.
#[macro_export]
macro_rules! _resource {
    ($router:expr, $resource:ty => $methods:tt { }) => {
        impl ::cargonauts::_internal::Wrapper<$resource> for ::cargonauts::_internal::Resource<$resource> {
            type Relationships = ();

            fn related(&self, _base_url: &str) -> Option<()> {
                None
            }

            fn include(&self, _params: &[String], _base_url: &str) -> Vec<::cargonauts::Value> {
                vec![]
            }
        }

        _methods!($router, $resource, $methods);
    };
    ($router:expr, $resource:ty => $methods:tt { $($rel:ty : $count:tt;)+ }) => {
        impl ::cargonauts::_internal::Wrapper<$resource> for ::cargonauts::_internal::Resource<$resource> {
            type Relationships = Relationships;

            fn related(&self, base_url: &str) -> Option<Relationships> {
                Some(Relationships {
                    id: self.id(),
                    base_url: base_url.to_string(),
                })
            }

            fn include(&self, params: &[String], base_url: &str) -> Vec<::cargonauts::Value> {
                let id = self.id();
                params.iter().flat_map(|param| {
                    $(
                        if param == <$rel as ::cargonauts::api::Resource>::resource() {
                            _include_relation!(&id, base_url, $resource, $rel, $count);
                        }
                    )*
                    return vec![]
                }).collect()
            }
        }

        struct Relationships {
            id: <$resource as ::cargonauts::api::Resource>::Id,
            base_url: String,
        }

        impl ::cargonauts::Serialize for Relationships {
            fn serialize<S: ::cargonauts::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
                let mut state = try!(serializer.serialize_map(None));
                let id = &self.id;
                let base_url = &self.base_url;
                $(
                    { _serialize_relation!(serializer, &mut state, id, base_url, $resource, $rel, $count); }
                )*
                serializer.serialize_map_end(state)
            }
        }

        _methods!($router, $resource, $methods);
    };
}

/// Do not call this macro, it is an implementation detail of the routes! macro.
#[macro_export]
macro_rules! _include_relation {
    ($id:expr, $base_url:expr, $resource:ty, $rel:ty, "has-one") => {
        if let Some(resource) = <$resource as ::cargonauts::api::HasOne<$rel>>::has_one($id) {
            let resource = ::cargonauts::_internal::Resource::wrap(resource, $base_url);
            return vec![::cargonauts::to_value(resource)]
        } else { return vec![] }
    };
    ($id:expr, $base_url:expr, $resource:ty, $rel:ty, "has-many") => {
        let mut values = vec![];
        for resource in <$resource as ::cargonauts::api::HasMany<$rel>>::has_many($id) {
            let resource = ::cargonauts::_internal::Resource::wrap(resource, $base_url);
            values.push(::cargonauts::to_value(resource))
        }
        return values
    };
    ($id:expr, $resource:ty, $rel:ty, $ignore:tt) => {
        // TODO handle errors more betterer
    };
}

/// Do not call this macro, it is an implementation detail of the routes! macro.
#[macro_export]
macro_rules! _serialize_relation {
    ($serializer:expr, $state:expr, $id:expr, $base_url:expr, $resource:ty, $rel:ty, "has-one") => {
        try!($serializer.serialize_map_key($state, <$rel as ::cargonauts::api::Resource>::resource()));
        try!($serializer.serialize_map_value($state, ::cargonauts::_internal::HasOne::<$resource, $rel>::new($id, $base_url)));
    };
    ($serializer:expr, $state:expr, $id:expr, $base_url:expr, $resource:ty, $rel:ty, "has-many") => {
        try!($serializer.serialize_map_key($state, <$rel as ::cargonauts::api::Resource>::resource()));
        try!($serializer.serialize_map_value($state, ::cargonauts::_internal::HasMany::<$resource, $rel>::new($id, $base_url)));
        
    };
    ($serializer:expr, $state:expr, $id:expr, $base_url:expr, $resource:ty, $rel:ty, $ignore:tt) => {
        // TODO handle errors more betterer
    };
}

/// Do not call this macro, it is an implementation detail of the routes! macro.
#[macro_export]
macro_rules! _methods {
    ($router:expr, $resource:ty,  ["get", $($method:tt),*]) => {
        $router.attach_get::<$resource>();
        _methods!($router, $resource,  [$($method),*])
    };
    ($router:expr, $resource:ty,  ["get"]) => {
        $router.attach_get::<$resource>();
    };
    ($router:expr, $resource:ty,  ["index", $($method:tt),*]) => {
        $router.attach_index::<$resource>();
        _methods!($router, $rels, $resource,  [$($method),*])
    };
    ($router:expr, $resource:ty,  ["index"]) => {
        $router.attach_index::<$resource>();
    };
    ($router:expr, $resource:ty,  [$ignore:tt, $($method:tt),*]) => {
        // TODO handle errors more betterer
        _methods!($router, $rels, $resource,  [$($method),*])
    };
    ($router:expr, $resource:ty,  $ignore:tt) => {
        // TODO handle errors more betterer
    };
}
