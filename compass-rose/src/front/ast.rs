use lalrpop_util::ParseError;
use front::parser;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Method {
    Get,
    Patch,
    Post,
    Delete,
    Index,
    Append,
    Replace,
    Remove,
    Clear,
}

#[derive(Debug)]
pub struct Alias {
    pub method: Method,
    pub route: String,
}

#[derive(Debug)]
pub struct Relation {
    pub variance: RelVariance,
    pub relation: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RelVariance {
    One,
    Many,
}

#[derive(Debug)]
pub enum ResourceAttr {
    Alias(Alias),
    Relation(Relation),
}

#[derive(Debug)]
pub struct Resource {
    pub resource: String,
    pub attrs: Vec<ResourceAttr>,
}

#[derive(Debug)]
pub struct Routes {
    pub resources: Vec<Resource>,
}

impl Routes {
    pub fn parse(dsl: &str) -> Result<Routes, ParseError<usize, (usize, &str), ()>> {
        parser::parse_routes(dsl)
    }
}
