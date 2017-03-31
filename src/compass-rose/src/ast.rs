pub struct Resource {
    pub header: ResourceHeader,
    pub members: Vec<ResourceMember>,
    pub attrs: Vec<Attribute>,
}

pub struct ResourceHeader {
    pub ty: String,
    pub endpoint: Option<String>,
    pub methods: Option<Vec<Method>>,
}

pub enum ResourceMember {
    Relation(Relation),
}

pub struct Relation {
    pub rel: String,
    pub endpoint: Option<String>,
    pub methods: Option<Vec<Method>>,
}

#[derive(Clone, Copy)]
pub enum Method {
    Get,
    Index,
}

pub enum Attribute {
    Ident(String),
    Arg(String, Vec<String>),
}
