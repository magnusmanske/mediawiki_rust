use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `meta=siteinfo` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMetaSiteinfoData {
    siprop: Option<Vec<String>>,
    sifilteriw: Option<String>,
    sishowalldb: bool,
    sinumberingroup: bool,
    siinlanguagecode: Option<String>,
}

impl ActionApiData for ActionApiMetaSiteinfoData {}

impl ActionApiMetaSiteinfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.siprop, "siprop", &mut params);
        Self::add_str(&self.sifilteriw, "sifilteriw", &mut params);
        Self::add_boolean(self.sishowalldb, "sishowalldb", &mut params);
        Self::add_boolean(self.sinumberingroup, "sinumberingroup", &mut params);
        Self::add_str(&self.siinlanguagecode, "siinlanguagecode", &mut params);
        params
    }
}

/// Builder for `meta=siteinfo` — returns general site information.
#[derive(Debug, Clone)]
pub struct ActionApiMetaSiteinfoBuilder {
    pub(crate) data: ActionApiMetaSiteinfoData,
}

impl ActionApiMetaSiteinfoBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiMetaSiteinfoData::default(),
        }
    }

    /// Which information to get (`siprop`).
    pub fn siprop<S: Into<String> + Clone>(mut self, siprop: &[S]) -> Self {
        self.data.siprop = Some(siprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Filter interwiki prefixes (`sifilteriw`).
    pub fn sifilteriw<S: AsRef<str>>(mut self, sifilteriw: S) -> Self {
        self.data.sifilteriw = Some(sifilteriw.as_ref().to_string());
        self
    }

    /// Show all databases in the server lag list (`sishowalldb`).
    pub fn sishowalldb(mut self, sishowalldb: bool) -> Self {
        self.data.sishowalldb = sishowalldb;
        self
    }

    /// List the number of users in user groups (`sinumberingroup`).
    pub fn sinumberingroup(mut self, sinumberingroup: bool) -> Self {
        self.data.sinumberingroup = sinumberingroup;
        self
    }

    /// Return localised language names in this language (`siinlanguagecode`).
    pub fn siinlanguagecode<S: AsRef<str>>(mut self, siinlanguagecode: S) -> Self {
        self.data.siinlanguagecode = Some(siinlanguagecode.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiMetaSiteinfoBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("meta".to_string(), "siteinfo".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiMetaSiteinfoBuilder {
        ActionApiMetaSiteinfoBuilder::new()
    }

    #[test]
    fn default_siprop_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("siprop"));
    }

    #[test]
    fn siprop_set() {
        let params = new_builder().siprop(&["general", "namespaces"]).data.params();
        assert_eq!(params["siprop"], "general|namespaces");
    }

    #[test]
    fn sifilteriw_set() {
        let params = new_builder().sifilteriw("local").data.params();
        assert_eq!(params["sifilteriw"], "local");
    }

    #[test]
    fn sishowalldb_flag() {
        let params = new_builder().sishowalldb(true).data.params();
        assert_eq!(params["sishowalldb"], "");
    }

    #[test]
    fn sinumberingroup_flag() {
        let params = new_builder().sinumberingroup(true).data.params();
        assert_eq!(params["sinumberingroup"], "");
    }

    #[test]
    fn siinlanguagecode_set() {
        let params = new_builder().siinlanguagecode("de").data.params();
        assert_eq!(params["siinlanguagecode"], "de");
    }

    #[test]
    fn runnable_params_contain_action_meta() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["meta"], "siteinfo");
    }
}
