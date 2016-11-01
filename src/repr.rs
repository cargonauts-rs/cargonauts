use SerializeTo;
use Serializer;

pub trait Represent {
    fn repr<S: Serializer>(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error>;
}

pub trait RepresentWith<S: Serializer> {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error>;
}

impl<S: Serializer, T: Represent> RepresentWith<S> for T {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        self.repr(serializer, field_set)
    }
}

pub struct SerializeRepr<'a, T: ?Sized + 'a> {
    pub repr: &'a T,
    pub field_set: Option<&'a [String]>,
}

impl<'a, S, T> SerializeTo<S> for SerializeRepr<'a, T>
where T: RepresentWith<S>,
      S: Serializer,
{
    fn serialize_to(&self, serializer: &mut S) -> Result<(), S::Error> {
        self.repr.repr_with(serializer, self.field_set.as_ref().map(|fields| &fields[..]))
    }
}

impl<'a, S: Serializer> SerializeTo<S> for SerializeRepr<'a, RepresentWith<S>> {
    fn serialize_to(&self, serializer: &mut S) -> Result<(), S::Error> {
        self.repr.repr_with(serializer, self.field_set)
    }
}
