/*!
The `Title` class deals with page titles and namespaces
*/

#![deny(missing_docs)]

/// Shortcut for crate::api::NamespaceID
type NamespaceID = crate::api::NamespaceID;

/// If the provided ID refers to a...
///
/// * content namespace, return the ID of the corresponding talk namespace.
/// * talk namespace, return the ID of the corresponding content namespace.
/// * special namespace, return None.
///
/// # Examples
///
/// ```
/// use mediawiki::title::toggle_namespace_id;
/// assert_eq!(toggle_namespace_id(0), Some(1));
/// assert_eq!(toggle_namespace_id(1), Some(0));
/// assert_eq!(toggle_namespace_id(-1), None);
/// ```
pub fn toggle_namespace_id(id: NamespaceID) -> Option<NamespaceID> {
    match id {
        n if n >= 0 && n % 2 == 0 => Some(n + 1),
        n if n >= 0 && n % 2 == 1 => Some(n - 1),
        _ => None,
    }
}

/// Title struct
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Title {
    title: String, // Always stored without underscores
    namespace_id: NamespaceID,
}

impl Title {
    /// Constructor, where un-prefixed title and namespace are known.
    /// Assumes title has correct capitalization
    pub fn new(title: &str, namespace_id: NamespaceID) -> Title {
        Title {
            title: Title::underscores_to_spaces(title),
            namespace_id,
        }
    }

    /// Constructor, where full namespace-prefixed title is known.
    /// Uses Api to parse valid namespaces
    pub fn new_from_full(full_title: &str, api: &crate::api::Api) -> Self {
        let mut v: Vec<&str> = full_title.split(':').collect();
        if v.len() == 1 {
            return Self::new(full_title, 0);
        }
        let namespace_name = Title::first_letter_uppercase(v.remove(0));
        let title = Title::underscores_to_spaces(&v.join(":"));
        let site_info = api.get_site_info();

        // Canonical namespaces
        if let Some(namespaces) = site_info["query"]["namespaces"].as_object() {
            for (_, ns) in namespaces {
                if let Some(namespace) = ns["*"].as_str() {
                    if Title::underscores_to_spaces(namespace) == namespace_name {
                        return Self::new_from_namespace_object(title, ns);
                    }
                }
                if let Some(namespace) = ns["canonical"].as_str() {
                    if Title::underscores_to_spaces(namespace) == namespace_name {
                        return Self::new_from_namespace_object(title, ns);
                    }
                }
            }
        }

        // Aliases
        if let Some(namespaces) = site_info["query"]["namespacealiases"].as_array() {
            for ns in namespaces {
                if let Some(namespace) = ns["*"].as_str() {
                    if Title::underscores_to_spaces(namespace) == namespace_name {
                        let namespace_id = ns["id"].as_i64().unwrap_or(0);
                        let title = match ns["case"].as_str() {
                            Some("first-letter") => Title::first_letter_uppercase(&title),
                            _ => title,
                        };
                        return Self::new(&title, namespace_id);
                    }
                }
            }
        }

        // Fallback
        Self::new(full_title, 0)
    }

    /// Constructor, used internally by `new_from_full`
    fn new_from_namespace_object(title: String, ns: &serde_json::Value) -> Self {
        let namespace_id = ns["id"].as_i64().unwrap_or_default();
        let title = match ns["case"].as_str() {
            Some("first-letter") => Title::first_letter_uppercase(&title),
            _ => title,
        };
        Self::new(&title, namespace_id)
    }

    /// Constructor, used by ``Api::result_array_to_titles``
    pub fn new_from_api_result(data: &serde_json::Value) -> Title {
        let namespace_id = data["ns"].as_i64().unwrap_or(0);
        let mut title = data["title"].as_str().unwrap_or("").to_string();

        // If namespace!=0, remove namespace prefix. THIS IS NOT IDEAL AND SHOULD USE Api AS IN new_from_full!
        if namespace_id > 0 {
            let mut v: Vec<&str> = title.split(':').collect();
            if v.len() > 1 {
                v.remove(0);
                title = v.join(":");
            }
        }

        Title {
            title: Title::underscores_to_spaces(&title),
            namespace_id,
        }
    }

    /// Returns the namespace ID
    pub fn namespace_id(&self) -> NamespaceID {
        self.namespace_id
    }

