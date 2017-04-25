use environment::Environment;
use Resource;

pub trait Request<T: Resource>: Sized {
    type BodyParts: 'static;
}

pub trait ResourceRequest<T: Resource>: Request<T> {
    fn new(Self::BodyParts, T::Identifier, &mut Environment) -> Self;
}

pub trait CollectionRequest<T: Resource>: Request<T> {
    fn new(Self::BodyParts, &mut Environment) -> Self;
}
