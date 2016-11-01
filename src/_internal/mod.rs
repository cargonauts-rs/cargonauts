//! This module is an implementation detail of cargonauts; it is only exposed in the API because
//! it needs to be called by the expansion of the `routes!` macro.
//!
//! **Never use any component of this module directly.** cargonauts provides absolutely no
//! guarantee about its backward compatibility, and it is not designed to support any use case
//! other than the expansion of the `routes!` macro. Using one of these types directly is
//! **always** a mistake.
mod rels;
mod router;

pub use self::rels::{_FetchRels, _UpdateRels, _MaybeLinkOne, _MaybeUnlinkOne, _MaybeReplaceLinks};
pub use self::router::Router as _Router;
