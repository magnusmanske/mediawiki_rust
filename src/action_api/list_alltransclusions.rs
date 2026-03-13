use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=alltransclusions` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListAlltranslusionsData {
    atcontinue: Option<String>,
    atfrom: Option<String>,
    atto: Option<String>,
    atprefix: Option<String>,
    atunique: bool,
    atprop: Option<Vec<String>>,
    atnamespace: NamespaceID,
    atlimit: usize,
    atdir: Option<String>,
}

impl ActionApiData for ActionApiListAlltranslusionsData {}

impl Default for ActionApiListAlltranslusionsData {
    fn default() -> Self {
        Self {
            atcontinue: None,
            atfrom: None,
            atto: None,
            atprefix: None,
            atunique: false,
            atprop: None,
            atnamespace: 10,
            atlimit: 10,
            atdir: None,
        }
    }
}

impl ActionApiListAlltranslusionsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.atcontinue, "atcontinue", &mut params);
        Self::add_str(&self.atfrom, "atfrom", &mut params);
        Self::add_str(&self.atto, "atto", &mut params);
        Self::add_str(&self.atprefix, "atprefix", &mut params);
        Self::add_boolean(self.atunique, "atunique", &mut params);
        Self::add_vec(&self.atprop, "atprop", &mut params);
        params.insert("atnamespace".to_string(), self.atnamespace.to_string());
        params.insert("atlimit".to_string(), self.atlimit.to_string());
        Self::add_str(&self.atdir, "atdir", &mut params);
        params
    }
}

/// Builder for `list=alltransclusions` — lists all transclusions.
#[derive(Debug, Clone)]
pub struct ActionApiListAlltranslusionsBuilder {
    pub(crate) data: ActionApiListAlltranslusionsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListAlltranslusionsBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListAlltranslusionsData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this title (`atfrom`).
    pub fn atfrom<S: AsRef<str>>(mut self, atfrom: S) -> Self {
        self.data.atfrom = Some(atfrom.as_ref().to_string());
        self
    }

    /// Stop listing at this title (`atto`).
    pub fn atto<S: AsRef<str>>(mut self, atto: S) -> Self {
        self.data.atto = Some(atto.as_ref().to_string());
        self
    }

    /// Prefix to search for (`atprefix`).
    pub fn atprefix<S: AsRef<str>>(mut self, atprefix: S) -> Self {
        self.data.atprefix = Some(atprefix.as_ref().to_string());
        self
    }

    /// Only show distinct transcluded titles (`atunique`).
    pub fn atunique(mut self, atunique: bool) -> Self {
        self.data.atunique = atunique;
        self
    }

    /// Properties to return (`atprop`).
    pub fn atprop<S: Into<String> + Clone>(mut self, atprop: &[S]) -> Self {
        self.data.atprop = Some(atprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Namespace to enumerate (`atnamespace`).
    pub fn atnamespace(mut self, atnamespace: NamespaceID) -> Self {
        self.data.atnamespace = atnamespace;
        self
    }

    /// Maximum number of items to return (`atlimit`).
    pub fn atlimit(mut self, atlimit: usize) -> Self {
        self.data.atlimit = atlimit;
        self
    }

    /// Direction to list (`atdir`).
    pub fn atdir<S: AsRef<str>>(mut self, atdir: S) -> Self {
        self.data.atdir = Some(atdir.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListAlltranslusionsBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "alltransclusions".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListAlltranslusionsBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListAlltranslusionsBuilder {
        ActionApiListAlltranslusionsBuilder::new()
    }

    #[test]
    fn default_atnamespace_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["atnamespace"], "10");
    }

    #[test]
    fn default_atlimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["atlimit"], "10");
    }

    #[test]
    fn atprefix_set() {
        let params = new_builder().atprefix("Infobox").data.params();
        assert_eq!(params["atprefix"], "Infobox");
    }

    #[test]
    fn atnamespace_set() {
        let params = new_builder().atnamespace(4).data.params();
        assert_eq!(params["atnamespace"], "4");
    }

    #[test]
    fn atlimit_set() {
        let params = new_builder().atlimit(50).data.params();
        assert_eq!(params["atlimit"], "50");
    }

    #[test]
    fn atunique_flag() {
        let params = new_builder().atunique(true).data.params();
        assert_eq!(params["atunique"], "");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "alltransclusions");
    }
}
