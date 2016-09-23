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

            fn get_related(&self, _base_url: &str) -> Option<()> {
                None
            }

            fn put_related<'a, I>(id: &<$resource as ::cargonauts::api::Resource>::Id, rels: I) -> Result<(), ::cargonauts::api::LinkError>
                where I: IntoIterator<Item = &'a ::cargonauts::router::Relationship> {
                if rels.next().is_none() {
                    Ok(())
                } else {
                    Err(::cargonauts::api::LinkError::Conflict)
                }
            }

            fn include(&self, params: &[String], _base_url: &str) -> Vec<::cargonauts::Value> {
                vec![]
            }
        }

        _methods!($router, $resource, $methods { });
    };
    ($router:expr, $resource:ty => $methods:tt { $($rel:ty : $count:tt;)+ }) => {
        impl ::cargonauts::_internal::Wrapper<$resource> for ::cargonauts::_internal::Resource<$resource> {
            type Relationships = Relationships;

            fn get_related(&self, base_url: &str) -> Option<Relationships> {
                Some(Relationships {
                    id: self.id(),
                    base_url: base_url.to_string(),
                })
            }

            fn put_related<'a, I>(id: &<$resource as ::cargonauts::api::Resource>::Id, rels: I) -> Result<(), ::cargonauts::api::LinkError>
                where I: IntoIterator<Item = &'a ::cargonauts::router::Relationship> {
                for relationship in rels {
                    $(
                        if relationship.resource == <$rel as ::cargonauts::api::Resource>::resource() {
                            _link_relation!(id, &relationship.member, $resource, $rel, $count);
                            continue
                        }
                    )*
                    return Err(::cargonauts::api::LinkError::Conflict)
                }
                Ok(())
            }

            fn include(&self, params: &[String], base_url: &str) -> Vec<::cargonauts::Value> {
                let id = self.id();
                params.iter().flat_map(|param| {
                    $(
                        if param == <$rel as ::cargonauts::api::Resource>::resource() {
                            _include_relation!(&id, base_url, $resource, $rel, $count);
                        }
                    )*
                    vec![]
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

        _methods!($router, $resource, $methods {$($rel, $count);*});
    };
}

/// Do not call this macro, it is an implementation detail of the routes! macro.
#[macro_export]
macro_rules! _link_relation {
    ($id:expr, $rel_id:expr, $resource:ty, $rel:ty, "has-one") => {
        if let ::cargonauts::router::RelationshipId::One(ref id) = *$rel_id {
            let id = try!(id.parse().or(Err(::cargonauts::api::LinkError::Conflict)));
            try!(<$resource as ::cargonauts::api::HasOne<$rel>>::link_one($id, &id));
        } else { return Err(::cargonauts::api::LinkError::Conflict) }
    };
    ($id:expr, $rel_id:expr, $resource:ty, $rel:ty, "has-many") => {
        if let ::cargonauts::router::RelationshipId::Many(ref ids) = *$rel_id {
            let ids = try!(ids.into_iter().map(|id| id.parse().or(Err(::cargonauts::api::LinkError::Conflict))).collect::<Result<Vec<_>, _>>());
            try!(<$resource as ::cargonauts::api::HasMany<$rel>>::link_many($id, &ids));
        }
    }
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
    ($router:expr, $resource:ty, ["get", $($method:tt),*] {$($rel:ty, $count:expr);*}) => {
        $router.attach_get::<$resource>();
        $(_rel_methods!($router, $resource, $rel, "get", $count);)*
        _methods!($router, $resource, [$($method),*] {$($rel, $count);*});
    };
    ($router:expr, $resource:ty, ["get"] {$($rel:ty, $count:expr);*}) => {
        $router.attach_get::<$resource>();
        $(_rel_methods!($router, $resource, $rel, "get", $count);)*
    };
    ($router:expr, $resource:ty, ["index", $($method:tt),*] {$($rel:ty, $count:expr);*}) => {
        $router.attach_index::<$resource>();
        _methods!($router, $resource, [$($method),*] {$($rel, $count);*})
    };
    ($router:expr, $resource:ty, ["index"] {$($rel:ty, $count:expr);*}) => {
        $router.attach_index::<$resource>();
    };
    ($router:expr, $resource:ty,  ["patch", $($method:tt),*] {$($rel:ty, $count:expr);*}) => {
        $router.attach_patch::<$resource>();
        _methods!($router, $resource, [$($method),*])
    };
    ($router:expr, $resource:ty, ["patch"] {$($rel:ty, $count:expr);*}) => {
        $router.attach_patch::<$resource>();
    };
    ($router:expr, $resource:ty, ["post", $($method:tt),*] {$($rel:ty, $count:expr);*}) => {
        $router.attach_post::<$resource>();
        _methods!($router, $resource, [$($method),*])
    };
    ($router:expr, $resource:ty, ["post"] {$($rel:ty, $count:expr);*}) => {
        $router.attach_post::<$resource>();
    };
    ($router:expr, $resource:ty, [$ignore:tt, $($method:tt),*] {$($rel:ty, $count:expr);*}) => {
        // TODO handle errors more betterer
        _methods!($router, $resource, [$($method),*])
    };
    ($router:expr, $resource:ty, $ignore:tt {$($rel:ty, $count:expr);*}) => {
        // TODO handle errors more betterer
    };
}

#[macro_export]
macro_rules! _rel_methods {
    ($router:expr, $resource:ty, $rel:ty, "get", "has-one") => {
        $router.attach_get_has_one::<$resource, $rel>();
    };
    ($router:expr, $resource:ty, $rel:ty, "get", "has-many") => {
        $router.attach_get_has_many::<$resource, $rel>();
    };
    ($router:expr, $resource:ty, $rel:ty, $method:expr, $count:expr) => {
        // TODO handle errors more betterer
    };
}
