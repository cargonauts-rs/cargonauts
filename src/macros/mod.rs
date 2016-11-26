#[macro_use]
mod relation;

#[macro_use]
mod resource;

/// The entry point for the routes DSL, which defines the endpoints of your API.
#[macro_export]
macro_rules! routes {
    {$(resource $resource:ident { $(has $count:ident $rel:ident;)* $(alias $method:ident as $route:expr;)*})*} => {
        pub fn attach_routes<T>(router: &mut T)
        where
            T: $crate::router::Router,
        {
            type S<T> = $crate::json::Serializer<T>;
            type P<T, R> = $crate::presenter::JsonApi<T, S<R>>;
            type D<T> = $crate::json::Deserializer<::std::io::Bytes<T>>;
            type C<T> = $crate::receiver::JsonApi<D<T>, T>;

            let mut router = $crate::_internal::_Router::new(router);
            $({ _resource!(router, $resource
                          { $($count $rel;)* }
                          { $($route => $method;)* }
            );})*
        }
    }
}

/// Do not call this macro, it is an implementation detail of the routes! macro.
#[macro_export]
macro_rules! _resource {
    ($router:expr, $resource:ty {} {$($route:expr => $method:ident;)*}) => {
        impl $crate::api::raw::RawResource for $resource {
            type FetchRels = $crate::api::raw::NoRelationships;
            type UpdateRels = $crate::api::raw::NoRelationships;
        }

        impl<I: 'static> $crate::_internal::_FetchRels<I> for $resource {
            fn rels(_: &$crate::api::Entity<Self>, _: &[$crate::router::IncludeQuery]) -> ::std::result::Result<(Self::FetchRels, Vec<$crate::api::raw::Include<I>>), $crate::api::Error> {
                Ok(($crate::api::raw::NoRelationships, vec![]))
            }
        }

        impl $crate::_internal::_UpdateRels for $resource {
            fn update_rels(_: &$crate::api::Entity<Self>, rels: $crate::api::raw::NoRelationships) -> ::std::result::Result<$crate::api::raw::NoRelationships, $crate::api::Error> {
                Ok($crate::api::raw::NoRelationships)
            }
        }
        $(_alias!($router, $resource, $route, $method);)*
        _methods!($router, $resource);


    };
    ($router:expr, $resource:ty { $($count:ident $rel:ident;)* } {$($route:expr => $method:ident;)*}) => {
        impl $crate::api::raw::RawResource for $resource {
            type FetchRels = Relationships;
            type UpdateRels = UpdateRelationships;
        }

        impl<I: 'static> $crate::_internal::_FetchRels<I> for $resource where
            I: $crate::presenter::ConvertInclude<$resource>,
        $(
            I: $crate::presenter::ConvertInclude<<$rel as $crate::api::rel::Relation>::Resource>,
        )* {
            fn rels(id: &$crate::api::Entity<Self>, includes: &[$crate::router::IncludeQuery]) -> ::std::result::Result<(Self::FetchRels, Vec<$crate::api::raw::Include<I>>), $crate::api::Error> {
                let mut include_objects = vec![];
                let rels = Relationships {
                    $(
                        $rel: {
                            _fetch_rel!(id, includes, include_objects, $resource, $rel, $count)
                        },
                    )*
                };
                Ok((rels, include_objects))
            }
        }

        impl $crate::_internal::_UpdateRels for $resource {
            fn update_rels(id: &$crate::api::Entity<Self>, rels: UpdateRelationships) -> ::std::result::Result<Relationships, $crate::api::Error> {
                $(
                    if let Some(rel) = rels.$rel {
                        _link_rel!(id, rel, $resource, $rel, $count);
                    };
                )*
                Ok(Relationships::default())
            }
        }

        #[allow(non_snake_case)]
        #[derive(Default)]
        struct Relationships {
            $(
                $rel: $crate::api::raw::RelationshipLinkage,
            )*
        }

        impl IntoIterator for Relationships {
            type Item = (&'static str, $crate::api::raw::RelationshipLinkage);
            type IntoIter = RelationshipsIntoIter;
            fn into_iter(self) -> RelationshipsIntoIter {
                RelationshipsIntoIter {
                    $(
                        $rel: Some(self.$rel),
                    )*
                    state: RelationshipsEnum::Init,
                }
            }
        }

        impl<'a> $crate::api::raw::FetchRelationships<'a> for Relationships {
            type Iter = RelationshipsIter<'a>;

            fn iter(&'a self) -> RelationshipsIter<'a> {
                RelationshipsIter {
                    rels: self,
                    state: RelationshipsEnum::Init,
                }
            }

            fn count(&self) -> usize {
                _count_rels!($($rel),*)
            }
        }

        #[allow(non_snake_case)]
        struct RelationshipsIntoIter {
            $(
                $rel: Option<$crate::api::raw::RelationshipLinkage>,
            )*
            state: RelationshipsEnum,
        }

        impl Iterator for RelationshipsIntoIter {
            type Item = (&'static str, $crate::api::raw::RelationshipLinkage);
            fn next(&mut self) -> Option<Self::Item> {
                match self.state {
                    RelationshipsEnum::Init => $( {
                        self.state = RelationshipsEnum::$rel;
                        self.$rel.take().map(|rel| (_name_rel!($rel, $count), rel))
                    }
                    RelationshipsEnum::$rel => )* { None }
                }
            }
        }

        struct RelationshipsIter<'a> {
            rels: &'a Relationships,
            state: RelationshipsEnum,
        }

        impl<'a> Iterator for RelationshipsIter<'a> {
            type Item = (&'static str, &'a $crate::api::raw::RelationshipLinkage);
            fn next(&mut self) -> Option<Self::Item> {
                match self.state {
                    RelationshipsEnum::Init => $( {
                        self.state = RelationshipsEnum::$rel;
                        Some((_name_rel!($rel, $count), &self.rels.$rel))
                    }
                    RelationshipsEnum::$rel => )* { None }
                }
            }
        }

        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        enum RelationshipsEnum {
            Init,
            $($rel,)*
        }

        #[derive(Clone, Eq, PartialEq, Debug, Default)]
        #[allow(non_snake_case)]
        struct UpdateRelationships {
            $(
                $rel: Option<$crate::api::raw::Relationship>,
            )*
        }

        impl $crate::api::raw::UpdateRelationships for UpdateRelationships {
            fn add_relationship(&mut self, name: String, rel: $crate::api::raw::Relationship) -> Result<(), $crate::api::raw::RelationshipError> {
                $( if &name[..] == _name_rel!($rel, $count) {
                    if self.$rel.is_none() {
                        self.$rel = Some(rel);
                        Ok(())
                    } else {
                        Err($crate::api::raw::RelationshipError::RelationshipAddedTwice)
                    }
                } else )* {
                    Err($crate::api::raw::RelationshipError::NoSuchRelationship)
                }
            }
        }

        _methods!($router, $resource);
        $(_alias!($router, $resource, $route, $method);)*
        $(_rel_methods!($router, $resource, $count $rel);)*
    };
}

/// Do not call this macro, it is an implementation detail of the routes! macro.
#[macro_export]
macro_rules! _methods {
    ($router:expr, $resource:ty) => {
        <$resource as $crate::_internal::_MaybeGet<P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeIndex<P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeDelete<P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeClear<P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeRemove<P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybePatch<P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybePost<P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeAppend<P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeReplace<P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
    };
}

#[macro_export]
macro_rules! _rel_methods {
    ($router:expr, $resource:ty, one $rel:ty) => {
        <$resource as $crate::_internal::_MaybeGetOne<$rel, P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeDeleteOne<$rel, P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybePatchOne<$rel, P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybePostOne<$rel, P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
    };
    ($router:expr, $resource:ty, many $rel:ty) => {
        <$resource as $crate::_internal::_MaybeIndexMany<$rel, P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeClearMany<$rel, P<T, T::Response>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeRemoveMany<$rel, P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeAppendMany<$rel, P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
        <$resource as $crate::_internal::_MaybeReplaceMany<$rel, P<T, T::Response>, C<T::Request>, T>>::attach(&mut $router);
    };
}

#[macro_export]
macro_rules! _alias {
    ($router:expr, $resource:ty, $route:expr, get) => {
        $router.attach_get_alias::<$resource, $crate::presenter::JsonApi<T, S<T::Response>>>($route);
    };
    ($router:expr, $resource:ty, $route:expr, index) => {
        $router.attach_index_alias::<$resource, $crate::presenter::JsonApi<T, S<T::Response>>>($route);
    };
}

#[macro_export]
macro_rules! _fetch_rel {
    ($id:expr, $includes:expr, $includes_out:expr, $resource:ty, $rel:ty, one) => {
        if let Some(include) = $includes.iter().find(|include| include.is_of(_name_rel!($rel, one))) {
            use $crate::{IntoFuture, Future};
            let response = <$resource as $crate::api::rel::raw::GetOne<I, $rel>>::get_one($id, include.transitive.clone()).into_future().wait()?;
            let identifier = $crate::api::raw::Identifier::from(&response.resource);
            $includes_out.push(response.resource.into());
            $includes_out.extend(response.includes);
            $crate::api::raw::RelationshipLinkage {
                linkage: Some($crate::api::raw::Relationship::One(Some(identifier))),
            }
        } else {
            $crate::api::raw::RelationshipLinkage {
                linkage: None,
            }
        };
    };
    ($id:expr, $includes:expr, $includes_out:expr, $resource:ty, $rel:ty, many) => {
        if let Some(include) = $includes.iter().find(|include| include.is_of(_name_rel!($rel, many))) {
            use $crate::{IntoFuture, Future};
            let response = <$resource as $crate::api::rel::raw::IndexMany<I, $rel>>::index_many($id, include.transitive.clone()).into_future().wait()?;
            let identifiers = response.resources.iter().map($crate::api::raw::Identifier::from).collect();
            $includes_out.extend(response.resources.into_iter().map(Into::into));
            $includes_out.extend(response.includes);
            $crate::api::raw::RelationshipLinkage {
                linkage: Some($crate::api::raw::Relationship::Many(identifiers)),
            }
        } else {
            $crate::api::raw::RelationshipLinkage {
                linkage: None,
            }
        }
    };
}

#[macro_export]
macro_rules! _link_rel {
    ($id:expr, $rel_obj:expr, $resource:ty, $rel:ty, one)   => {
        match $rel_obj {
            $crate::api::raw::Relationship::One(Some(ref identifier)) => {
                use $crate::{IntoFuture, Future};
                let rel_id = identifier.id.parse().or(Err($crate::api::Error::BadRequest))?;
                <$resource as $crate::_internal::_MaybeLinkOne<$rel>>::link_one($id, &rel_id).into_future().wait()?;

            }
            $crate::api::raw::Relationship::One(None)           => {
                use $crate::{IntoFuture, Future};
                <$resource as $crate::_internal::_MaybeUnlinkOne<$rel>>::unlink_one($id).into_future().wait()?;
            }
            $crate::api::raw::Relationship::Many(_)             => {
                return Err($crate::api::Error::BadRequest);
            }
        }
    };
    ($id:expr, $rel_obj:expr, $resource:ty, $rel:ty, many)   => {
        match $rel_obj {
            $crate::api::raw::Relationship::One(_)                  => {
                return Err($crate::api::Error::BadRequest);
            }
            $crate::api::raw::Relationship::Many(ref identifiers)   => {
                use $crate::{IntoFuture, Future};
                let rel_ids = identifiers.iter().map(|identifier| identifier.id.parse().or(Err($crate::api::Error::BadRequest))).collect::<::std::result::Result<::std::vec::Vec<_>, _>>()?;
                <$resource as $crate::_internal::_MaybeReplaceLinks<$rel>>::replace_links($id, &rel_ids).into_future().wait()?;
            }
        }
    };
}

#[macro_export]
macro_rules! _name_rel {
    ($rel:ty, one) => {
        <$rel as $crate::api::rel::ToOne>::to_one()
    };
    ($rel:ty, many) => {
        <$rel as $crate::api::rel::ToMany>::to_many()
    };
}

#[macro_export]
macro_rules! _count_rels {
    ($head:ident, $($rest:ident),*) => (1 + _count_rels!($($rest),*));
    ($last:ident)                   => (1);
}
