use super::{ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=random` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListRandomData {
    rnnamespace: Option<Vec<NamespaceID>>,
    rnfilterredir: Option<String>,
    rnlimit: usize,
    rnredirect: bool,
}

impl ActionApiData for ActionApiListRandomData {}

impl Default for ActionApiListRandomData {
    fn default() -> Self {
        Self {
            rnnamespace: None,
            rnfilterredir: None,
            rnlimit: 1,
            rnredirect: false,
        }
    }
}

impl ActionApiListRandomData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(ref ns) = self.rnnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("rnnamespace".to_string(), s.join("|"));
        }
        Self::add_str(&self.rnfilterredir, "rnfilterredir", &mut params);
        params.insert("rnlimit".to_string(), self.rnlimit.to_string());
        Self::add_boolean(self.rnredirect, "rnredirect", &mut params);
        params
    }
}

/// Builder for `list=random` — gets a set of random pages.
#[derive(Debug, Clone)]
pub struct ActionApiListRandomBuilder {
    pub(crate) data: ActionApiListRandomData,
}

impl ActionApiListRandomBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListRandomData::default(),
        }
    }

    /// Namespaces to return random pages from (`rnnamespace`).
    pub fn rnnamespace(mut self, rnnamespace: &[NamespaceID]) -> Self {
        self.data.rnnamespace = Some(rnnamespace.to_vec());
        self
    }

    /// Filter redirects: `"all"`, `"redirects"`, or `"nonredirects"` (`rnfilterredir`).
    pub fn rnfilterredir<S: AsRef<str>>(mut self, rnfilterredir: S) -> Self {
        self.data.rnfilterredir = Some(rnfilterredir.as_ref().to_string());
        self
    }

    /// Maximum number of random pages to return (`rnlimit`).
    pub fn rnlimit(mut self, rnlimit: usize) -> Self {
        self.data.rnlimit = rnlimit;
        self
    }

    /// Use deprecated redirect parameter (`rnredirect`).
    pub fn rnredirect(mut self, rnredirect: bool) -> Self {
        self.data.rnredirect = rnredirect;
        self
    }
}

impl ActionApiRunnable for ActionApiListRandomBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "random".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListRandomBuilder {
        ActionApiListRandomBuilder::new()
    }

    #[test]
    fn default_rnlimit_is_1() {
        let params = new_builder().data.params();
        assert_eq!(params["rnlimit"], "1");
    }

    #[test]
    fn default_rnnamespace_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("rnnamespace"));
    }

    #[test]
    fn rnnamespace_set() {
        let params = new_builder().rnnamespace(&[0, 4]).data.params();
        assert_eq!(params["rnnamespace"], "0|4");
    }

    #[test]
    fn rnfilterredir_set() {
        let params = new_builder().rnfilterredir("nonredirects").data.params();
        assert_eq!(params["rnfilterredir"], "nonredirects");
    }

    #[test]
    fn rnlimit_set() {
        let params = new_builder().rnlimit(5).data.params();
        assert_eq!(params["rnlimit"], "5");
    }

    #[test]
    fn rnredirect_flag() {
        let params = new_builder().rnredirect(true).data.params();
        assert_eq!(params["rnredirect"], "");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "random");
    }
}
