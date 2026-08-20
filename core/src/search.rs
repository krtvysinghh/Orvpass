#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    pub raw: String,
}

impl SearchQuery {
    pub fn new(query: impl Into<String>) -> Self {
        Self { raw: query.into() }
    }
}
