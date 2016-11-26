use api::{Resource, Error};
use api::raw::{RawResource, RawHasPatch, RawReceived, Identifier};
use api::rel::{ToOne, ToMany};
use router::Request;

mod jsonapi;

pub use self::jsonapi::JsonApi;

pub trait Receiver<T: RawResource, R: Request> {
    fn receive_resource(request: R) -> Result<RawReceived<T, T>, Error>;
    fn receive_collection(request: R) -> Result<Vec<RawReceived<T, T>>, Error>;
    fn receive_to_one<Rel: ToOne>(request: R) -> Result<Option<Identifier>, Error>;
    fn receive_to_many<Rel: ToMany>(request: R) -> Result<Vec<Identifier>, Error>;
}

pub trait PatchReceiver<T: RawHasPatch<X>, R: Request, X> {
    fn receive_patch(request: R) -> Result<RawReceived<T, T::Patch>, Error>;
}

pub trait IdReceiver<T: Resource, R: Request> {
    fn receive_ids(request: R) -> Result<Vec<T::Id>, Error>;
}
