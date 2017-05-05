mod delete;
mod get;
mod get_one;
mod get_many;
mod index;
mod patch;
mod post;

pub use self::delete::Delete;
pub use self::get::Get;
pub use self::index::Index;
pub use self::patch::Patch;
pub use self::post::Post;

pub use self::get_one::GetOne;
pub use self::get_many::GetMany;
