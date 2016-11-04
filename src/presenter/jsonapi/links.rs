use Serialize;
use Serializer;

pub struct LinkObject<'a> {
    pub self_link: Option<&'a str>,
    pub related_link: Option<&'a str>,
}

impl<'a> LinkObject<'a> {
    fn count(&self) -> usize {
        match (self.self_link.is_some(), self.related_link.is_some()) {
            (true, true)    => 2,
            (false, false)  => 0,
            (_, _)          => 1,
        }
    }
}

impl<'a> Serialize for LinkObject<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(self.count()))?;
        if let Some(self_link) = self.self_link.as_ref() {
            serializer.serialize_map_key(&mut state, "self")?;
            serializer.serialize_map_value(&mut state, self_link)?;
        }
        if let Some(related_link) = self.related_link.as_ref() {
            serializer.serialize_map_key(&mut state, "related")?;
            serializer.serialize_map_value(&mut state, related_link)?;
        }
        serializer.serialize_map_end(state)
    }
}
