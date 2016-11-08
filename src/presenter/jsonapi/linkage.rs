use api::raw::Identifier;
use repr::Represent;
use Serialize;
use Serializer;

pub struct ToOneLinkage<'a>(pub &'a Option<Identifier>);

impl<'a> Serialize for ToOneLinkage<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        match self.0.as_ref() {
            Some(identifier)    => IdentifierObject(identifier).serialize(serializer),
            None                => serializer.serialize_unit(),
        }
    }
}

impl<'a> Represent for ToOneLinkage<'a> {
    fn repr<S: Serializer>(&self, serializer: &mut S, _: Option<&[String]>) -> Result<(), S::Error> {
        match self.0.as_ref() {
            Some(identifier)    => IdentifierObject(identifier).serialize(serializer),
            None                => serializer.serialize_unit(),
        }
    }
}

pub struct ToManyLinkage<'a>(pub &'a [Identifier]);

impl<'a> Represent for ToManyLinkage<'a> {
    fn repr<S: Serializer>(&self, serializer: &mut S, _: Option<&[String]>) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.0.len()))?;
        for identifier in self.0 {
            serializer.serialize_seq_elt(&mut state, IdentifierObject(identifier))?;
        }
        serializer.serialize_seq_end(state)
    }
}
impl<'a> Serialize for ToManyLinkage<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.0.len()))?;
        for identifier in self.0 {
            serializer.serialize_seq_elt(&mut state, IdentifierObject(identifier))?;
        }
        serializer.serialize_seq_end(state)
    }
}

struct IdentifierObject<'a>(&'a Identifier);

impl<'a> Serialize for IdentifierObject<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(2))?;
        serializer.serialize_map_key(&mut state, "id")?;
        serializer.serialize_map_value(&mut state, &self.0.id)?;
        serializer.serialize_map_key(&mut state, "type")?;
        serializer.serialize_map_value(&mut state, self.0.resource)?;
        serializer.serialize_map_end(state)
    }
}

#[cfg(test)]
mod tests {
    use api::raw::Identifier;
    use std::collections::BTreeMap;
    use json::to_value;

    #[test]
    fn serialize_identifier() {
        let identifier = Identifier {
            resource: "identified",
            id: String::from("101"),
        };
        let expected = {
            let mut identifier = BTreeMap::new();
            identifier.insert(String::from("type"), to_value("identified"));
            identifier.insert(String::from("id"), to_value("101"));
            to_value(&identifier)
        };
        assert_eq!(to_value(&super::IdentifierObject(&identifier)), expected);
    }
}
