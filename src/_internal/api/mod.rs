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
pub use self::delete::{_attach_delete, _MaybeDelete as _MaybeAttachDelete};
pub use self::get::{_attach_get, _MaybeGet as _MaybeAttachGet};
pub use self::index::{_attach_index, _MaybeIndex as _MaybeAttachIndex};
pub use self::post::{_attach_post, _attach_post_async, _MaybePost as _MaybeAttachPost, _MaybePostAsync as _MaybeAttachPostAsync};
pub use self::patch::{_attach_patch, _attach_patch_async, _MaybePatch as _MaybeAttachPatch, _MaybePatchAsync as _MaybeAttachPatchAsync};
pub use self::remove::{_attach_remove, _MaybeRemove as _MaybeAttachRemove};
pub use self::replace::{_attach_replace, _MaybeReplace as _MaybeAttachReplace};

mod has_one;
mod has_many;
mod post_links;
mod remove_links;
mod replace_links;
mod update_link;

pub use self::has_one::{_attach_has_one, _MaybeHasOne as _MaybeAttachHasOne};
pub use self::has_many::{_attach_has_many, _MaybeHasMany as _MaybeAttachHasMany};
pub use self::post_links::{_attach_post_links, _MaybePostLinks as _MaybeAttachPostLinks};
pub use self::remove_links::{_attach_remove_links, _MaybeRemoveLinks as _MaybeAttachRemoveLinks};
pub use self::replace_links::{_attach_replace_links, _MaybeReplaceLinks as _MaybeAttachReplaceLinks};
pub use self::update_link::{_attach_update_link, _MaybeUpdateLink as _MaybeAttachUpdateLink};

mod delete_one;
mod get_one;
mod index_many;
mod patch_one;
mod post_many;
mod remove_many;
mod replace_many;

pub use self::delete_one::{_attach_delete_one, _MaybeDeleteOne as _MaybeAttachDeleteOne};
pub use self::get_one::{_attach_get_one, _MaybeGetOne as _MaybeAttachGetOne};
pub use self::index_many::{_attach_index_many, _MaybeIndexMany as _MaybeAttachIndexMany};
pub use self::patch_one::{_attach_patch_one, _MaybePatchOne as _MaybeAttachPatchOne};
pub use self::post_many::{_attach_post_many, _MaybePostMany as _MaybeAttachPostMany};
pub use self::remove_many::{_attach_remove_many, _MaybeRemoveMany as _MaybeAttachRemoveMany};
pub use self::replace_many::{_attach_replace_many, _MaybeReplaceMany as _MaybeAttachReplaceMany};

use api::Resource;
use router::{Router, ResourceRoute};
use Future;

fn attach<R, T>(router: &mut R,
                route: ResourceRoute<'static>,
                handler: fn(R::Request, R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>)
where R: Router, T: Resource, {
    router.attach_resource(T::resource_plural(), route, handler)
}
