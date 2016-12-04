macro_rules! try_status {
    ($x:expr, $p:expr)  => {
        match $x {
            Ok(x)   => x,
            Err(_)  => {
                return Box::new(Ok($p.present_err(Error::Conflict)).into_future()) as Box<Future<Item = R::Response, Error = ()>>
            }
        }
    };
}

mod alias;
mod delete;
mod get;
mod index;
mod patch;
mod post;
mod remove;
mod replace;

pub use self::alias::{_attach_get_aliased, _attach_index_aliased};
pub use self::delete::{_attach_delete, _MaybeAttachDelete};
pub use self::get::{_attach_get, _MaybeAttachGet};
pub use self::index::{_attach_index, _MaybeAttachIndex};
pub use self::post::{_attach_post, _attach_post_async, _MaybeAttachPost, _MaybeAttachPostAsync};
pub use self::patch::{_attach_patch, _attach_patch_async, _MaybeAttachPatch, _MaybeAttachPatchAsync};
pub use self::remove::{_attach_remove, _MaybeAttachRemove};
pub use self::replace::{_attach_replace, _MaybeAttachReplace};

mod has_one;
mod has_many;
mod post_links;
mod remove_links;
mod replace_links;
mod update_link;

pub use self::has_one::{_attach_has_one, _MaybeAttachHasOne};
pub use self::has_many::{_attach_has_many, _MaybeAttachHasMany};
pub use self::post_links::{_attach_post_links, _MaybeAttachPostLinks};
pub use self::remove_links::{_attach_remove_links, _MaybeAttachRemoveLinks};
pub use self::replace_links::{_attach_replace_links, _MaybeAttachReplaceLinks};
pub use self::update_link::{_attach_update_link, _MaybeAttachUpdateLink};

mod delete_one;
mod get_one;
mod index_many;
mod patch_one;
mod post_many;
mod remove_many;
mod replace_many;

pub use self::delete_one::{_attach_delete_one, _MaybeAttachDeleteOne};
pub use self::get_one::{_attach_get_one, _MaybeAttachGetOne};
pub use self::index_many::{_attach_index_many, _MaybeAttachIndexMany};
pub use self::patch_one::{_attach_patch_one, _MaybeAttachPatchOne};
pub use self::post_many::{_attach_post_many, _MaybeAttachPostMany};
pub use self::remove_many::{_attach_remove_many, _MaybeAttachRemoveMany};
pub use self::replace_many::{_attach_replace_many, _MaybeAttachReplaceMany};

use api::Resource;
use router::{Router, Method, Component, ResourceRoute};
use Future;

fn attach<R, T>(router: &mut R,
                method: Method,
                component: Component<'static>,
                handler: fn(R::Request, R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>)
where R: Router, T: Resource, {
    router.attach_resource(ResourceRoute {
        name: T::resource_plural(),
        component: component,
        method: method,
    }, handler)
}
