use Serializer;
use Serialize;

pub struct JsonApiObject;

impl Serialize for JsonApiObject {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(1))?;
        serializer.serialize_map_key(&mut state, "version")?;
        serializer.serialize_map_value(&mut state, "1.0")?;
        serializer.serialize_map_end(state)
    }
}
