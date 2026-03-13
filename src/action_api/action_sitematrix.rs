use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `action=sitematrix` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiSitematrixData {
    smtype: Option<Vec<String>>,
    smlangprop: Option<Vec<String>>,
    smsiteprop: Option<Vec<String>>,
    smlimit: usize,
    smcontinue: Option<String>,
}

impl ActionApiData for ActionApiSitematrixData {}

impl Default for ActionApiSitematrixData {
    fn default() -> Self {
        Self {
            smtype: None,
            smlangprop: None,
            smsiteprop: None,
            smlimit: 500,
            smcontinue: None,
        }
    }
}

impl ActionApiSitematrixData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.smtype, "smtype", &mut params);
        Self::add_vec(&self.smlangprop, "smlangprop", &mut params);
        Self::add_vec(&self.smsiteprop, "smsiteprop", &mut params);
        params.insert("smlimit".to_string(), self.smlimit.to_string());
        Self::add_str(&self.smcontinue, "smcontinue", &mut params);
        params
    }
}

/// Builder for `action=sitematrix` — returns the Wikimedia sitematrix.
#[derive(Debug, Clone)]
pub struct ActionApiSitematrixBuilder {
    pub(crate) data: ActionApiSitematrixData,
}

impl ActionApiSitematrixBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiSitematrixData::default(),
        }
    }

    /// Filter by site type (`smtype`).
    pub fn smtype<S: Into<String> + Clone>(mut self, smtype: &[S]) -> Self {
        self.data.smtype = Some(smtype.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Properties to return for each language (`smlangprop`).
    pub fn smlangprop<S: Into<String> + Clone>(mut self, smlangprop: &[S]) -> Self {
        self.data.smlangprop = Some(smlangprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Properties to return for each site (`smsiteprop`).
    pub fn smsiteprop<S: Into<String> + Clone>(mut self, smsiteprop: &[S]) -> Self {
        self.data.smsiteprop = Some(smsiteprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of results to return (`smlimit`).
    pub fn smlimit(mut self, smlimit: usize) -> Self {
        self.data.smlimit = smlimit;
        self
    }
}

impl ActionApiRunnable for ActionApiSitematrixBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "sitematrix".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiSitematrixBuilder {
        ActionApiSitematrixBuilder::new()
    }

    #[test]
    fn default_smlimit_is_500() {
        let params = new_builder().data.params();
        assert_eq!(params["smlimit"], "500");
    }

    #[test]
    fn default_smtype_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("smtype"));
    }

    #[test]
    fn smtype_set() {
        let params = new_builder().smtype(&["special", "language"]).data.params();
        assert_eq!(params["smtype"], "special|language");
    }

    #[test]
    fn smlangprop_set() {
        let params = new_builder().smlangprop(&["code", "name"]).data.params();
        assert_eq!(params["smlangprop"], "code|name");
    }

    #[test]
    fn smsiteprop_set() {
        let params = new_builder().smsiteprop(&["url", "dbname"]).data.params();
        assert_eq!(params["smsiteprop"], "url|dbname");
    }

    #[test]
    fn smlimit_set() {
        let params = new_builder().smlimit(100).data.params();
        assert_eq!(params["smlimit"], "100");
    }

    #[test]
    fn runnable_params_contain_action() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "sitematrix");
    }
}
