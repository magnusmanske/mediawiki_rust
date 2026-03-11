use super::{ActionApiData, ActionApiRunnable, Runnable};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

pub(crate) type NoSearch = super::NoTitlesOrGenerator;

/// Internal data container for `action=opensearch` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiOpensearchData {
    search: Option<String>,
    namespace: Option<Vec<NamespaceID>>,
    limit: usize,
    redirects: Option<String>,
}

impl ActionApiData for ActionApiOpensearchData {}

impl Default for ActionApiOpensearchData {
    fn default() -> Self {
        Self {
            search: None,
            namespace: None,
            limit: 10,
            redirects: None,
        }
    }
}

impl ActionApiOpensearchData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "opensearch".to_string());
        Self::add_str(&self.search, "search", &mut params);
        if let Some(ns) = &self.namespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("namespace".to_string(), s.join("|"));
        }
        params.insert("limit".to_string(), self.limit.to_string());
        Self::add_str(&self.redirects, "redirects", &mut params);
        params
    }
}

/// Builder for `action=opensearch`. Call `.search()` to set the search string and make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiOpensearchBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiOpensearchData,
}

impl<T> ActionApiOpensearchBuilder<T> {
    /// Namespaces to search within (`namespace`).
    pub fn namespace(mut self, namespace: &[NamespaceID]) -> Self {
        self.data.namespace = Some(namespace.to_vec());
        self
    }

    /// Maximum number of results to return (`limit`).
    pub fn limit(mut self, limit: usize) -> Self {
        self.data.limit = limit;
        self
    }

    /// How to handle redirect pages in results, e.g. `"resolve"` (`redirects`).
    pub fn redirects<S: AsRef<str>>(mut self, redirects: S) -> Self {
        self.data.redirects = Some(redirects.as_ref().to_string());
        self
    }
}

impl ActionApiOpensearchBuilder<NoSearch> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiOpensearchData::default(),
        }
    }

    /// Search string to look up (`search`).
    pub fn search<S: AsRef<str>>(mut self, search: S) -> ActionApiOpensearchBuilder<Runnable> {
        self.data.search = Some(search.as_ref().to_string());
        ActionApiOpensearchBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiOpensearchBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApi};

    fn new_builder() -> ActionApiOpensearchBuilder<NoSearch> {
        ActionApiOpensearchBuilder::new()
    }

    #[test]
    fn search_set() {
        let params = new_builder().search("Albert Einstein").data.params();
        assert_eq!(params["search"], "Albert Einstein");
    }

    #[test]
    fn default_limit_10() {
        let params = new_builder().search("foo").data.params();
        assert_eq!(params["limit"], "10");
    }

    #[test]
    fn limit_set() {
        let params = new_builder().limit(5).search("foo").data.params();
        assert_eq!(params["limit"], "5");
    }

    #[test]
    fn namespace_set() {
        let params = new_builder().namespace(&[0, 4]).search("foo").data.params();
        assert_eq!(params["namespace"], "0|4");
    }

    #[test]
    fn redirects_set() {
        let params = new_builder()
            .redirects("resolve")
            .search("foo")
            .data
            .params();
        assert_eq!(params["redirects"], "resolve");
    }

    #[test]
    fn action_is_opensearch() {
        let params = new_builder().search("foo").data.params();
        assert_eq!(params["action"], "opensearch");
    }

    #[test]
    fn http_method_is_get() {
        let builder = new_builder().search("foo");
        assert_eq!(builder.http_method(), "GET");
    }

    #[tokio::test]
    async fn test_opensearch() {
        let api = Api::new("https://en.wikipedia.org/w/api.php")
            .await
            .unwrap();
        let result = ActionApi::opensearch()
            .search("Albert Einstein")
            .limit(3)
            .run(&api)
            .await
            .unwrap();
        assert!(result.is_array());
    }
}
