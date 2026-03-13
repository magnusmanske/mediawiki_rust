use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=exturlusage` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListExturlusageData {
    euquery: Option<String>,
    euprotocol: Option<String>,
    eunamespace: Option<Vec<NamespaceID>>,
    euprop: Option<Vec<String>>,
    eulimit: usize,
    euexpandurl: bool,
    eucontinue: Option<String>,
}

impl ActionApiData for ActionApiListExturlusageData {}

impl Default for ActionApiListExturlusageData {
    fn default() -> Self {
        Self {
            euquery: None,
            euprotocol: None,
            eunamespace: None,
            euprop: None,
            eulimit: 10,
            euexpandurl: false,
            eucontinue: None,
        }
    }
}

impl ActionApiListExturlusageData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.euquery, "euquery", &mut params);
        Self::add_str(&self.euprotocol, "euprotocol", &mut params);
        if let Some(ref ns) = self.eunamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("eunamespace".to_string(), s.join("|"));
        }
        Self::add_vec(&self.euprop, "euprop", &mut params);
        params.insert("eulimit".to_string(), self.eulimit.to_string());
        Self::add_boolean(self.euexpandurl, "euexpandurl", &mut params);
        Self::add_str(&self.eucontinue, "eucontinue", &mut params);
        params
    }
}

/// Builder for `list=exturlusage` — enumerates pages containing a given URL.
#[derive(Debug, Clone)]
pub struct ActionApiListExturlusageBuilder {
    pub(crate) data: ActionApiListExturlusageData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListExturlusageBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListExturlusageData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// URL to search for (`euquery`).
    pub fn euquery<S: AsRef<str>>(mut self, euquery: S) -> Self {
        self.data.euquery = Some(euquery.as_ref().to_string());
        self
    }

    /// Protocol of the URL (`euprotocol`).
    pub fn euprotocol<S: AsRef<str>>(mut self, euprotocol: S) -> Self {
        self.data.euprotocol = Some(euprotocol.as_ref().to_string());
        self
    }

    /// Namespaces to enumerate (`eunamespace`).
    pub fn eunamespace(mut self, eunamespace: &[NamespaceID]) -> Self {
        self.data.eunamespace = Some(eunamespace.to_vec());
        self
    }

    /// Properties to return (`euprop`).
    pub fn euprop<S: Into<String> + Clone>(mut self, euprop: &[S]) -> Self {
        self.data.euprop = Some(euprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of pages to return (`eulimit`).
    pub fn eulimit(mut self, eulimit: usize) -> Self {
        self.data.eulimit = eulimit;
        self
    }

    /// Expand protocol-relative URLs with the canonical protocol (`euexpandurl`).
    pub fn euexpandurl(mut self, euexpandurl: bool) -> Self {
        self.data.euexpandurl = euexpandurl;
        self
    }
}

impl ActionApiRunnable for ActionApiListExturlusageBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "exturlusage".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListExturlusageBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListExturlusageBuilder {
        ActionApiListExturlusageBuilder::new()
    }

    #[test]
    fn default_eulimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["eulimit"], "10");
    }

    #[test]
    fn default_euquery_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("euquery"));
    }

    #[test]
    fn euquery_set() {
        let params = new_builder().euquery("example.com").data.params();
        assert_eq!(params["euquery"], "example.com");
    }

    #[test]
    fn euprotocol_set() {
        let params = new_builder().euprotocol("https").data.params();
        assert_eq!(params["euprotocol"], "https");
    }

    #[test]
    fn eunamespace_set() {
        let params = new_builder().eunamespace(&[0, 4]).data.params();
        assert_eq!(params["eunamespace"], "0|4");
    }

    #[test]
    fn eulimit_set() {
        let params = new_builder().eulimit(50).data.params();
        assert_eq!(params["eulimit"], "50");
    }

    #[test]
    fn euprop_set() {
        let params = new_builder().euprop(&["ids", "title", "url"]).data.params();
        assert_eq!(params["euprop"], "ids|title|url");
    }

    #[test]
    fn euexpandurl_flag() {
        let params = new_builder().euexpandurl(true).data.params();
        assert_eq!(params["euexpandurl"], "");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "exturlusage");
    }
}