    /// Returns the canonical namespace text, based on the Api
    pub fn namespace_name<'a>(&self, api: &'a crate::api::Api) -> Option<&'a str> {
        api.get_canonical_namespace_name(self.namespace_id)
    }

    /// Returns the local namespace text, based on the Api
    pub fn local_namespace_name<'a>(&self, api: &'a crate::api::Api) -> Option<&'a str> {
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
            match Title::spaces_to_underscores(self.local_namespace_name(api)?).as_str() {
                "" => self.with_underscores(),
                ns => ns.to_owned() + ":" + &self.with_underscores(),
            },
        )
    }

    /// Returns the namespace-prefixed title, with spaces instead of underscores
    pub fn full_pretty(&self, api: &crate::api::Api) -> Option<String> {
        Some(
            match Title::underscores_to_spaces(self.local_namespace_name(api)?).as_str() {
                "" => self.pretty().to_string(),
                ns => ns.to_owned() + ":" + self.pretty(),
            },
        )
    }

    /// Changes all spaces to underscores
    pub fn spaces_to_underscores(s: &str) -> String {
        s.trim().replace(' ', "_")
    }

    /// Changes all underscores to spaces
    pub fn underscores_to_spaces(s: &str) -> String {
        s.replace('_', " ").trim().to_string()
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

    /// Changes this Title to refer to the other member of the corresponding
    /// article-talk page pair for this page. Won't change Special pages.
    ///
    /// # Examples
    ///
    /// ```
    /// use mediawiki::title::Title;
    /// let mut title1 = Title::new("Test", 0);
    /// title1.toggle_talk();
    /// assert_eq!(title1, Title::new("Test", 1));
    ///
    /// let mut title2 = Title::new("Test", 1);
    /// title2.toggle_talk();
    /// assert_eq!(title2, Title::new("Test", 0));
    ///
    /// let mut title3 = Title::new("Test", -1);
    /// title3.toggle_talk();
    /// assert_eq!(title3, Title::new("Test", -1));
    /// ```
    pub fn toggle_talk(&mut self) {
        self.namespace_id = toggle_namespace_id(self.namespace_id).unwrap_or(self.namespace_id);
    }

    /// Returns a new Title referring to the other member of the corresponding
    /// article-talk page pair for this page. Won't change Special pages.
    ///
    /// # Examples
    ///
    /// ```
    /// use mediawiki::title::Title;
    /// assert_eq!(Title::new("Test", 0).into_toggle_talk(),
    ///     Title::new("Test", 1));
    ///
    /// assert_eq!(Title::new("Test", 1).into_toggle_talk(),
    ///     Title::new("Test", 0));
    ///
    /// assert_eq!(Title::new("Test", -1).into_toggle_talk(),
    ///     Title::new("Test", -1));
    /// ```
    pub fn into_toggle_talk(self) -> Self {
        Title::new(
            &self.title,
            toggle_namespace_id(self.namespace_id).unwrap_or(self.namespace_id),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::*;

    async fn wd_api() -> Api {
        Api::new("https://www.wikidata.org/w/api.php")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn new_from_full_main_namespace() {
        assert_eq!(
            Title::new_from_full(&"Main namespace", &wd_api().await),
            Title::new("Main namespace", 0)
        );
    }

    #[tokio::test]
    async fn new_from_full_canonical_namespace() {
        assert_eq!(
            Title::new_from_full(&"File:Some file.jpg", &wd_api().await),
            Title::new("Some file.jpg", 6)
        );
    }

    #[tokio::test]
    async fn new_from_full_canonical_namespace_with_colon() {
        assert_eq!(
            Title::new_from_full(&"Project talk:A project:yes, really", &wd_api().await),
            Title::new("A project:yes, really", 5)
        );
    }

    #[tokio::test]
    async fn new_from_full_namespace_alias() {
        assert_eq!(
            Title::new_from_full(&"Item:Q12345", &wd_api().await),
            Title::new("Q12345", 0)
        );
    }

    #[tokio::test]
    async fn new_from_full_special_namespace() {
        assert_eq!(
            Title::new_from_full(&"Special:A title", &wd_api().await),
            Title::new("A title", -1)
        );
    }

    #[tokio::test]
    async fn new_from_full_invalid_namespace() {
        assert_eq!(
            Title::new_from_full(&"This is not a namespace:A title", &wd_api().await),
            Title::new("This is not a namespace:A title", 0)
        );
    }

    #[tokio::test]
    async fn spaces_to_underscores() {
        assert_eq!(
            Title::spaces_to_underscores(&" A little  test "),
            "A_little__test"
        );
    }

    #[tokio::test]
    async fn underscores_to_spaces() {
        assert_eq!(
            Title::underscores_to_spaces(&"_A_little__test_"),
            "A little  test"
        );
    }

    #[tokio::test]
    async fn first_letter_uppercase() {
        assert_eq!(Title::first_letter_uppercase(&""), "");
        assert_eq!(Title::first_letter_uppercase(&"FooBar"), "FooBar");
        assert_eq!(Title::first_letter_uppercase(&"fooBar"), "FooBar");
        assert_eq!(Title::first_letter_uppercase(&"über"), "Über");
    }

    #[tokio::test]
    async fn full() {
        let api = &wd_api().await;
        let title = Title::new_from_full(&"User talk:Magnus_Manske", api);
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
