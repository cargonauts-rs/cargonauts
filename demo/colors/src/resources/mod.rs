// The resources module is for defining your application's resources. Every
// app will have many resources.
//
// Each resource must implement the `Resource` trait. For every method you've
// associated with the resource in your routing file, you must also implement
// that method for your resource.
mod square;

pub use self::square::Square;
