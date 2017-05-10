use rigging::http;
use rigging::resource::ResourceEndpoint;

pub struct Fields {
    fields: Vec<String>,
}

impl Fields {
    pub fn new<T: ResourceEndpoint>(req: &http::Request) -> Option<Self> {
        let q = req.query();

        // Get only the query string for the fields of this resource
        let q = q.and_then(|q| q.split('&').filter(|q| q.starts_with("fields")).find(|q| {
            q.trim_left_matches("fields%5b").starts_with(T::RESOURCE)
                || q.trim_left_matches("fields%5B").starts_with(T::RESOURCE)
        }));

        // Get the fields from that list
        let fields = q.and_then(|q| q.splitn(2, '=').last()).map(|fields| {
            fields.split(',').map(String::from).collect()
        });

        fields.map(|fields| Fields { fields })
    }

    pub fn contains(&self, field: &str) -> bool {
        self.fields.iter().any(|f| f == field)
    }
}
