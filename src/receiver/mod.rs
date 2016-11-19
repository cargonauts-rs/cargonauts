use std::io::Read;
use io_adapter::ReadAdapter;

use api::Error;
use api::raw::{RawUpdate, RawHasPatch, RawReceived, Identifier};
use api::rel::{ToOne, ToMany};

mod jsonapi;

pub use self::jsonapi::JsonApi;

pub trait Receiver<T: RawUpdate, R: Read>: ReadAdapter<R> {
    fn receive_resource(self) -> Result<RawReceived<T, T>, Error>;
    fn receive_collection(self) -> Result<Vec<RawReceived<T, T>>, Error>;
    fn receive_to_one<Rel: ToOne>(self) -> Result<Option<Identifier>, Error>;
    fn receive_to_many<Rel: ToMany>(self) -> Result<Vec<Identifier>, Error>;
}

pub trait PatchReceiver<T: RawHasPatch<X>, R: Read, X>: ReadAdapter<R> {
    fn receive_patch(self) -> Result<RawReceived<T, T::Patch>, Error>;
}
