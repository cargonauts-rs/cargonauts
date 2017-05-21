// The methods module is for defining custom methods you want your resources to
// support.
//
// A method must implement the `Method` trait and either `ResourceMethod` or
// `CollectionMethod` (but not both!). Methods are themselves traits, and can
// be a bit tricky to implement correctly. Read the docs for more info.
//
// Usually, you will not need to define your own methods; cargonauts comes with
// several methods built-in that should be satisfactory for most use cases.
mod random;

pub use self::random::Random;
