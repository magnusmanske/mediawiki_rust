use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=allredirects` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListAllredirectsData {
    arcontinue: Option<String>,
    arfrom: Option<String>,
    arto: Option<String>,
    arprefix: Option<String>,
    arunique: bool,
    arprop: Option<Vec<String>>,
    arnamespace: NamespaceID,
    arlimit: usize,
    ardir: Option<String>,
}

impl ActionApiData for ActionApiListAllredirectsData {}

impl Default for ActionApiListAllredirectsData {
    fn default() -> Self {
        Self {
            arcontinue: None,
            arfrom: None,
            arto: None,
            arprefix: None,
            arunique: false,
            arprop: None,
            arnamespace: 0,
            arlimit: 10,
            ardir: None,
        }
    }
}

impl ActionApiListAllredirectsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.arcontinue, "arcontinue", &mut params);
        Self::add_str(&self.arfrom, "arfrom", &mut params);
        Self::add_str(&self.arto, "arto", &mut params);
        Self::add_str(&self.arprefix, "arprefix", &mut params);
        Self::add_boolean(self.arunique, "arunique", &mut params);
        Self::add_vec(&self.arprop, "arprop", &mut params);
        params.insert("arnamespace".to_string(), self.arnamespace.to_string());
        params.insert("arlimit".to_string(), self.arlimit.to_string());
        Self::add_str(&self.ardir, "ardir", &mut params);
        params
    }
}

/// Builder for `list=allredirects` — lists all redirects.
#[derive(Debug, Clone)]
pub struct ActionApiListAllredirectsBuilder {
    pub(crate) data: ActionApiListAllredirectsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListAllredirectsBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListAllredirectsData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this title (`arfrom`).
    pub fn arfrom<S: AsRef<str>>(mut self, arfrom: S) -> Self {
        self.data.arfrom = Some(arfrom.as_ref().to_string());
        self
    }

    /// Stop listing at this title (`arto`).
    pub fn arto<S: AsRef<str>>(mut self, arto: S) -> Self {
        self.data.arto = Some(arto.as_ref().to_string());
        self
    }

    /// Prefix to search for (`arprefix`).
    pub fn arprefix<S: AsRef<str>>(mut self, arprefix: S) -> Self {
        self.data.arprefix = Some(arprefix.as_ref().to_string());
        self
    }

    /// Only show distinct redirect titles (`arunique`).
    pub fn arunique(mut self, arunique: bool) -> Self {
        self.data.arunique = arunique;
        self
    }

    /// Properties to return (`arprop`).
    pub fn arprop<S: Into<String> + Clone>(mut self, arprop: &[S]) -> Self {
        self.data.arprop = Some(arprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Namespace to enumerate (`arnamespace`).
    pub fn arnamespace(mut self, arnamespace: NamespaceID) -> Self {
        self.data.arnamespace = arnamespace;
        self
    }

    /// Maximum number of items to return (`arlimit`).
    pub fn arlimit(mut self, arlimit: usize) -> Self {
        self.data.arlimit = arlimit;
        self
    }

    /// Direction to list (`ardir`).
    pub fn ardir<S: AsRef<str>>(mut self, ardir: S) -> Self {
        self.data.ardir = Some(ardir.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListAllredirectsBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "allredirects".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListAllredirectsBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListAllredirectsBuilder {
        ActionApiListAllredirectsBuilder::new()
    }

    #[test]
    fn default_arnamespace_is_0() {
        let params = new_builder().data.params();
        assert_eq!(params["arnamespace"], "0");
    }

    #[test]
    fn default_arlimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["arlimit"], "10");
    }

    #[test]
    fn arprefix_set() {
        let params = new_builder().arprefix("Albert").data.params();
        assert_eq!(params["arprefix"], "Albert");
    }

    #[test]
    fn arnamespace_set() {
        let params = new_builder().arnamespace(4).data.params();
        assert_eq!(params["arnamespace"], "4");
    }

    #[test]
    fn arlimit_set() {
        let params = new_builder().arlimit(50).data.params();
        assert_eq!(params["arlimit"], "50");
    }

    #[test]
    fn arunique_flag() {
        let params = new_builder().arunique(true).data.params();
        assert_eq!(params["arunique"], "");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "allredirects");
    }
}
