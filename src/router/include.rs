#[derive(Clone)]
pub struct IncludeQuery {
    pub direct: String,
    pub transitive: Vec<IncludeQuery>,
}

impl IncludeQuery {
    pub fn is_of(&self, relation: &str) -> bool {
        self.direct == relation
    }
}
