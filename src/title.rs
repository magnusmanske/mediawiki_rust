//use serde_json::Value;

type NamespaceID = crate::api::NamespaceID;

#[derive(Debug, Clone, PartialEq)]
pub struct Title {
    title: String,
    namespace_id: NamespaceID,
}

impl Title {
    pub fn new(title: &str, namespace_id: NamespaceID) -> Title {
        Title {
            title: title.to_string(),
            namespace_id: namespace_id,
        }
    }

    pub fn new_from_full(full_title: &String, api: &crate::api::Api) -> Self {
        let mut v: Vec<&str> = full_title.split(":").collect();
        println!("{:#?}", &v);
        if v.len() == 1 {
            return Self::new(&full_title.to_string(), 0);
        }
        let namespace_name = Title::underscores_to_spaces(&v.remove(0).to_string());
        let title = v.join(":");
        let site_info = api.get_site_info();

        // Canonical namespaces
        match site_info["namespaces"].as_object() {
            Some(namespaces) => {
                for (_, ns) in namespaces {
                    match ns["*"].as_str() {
                        Some(namespace) => {
                            println!("1: {} / {}", &namespace_name, &namespace);
                            if Title::underscores_to_spaces(&namespace.to_string())
                                == namespace_name
                            {
                                return Self::new(&title.to_string(), ns["id"].as_i64().unwrap());
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        // Aliases
        match site_info["namespacealiases"].as_array() {
            Some(namespaces) => {
                for ns in namespaces {
                    match ns["*"].as_str() {
                        Some(namespace) => {
                            println!("2: {} / {}", &namespace_name, &namespace);
                            if Title::underscores_to_spaces(&namespace.to_string())
                                == namespace_name
                            {
                                return Self::new(&title.to_string(), ns["id"].as_i64().unwrap());
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        // Fallback
        Self::new(&full_title.to_string(), 0)
    }

    pub fn new_from_api_result(data: &serde_json::Value) -> Title {
        Title {
            title: data["title"].as_str().unwrap_or("").to_string(),
            namespace_id: data["ns"].as_i64().unwrap_or(0).into(),
        }
    }

    pub fn namespace_id(&self) -> NamespaceID {
        self.namespace_id
    }

    pub fn namespace_name(&self, api: &crate::api::Api) -> Option<String> {
        api.get_canonical_namespace_name(self.namespace_id)
    }

    pub fn with_underscores(&self) -> String {
        Title::spaces_to_underscores(&self.title)
    }

    pub fn pretty(&self) -> String {
        Title::underscores_to_spaces(&self.title)
    }

    pub fn full_with_underscores(&self, api: &crate::api::Api) -> Option<String> {
        Some(
            Title::spaces_to_underscores(&self.namespace_name(api)?)
                + ":"
                + &Title::spaces_to_underscores(&self.title),
        )
    }

    pub fn full_pretty(&self, api: &crate::api::Api) -> Option<String> {
        Some(
            Title::underscores_to_spaces(&self.namespace_name(api)?)
                + ":"
                + &Title::underscores_to_spaces(&self.title),
        )
    }

    fn spaces_to_underscores(s: &String) -> String {
        s.trim().replace(" ", "_")
    }

    fn underscores_to_spaces(s: &String) -> String {
        s.replace("_", " ").trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::*;

    #[test]
    fn new_from_full_main_namespace() {
        let api = Api::new("https://www.wikidata.org/w/api.php").unwrap();
        assert_eq!(
            Title::new_from_full(&"Main namespace".to_string(), &api),
            Title::new("Main namespace", 0)
        );
    }

    #[test]
    fn new_from_full_canonical_namespace() {
        let api = Api::new("https://www.wikidata.org/w/api.php").unwrap();
        assert_eq!(
            Title::new_from_full(&"File:Some file.jpg".to_string(), &api),
            Title::new("Some file.jpg", 6)
        );
    }

    #[test]
    fn new_from_full_canonical_namespace_with_colon() {
        let api = Api::new("https://www.wikidata.org/w/api.php").unwrap();
        assert_eq!(
            Title::new_from_full(&"Project talk:A project:yes, really".to_string(), &api),
            Title::new("A project:yes, really", 5)
        );
    }

    #[test]
    fn new_from_full_namespace_alias() {
        let api = Api::new("https://www.wikidata.org/w/api.php").unwrap();
        assert_eq!(
            Title::new_from_full(&"Item:Q12345".to_string(), &api),
            Title::new("Q12345", 0)
        );
    }

    #[test]
    fn new_from_full_invalid_namespace() {
        let api = Api::new("https://www.wikidata.org/w/api.php").unwrap();
        assert_eq!(
            Title::new_from_full(&"This is not a namespace:A title".to_string(), &api),
            Title::new("This is not a namespace:A title", 0)
        );
    }

}
