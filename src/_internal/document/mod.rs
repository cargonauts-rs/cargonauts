mod collection;
mod null;
mod rel;
mod resource;

pub use self::collection::CollectionDocument;
pub use self::null::NullDocument;
pub use self::rel::RelDocument;
pub use self::resource::ResourceDocument;

use Serialize;
use Serializer;

struct JsonApi;

impl Serialize for JsonApi {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(1)));
        try!(serializer.serialize_map_key(&mut state, "version"));
        try!(serializer.serialize_map_value(&mut state, "1.0"));
        serializer.serialize_map_end(state)
    }
}
