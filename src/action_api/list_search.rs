use super::{
    ActionApiContinuable, ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `list=search` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListSearchData {
    srsearch: Option<String>,
    srnamespace: Option<Vec<NamespaceID>>,
    srlimit: usize,
    sroffset: usize,
    srwhat: Option<String>,
    srinfo: Option<Vec<String>>,
    srprop: Option<Vec<String>>,
    srinterwiki: bool,
    srsort: Option<String>,
}

impl ActionApiData for ActionApiListSearchData {}

impl Default for ActionApiListSearchData {
    fn default() -> Self {
        Self {
            srsearch: None,
            srnamespace: None,
            srlimit: 10,
            sroffset: 0,
            srwhat: None,
            srinfo: None,
            srprop: None,
            srinterwiki: false,
            srsort: None,
        }
    }
}

impl ActionApiListSearchData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(srsearch) = &self.srsearch {
            params.insert("srsearch".to_string(), srsearch.clone());
        }
        if let Some(ns) = &self.srnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("srnamespace".to_string(), s.join("|"));
        }
        params.insert("srlimit".to_string(), self.srlimit.to_string());
        if self.sroffset > 0 {
            params.insert("sroffset".to_string(), self.sroffset.to_string());
        }
        Self::add_str(&self.srwhat, "srwhat", &mut params);
        Self::add_vec(&self.srinfo, "srinfo", &mut params);
        Self::add_vec(&self.srprop, "srprop", &mut params);
        Self::add_boolean(self.srinterwiki, "srinterwiki", &mut params);
        Self::add_str(&self.srsort, "srsort", &mut params);
        params
    }
}

/// Builder for the `list=search` API module; supports pagination via `ActionApiContinuable`.
#[derive(Debug, Clone)]
pub struct ActionApiListSearchBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListSearchData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListSearchBuilder<T> {
    /// Filter results to pages in these namespaces (`srnamespace`).
    pub fn srnamespace(mut self, srnamespace: &[NamespaceID]) -> Self {
        self.data.srnamespace = Some(srnamespace.to_vec());
        self
    }

    /// Maximum number of results to return (`srlimit`).
    pub fn srlimit(mut self, srlimit: usize) -> Self {
        self.data.srlimit = srlimit;
        self
    }

    /// Number of results to skip before returning (`sroffset`).
    pub fn sroffset(mut self, sroffset: usize) -> Self {
        self.data.sroffset = sroffset;
        self
    }

    /// Which type of search to perform: `title`, `text`, or `nearmatch` (`srwhat`).
    pub fn srwhat<S: AsRef<str>>(mut self, srwhat: S) -> Self {
        self.data.srwhat = Some(srwhat.as_ref().to_string());
        self
    }

    /// Metadata to return about the search results (e.g. `totalhits`) (`srinfo`).
    pub fn srinfo<S: Into<String> + Clone>(mut self, srinfo: &[S]) -> Self {
        self.data.srinfo = Some(srinfo.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Properties to retrieve for each search result (`srprop`).
    pub fn srprop<S: Into<String> + Clone>(mut self, srprop: &[S]) -> Self {
        self.data.srprop = Some(srprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// When set, include interwiki results in the search response (`srinterwiki`).
    pub fn srinterwiki(mut self, srinterwiki: bool) -> Self {
        self.data.srinterwiki = srinterwiki;
        self
    }

    /// Sort order for search results (`srsort`).
    pub fn srsort<S: AsRef<str>>(mut self, srsort: S) -> Self {
        self.data.srsort = Some(srsort.as_ref().to_string());
        self
    }
}

impl ActionApiListSearchBuilder<NoTitlesOrGenerator> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListSearchData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Search query string (`srsearch`).
    pub fn srsearch<S: AsRef<str>>(mut self, srsearch: S) -> ActionApiListSearchBuilder<Runnable> {
        self.data.srsearch = Some(srsearch.as_ref().to_string());
        ActionApiListSearchBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiRunnable for ActionApiListSearchBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "search".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListSearchBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListSearchBuilder<NoTitlesOrGenerator> {
        ActionApiListSearchBuilder::new()
    }

    #[test]
    fn default_srlimit_is_10() {
        let params = new_builder().srsearch("foo").data.params();
        assert_eq!(params["srlimit"], "10");
    }

    #[test]
    fn srsearch_set() {
        let params = new_builder().srsearch("Albert Einstein").data.params();
        assert_eq!(params["srsearch"], "Albert Einstein");
    }

    #[test]
    fn srnamespace_set() {
        let params = new_builder()
            .srnamespace(&[0, 4])
            .srsearch("Foo")
            .data
            .params();
        assert_eq!(params["srnamespace"], "0|4");
    }

    #[test]
    fn srlimit_set() {
        let params = new_builder().srlimit(20).srsearch("Foo").data.params();
        assert_eq!(params["srlimit"], "20");
    }

    #[test]
    fn sroffset_zero_absent() {
        let params = new_builder().srsearch("Foo").data.params();
        assert!(!params.contains_key("sroffset"));
    }

    #[test]
    fn sroffset_nonzero_set() {
        let params = new_builder().sroffset(10).srsearch("Foo").data.params();
        assert_eq!(params["sroffset"], "10");
    }

    #[test]
    fn srwhat_set() {
        let params = new_builder().srwhat("text").srsearch("Foo").data.params();
        assert_eq!(params["srwhat"], "text");
    }

    #[test]
    fn srprop_set() {
        let params = new_builder()
            .srprop(&["snippet", "size", "wordcount"])
            .srsearch("Foo")
            .data
            .params();
        assert_eq!(params["srprop"], "snippet|size|wordcount");
    }

    #[test]
    fn srinterwiki_set() {
        let params = new_builder()
            .srinterwiki(true)
            .srsearch("Foo")
            .data
            .params();
        assert!(params.contains_key("srinterwiki"));
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().srsearch("Albert");
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "search");
    }

    #[tokio::test]
    async fn test_search() {
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("list", "search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "search": [
                        {"ns": 0, "title": "Albert Einstein", "pageid": 736},
                        {"ns": 0, "title": "Albert Einstein Award", "pageid": 1000},
                        {"ns": 0, "title": "Albert Einstein Medal", "pageid": 1001}
                    ]
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiList::search()
            .srsearch("Albert Einstein")
            .srlimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["search"].is_array());
    }
}
