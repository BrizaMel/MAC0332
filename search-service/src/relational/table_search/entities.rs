pub struct TableSearchInfo {
    pub schema: String,
    pub name: String,
}

impl TableSearchInfo {
    pub fn new(schema: String, name: String) -> Self {
        Self { schema, name }
    }
}
