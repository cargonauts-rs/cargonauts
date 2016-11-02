use std::error::Error as ErrorTrait;

use api::Error;
use Serializer;
use Serialize;

pub struct ErrorObject<'a> {
    pub status: &'a Error,
}

impl<'a> Serialize for ErrorObject<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(1))?;
        serializer.serialize_map_key(&mut state, "status")?;
        serializer.serialize_map_value(&mut state, self.status.description())?;
        serializer.serialize_map_end(state)
    }
}
