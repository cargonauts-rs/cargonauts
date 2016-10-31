use SerializeTo;
use Serializer;
use json;

pub trait Represent {
    fn repr<P: Presenter>(&self, presenter: &mut P) -> Result<(), P::Error>;
}

pub trait RepresentWith<P: Presenter> {
    fn repr_with(&self, presenter: &mut P) -> Result<(), P::Error>;
}

pub trait Presenter: Serializer {
    fn field_sets(&self) -> Option<&[String]>;
}

impl<P: Presenter, T: Represent> RepresentWith<P> for T {
    fn repr_with(&self, presenter: &mut P) -> Result<(), P::Error> {
        self.repr(presenter)
    }
}

pub struct SerializeRepr<'a, T: ?Sized + 'a> {
    pub repr: &'a T,
}

impl<'a, P, T: ?Sized> SerializeTo<P> for SerializeRepr<'a, T>
where T: RepresentWith<P>,
      P: Presenter,
{
    default fn serialize_to(&self, serializer: &mut P) -> Result<(), P::Error> {
        self.repr.repr_with(serializer)
    }
}

impl Presenter for json::value::Serializer {
    fn field_sets(&self) -> Option<&[String]> {
        None
    }
}
