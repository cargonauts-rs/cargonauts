use std::result;
use std::str::FromStr;

use Serialize;

mod error;
mod get;
mod delete;
mod index;
mod patch;
mod post;

pub mod raw;
pub mod rel;

pub use self::error::Error;
pub use self::get::Get;
pub use self::delete::Delete;
pub use self::index::Index;
pub use self::patch::Patch;
pub use self::post::Post;

pub trait Resource: Serialize + Sized {
    type Id: ToString + FromStr + PartialEq + Clone;
    fn id(&self) -> Self::Id;
    fn resource() -> &'static str;
    fn resource_plural() -> &'static str;
}

pub type Result<T> = result::Result<T, Error>;
