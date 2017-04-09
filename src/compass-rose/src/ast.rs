#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Resource {
    pub header: ResourceHeader,
    pub members: Vec<ResourceMember>,
    pub attrs: Vec<Attribute>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct ResourceHeader {
    pub ty: String,
    pub endpoint: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ResourceMember {
    Relation(Relation),
    Method(Method),
}

impl ResourceMember {
    pub fn as_relation(&self) -> Option<&Relation> {
        match *self {
            ResourceMember::Relation(ref rel)   => Some(rel),
            _                                   => None,
        }
    }

    pub fn as_method(&self) -> Option<&Method> {
        match *self {
            ResourceMember::Method(ref method)  => Some(method),
            _                                   => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Relation {
    pub rel: String,
    pub endpoint: Option<String>,
    pub members: Vec<RelationMember>,
    pub attrs: Vec<Attribute>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RelationMember {
    Method(Method<RelMethodKind>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Method<Kind = MethodKind> {
    pub method: Kind,
    pub format: String,
    pub attrs: Vec<Attribute>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MethodKind {
    Get,
    Index,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RelMethodKind {
    GetOne,
    GetMany,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Attribute {
    Ident(String),
    Arg(String, Vec<String>),
}
