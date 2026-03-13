use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=protectedtitles` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListProtectedtitlesData {
    ptnamespace: Option<Vec<NamespaceID>>,
    ptlevel: Option<Vec<String>>,
    ptlimit: usize,
    ptdir: Option<String>,
    ptstart: Option<String>,
    ptend: Option<String>,
    ptprop: Option<Vec<String>>,
    ptcontinue: Option<String>,
}

impl ActionApiData for ActionApiListProtectedtitlesData {}

impl Default for ActionApiListProtectedtitlesData {
    fn default() -> Self {
        Self {
            ptnamespace: None,
            ptlevel: None,
            ptlimit: 10,
            ptdir: None,
            ptstart: None,
            ptend: None,
            ptprop: None,
            ptcontinue: None,
        }
    }
}

impl ActionApiListProtectedtitlesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(ref ns) = self.ptnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("ptnamespace".to_string(), s.join("|"));
        }
        Self::add_vec(&self.ptlevel, "ptlevel", &mut params);
        params.insert("ptlimit".to_string(), self.ptlimit.to_string());
        Self::add_str(&self.ptdir, "ptdir", &mut params);
        Self::add_str(&self.ptstart, "ptstart", &mut params);
        Self::add_str(&self.ptend, "ptend", &mut params);
        Self::add_vec(&self.ptprop, "ptprop", &mut params);
        Self::add_str(&self.ptcontinue, "ptcontinue", &mut params);
        params
    }
}

/// Builder for `list=protectedtitles` — lists all titles protected from creation.
#[derive(Debug, Clone)]
pub struct ActionApiListProtectedtitlesBuilder {
    pub(crate) data: ActionApiListProtectedtitlesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListProtectedtitlesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListProtectedtitlesData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Namespaces to enumerate (`ptnamespace`).
    pub fn ptnamespace(mut self, ptnamespace: &[NamespaceID]) -> Self {
        self.data.ptnamespace = Some(ptnamespace.to_vec());
        self
    }

    /// Filter protection levels (`ptlevel`).
    pub fn ptlevel<S: Into<String> + Clone>(mut self, ptlevel: &[S]) -> Self {
        self.data.ptlevel = Some(ptlevel.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of titles to return (`ptlimit`).
    pub fn ptlimit(mut self, ptlimit: usize) -> Self {
        self.data.ptlimit = ptlimit;
        self
    }

    /// Direction to enumerate (`ptdir`).
    pub fn ptdir<S: AsRef<str>>(mut self, ptdir: S) -> Self {
        self.data.ptdir = Some(ptdir.as_ref().to_string());
        self
    }

    /// Start listing from this timestamp (`ptstart`).
    pub fn ptstart<S: AsRef<str>>(mut self, ptstart: S) -> Self {
        self.data.ptstart = Some(ptstart.as_ref().to_string());
        self
    }

    /// Stop listing at this timestamp (`ptend`).
    pub fn ptend<S: AsRef<str>>(mut self, ptend: S) -> Self {
        self.data.ptend = Some(ptend.as_ref().to_string());
        self
    }

    /// Properties to return (`ptprop`).
    pub fn ptprop<S: Into<String> + Clone>(mut self, ptprop: &[S]) -> Self {
        self.data.ptprop = Some(ptprop.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiRunnable for ActionApiListProtectedtitlesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "protectedtitles".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListProtectedtitlesBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListProtectedtitlesBuilder {
        ActionApiListProtectedtitlesBuilder::new()
    }

    #[test]
    fn default_ptlimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["ptlimit"], "10");
    }

    #[test]
    fn default_ptnamespace_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("ptnamespace"));
    }

    #[test]
    fn ptnamespace_set() {
        let params = new_builder().ptnamespace(&[0, 4]).data.params();
        assert_eq!(params["ptnamespace"], "0|4");
    }

    #[test]
    fn ptlevel_set() {
        let params = new_builder().ptlevel(&["sysop"]).data.params();
        assert_eq!(params["ptlevel"], "sysop");
    }

    #[test]
    fn ptlimit_set() {
        let params = new_builder().ptlimit(50).data.params();
        assert_eq!(params["ptlimit"], "50");
    }

    #[test]
    fn ptprop_set() {
        let params = new_builder().ptprop(&["timestamp", "expiry"]).data.params();
        assert_eq!(params["ptprop"], "timestamp|expiry");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "protectedtitles");
    }
}
