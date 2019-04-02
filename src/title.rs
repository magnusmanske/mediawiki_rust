//use serde_json::Value;

#[derive(Debug)]
/// Under development
pub struct Title {
    title: String,
    namespace_id: u64,
}

impl Title {
    pub fn new(title: &str, namespace_id: u64) -> Title {
        Title {
            title: title.to_string(),
            namespace_id: namespace_id,
        }
    }

    pub fn new_from_api_result(data: &serde_json::Value) -> Title {
        Title {
            title: data["title"].as_str().unwrap_or("").to_string(),
            namespace_id: data["ns"].as_u64().unwrap_or(0),
        }
    }
}
