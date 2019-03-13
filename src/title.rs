//use serde_json::Value;

#[derive(Debug)]
/// Under development
pub struct Title {
    title: String,
    namespace_id: u32,
}

impl Title {
    pub fn new(title: &str, namespace_id: u32) -> Title {
        Title {
            title: title.to_string(),
            namespace_id: namespace_id,
        }
    }
}
