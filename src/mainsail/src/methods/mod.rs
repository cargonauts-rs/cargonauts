mod delete;
mod delete_related;
mod get;
mod get_one;
mod get_many;
mod index;
mod patch;
mod post;
mod post_related;
mod update_related;

pub use self::delete::Delete;
pub use self::get::Get;
pub use self::index::Index;
pub use self::patch::Patch;
pub use self::post::Post;

pub use self::delete_related::DeleteRelated;
pub use self::get_one::GetOne;
pub use self::get_many::GetMany;
pub use self::post_related::PostRelated;
pub use self::update_related::UpdateRelated;
