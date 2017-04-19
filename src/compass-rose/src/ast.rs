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
    Method(Method),
}

impl RelationMember {
    pub fn as_method(&self) -> Option<&Method> {
        match *self {
            RelationMember::Method(ref method)  => Some(method),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Method {
    pub method: String,
    pub format: String,
    pub attrs: Vec<Attribute>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Attribute {
    Ident(String),
    Arg(String, Vec<String>),
}

impl Attribute {
    pub fn as_middleware(&self) -> Option<String> {
        match *self {
            Attribute::Arg(ref s, ref args) if s == "middleware" && args.len() == 1 => {
                Some(args[0].clone())
            }
            _ => None
        }
    }
}
