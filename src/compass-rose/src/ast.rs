#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Routes {
    pub asset_handler: Option<String>,
    pub setup: Option<Setup>,
    pub resources: Vec<Resource>,
}
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Setup {
    pub members: Vec<SetupMember>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SetupMember {
    Connection(Connection),
}

impl SetupMember {
    pub fn as_conn(&self) -> Option<&Connection> {
        match *self {
            SetupMember::Connection(ref c) => Some(c),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Connection {
    pub conn: String,
    pub name: String,
}

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
    Relation(Relation, Vec<Attribute>),
    Method(Method, Vec<Attribute>),
}

impl ResourceMember {
    pub fn as_relation(&self) -> Option<&Relation> {
        match *self {
            ResourceMember::Relation(ref rel, _)    => Some(rel),
            _                                       => None,
        }
    }

    pub fn as_method(&self) -> Option<(&Method, &[Attribute])> {
        match *self {
            ResourceMember::Method(ref method, ref attrs)   => Some((method, &attrs[..])),
            _                                               => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Relation {
    pub rel: String,
    pub endpoint: Option<String>,
    pub members: Vec<RelationMember>,
    pub kind: RelationKind,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RelationKind {
    Single,
    Many,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RelationMember {
    Method(Method, Vec<Attribute>),
}

impl RelationMember {
    pub fn as_method(&self) -> Option<(&Method, &[Attribute])> {
        match *self {
            RelationMember::Method(ref method, ref attrs)   => Some((method, &attrs[..])),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Method {
    pub method: String,
    pub format: String,
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
