/*!
The `Title` class deals with page titles and namespaces
*/

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate lazy_static;

use std::hash::{Hash, Hasher};

/// Shortcut for crate::api::NamespaceID
type NamespaceID = crate::api::NamespaceID;

/// Title struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title {
    title: String, // Always stored without underscores
    namespace_id: NamespaceID,
}

impl Hash for Title {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.namespace_id.hash(state);
        self.title.hash(state);
    }
}

impl Title {
    /// Constructor, where un-prefixed title and namespace are known.
    /// Assumes title has correct capitalization
    pub fn new(title: &str, namespace_id: NamespaceID) -> Title {
        Title {
            title: Title::underscores_to_spaces(&title.to_string()),
            namespace_id: namespace_id,
        }
    }

    /// Constructor, where full namespace-prefixed title is known.
    /// Uses Api to parse valid namespaces
    pub fn new_from_full(full_title: &str, api: &crate::api::Api) -> Self {
        let mut v: Vec<&str> = full_title.split(":").collect();
        if v.len() == 1 {
            return Self::new(&full_title.to_string(), 0);
        }
        let namespace_name = Title::first_letter_uppercase(&v.remove(0).to_string());
        let title = Title::underscores_to_spaces(&v.join(":"));
        let site_info = api.get_site_info();

        // Canonical namespaces
        match site_info["query"]["namespaces"].as_object() {
            Some(namespaces) => {
                for (_, ns) in namespaces {
                    match ns["*"].as_str() {
                        Some(namespace) => {
                            if Title::underscores_to_spaces(&namespace.to_string())
                                == namespace_name
                            {
                                return Self::new_from_namespace_object(title, ns);
                            }
                        }
                        None => {}
                    }
                    match ns["canonical"].as_str() {
                        Some(namespace) => {
                            if Title::underscores_to_spaces(&namespace.to_string())
                                == namespace_name
                            {
                                return Self::new_from_namespace_object(title, ns);
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        // Aliases
        match site_info["query"]["namespacealiases"].as_array() {
            Some(namespaces) => {
                for ns in namespaces {
                    match ns["*"].as_str() {
                        Some(namespace) => {
                            if Title::underscores_to_spaces(&namespace.to_string())
                                == namespace_name
                            {
                                let namespace_id = ns["id"].as_i64().unwrap();
                                let title = match ns["case"].as_str() {
                                    Some("first-letter") => Title::first_letter_uppercase(&title),
                                    _ => title.to_string(),
                                };
                                return Self::new(&title, namespace_id);
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

    /// Constructor, used internally by `new_from_full`
    fn new_from_namespace_object(title: String, ns: &serde_json::Value) -> Self {
        let namespace_id = ns["id"].as_i64().unwrap();
        let title = match ns["case"].as_str() {
            Some("first-letter") => Title::first_letter_uppercase(&title),
            _ => title.to_string(),
        };
        return Self::new(&title, namespace_id);
    }

    /// Constructor, used by ``Api::result_array_to_titles``
    pub fn new_from_api_result(data: &serde_json::Value) -> Title {
        Title {
            title: Title::underscores_to_spaces(&data["title"].as_str().unwrap_or("").to_string()),
            namespace_id: data["ns"].as_i64().unwrap_or(0).into(),
        }
    }

    /// Returns the namespace ID
    pub fn namespace_id(&self) -> NamespaceID {
        self.namespace_id
    }

    /// Returns the canonical namespace text, based on the Api
    pub fn namespace_name(&self, api: &crate::api::Api) -> Option<String> {
        api.get_canonical_namespace_name(self.namespace_id)
    }

    /// Returns the local namespace text, based on the Api
    pub fn local_namespace_name(&self, api: &crate::api::Api) -> Option<String> {
        api.get_local_namespace_name(self.namespace_id)
    }

    /// Returns the non-namespace-prefixed title, with underscores
    pub fn with_underscores(&self) -> String {
        Title::spaces_to_underscores(&self.title)
    }

    /// Returns the non-namespace-prefixed title, with spaces instead of underscores
    pub fn pretty(&self) -> &str {
        &self.title // was Title::underscores_to_spaces(&self.title) but always storing without underscores
    }

    /// Returns the namespace-prefixed title, with underscores
    pub fn full_with_underscores(&self, api: &crate::api::Api) -> Option<String> {
        Some(
            match Title::spaces_to_underscores(&self.local_namespace_name(api)?).as_str() {
                "" => self.with_underscores(),
                ns => ns.to_owned() + ":" + &self.with_underscores(),
            },
        )
    }

    /// Returns the namespace-prefixed title, with spaces instead of underscores
    pub fn full_pretty(&self, api: &crate::api::Api) -> Option<String> {
        Some(
            match Title::underscores_to_spaces(&self.local_namespace_name(api)?).as_str() {
                "" => self.pretty().to_string(),
                ns => ns.to_owned() + ":" + &self.pretty(),
            },
        )
    }

    /// Changes all spaces to underscores
    pub fn spaces_to_underscores(s: &str) -> String {
        s.trim().replace(" ", "_")
    }

    /// Changes all underscores to spaces
    pub fn underscores_to_spaces(s: &str) -> String {
        s.replace("_", " ").trim().to_string()
    }

    /// Changes the first letter to uppercase.
    /// Enforces spaces instead of underscores.
    pub fn first_letter_uppercase(s: &str) -> String {
        let s = Title::underscores_to_spaces(s);
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::*;

    fn wd_api() -> &'static Api {
        lazy_static! {
            static ref API: Api = Api::new("https://www.wikidata.org/w/api.php").unwrap();
        }
        &API
    }

    #[test]
    fn new_from_full_main_namespace() {
        assert_eq!(
            Title::new_from_full(&"Main namespace".to_string(), wd_api()),
            Title::new("Main namespace", 0)
        );
    }

    #[test]
    fn new_from_full_canonical_namespace() {
        assert_eq!(
            Title::new_from_full(&"File:Some file.jpg".to_string(), wd_api()),
            Title::new("Some file.jpg", 6)
        );
    }

    #[test]
    fn new_from_full_canonical_namespace_with_colon() {
        assert_eq!(
            Title::new_from_full(&"Project talk:A project:yes, really".to_string(), wd_api()),
            Title::new("A project:yes, really", 5)
        );
    }

    #[test]
    fn new_from_full_namespace_alias() {
        assert_eq!(
            Title::new_from_full(&"Item:Q12345".to_string(), wd_api()),
            Title::new("Q12345", 0)
        );
    }

    #[test]
    fn new_from_full_special_namespace() {
        assert_eq!(
            Title::new_from_full(&"Special:A title".to_string(), wd_api()),
            Title::new("A title", -1)
        );
    }

    #[test]
    fn new_from_full_invalid_namespace() {
        assert_eq!(
            Title::new_from_full(&"This is not a namespace:A title".to_string(), wd_api()),
            Title::new("This is not a namespace:A title", 0)
        );
    }

    #[test]
    fn spaces_to_underscores() {
        assert_eq!(
            Title::spaces_to_underscores(&" A little  test ".to_string()),
            "A_little__test"
        );
    }

    #[test]
    fn underscores_to_spaces() {
        assert_eq!(
            Title::underscores_to_spaces(&"_A_little__test_".to_string()),
            "A little  test"
        );
    }

    #[test]
    fn first_letter_uppercase() {
        assert_eq!(Title::first_letter_uppercase(&"".to_string()), "");
        assert_eq!(
            Title::first_letter_uppercase(&"FooBar".to_string()),
            "FooBar"
        );
        assert_eq!(
            Title::first_letter_uppercase(&"fooBar".to_string()),
            "FooBar"
        );
        assert_eq!(Title::first_letter_uppercase(&"über".to_string()), "Über");
    }

    #[test]
    fn full() {
        let api = wd_api();
        let title = Title::new_from_full(&"User talk:Magnus_Manske".to_string(), api);
        assert_eq!(
            title.full_pretty(api),
            Some("User talk:Magnus Manske".to_string())
        );
        assert_eq!(
            title.full_with_underscores(api),
            Some("User_talk:Magnus_Manske".to_string())
        );
    }

}
