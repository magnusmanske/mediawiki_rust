use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=recentchanges` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListRecentchangesData {
    rcstart: Option<String>,
    rcend: Option<String>,
    rcdir: Option<String>,
    rcnamespace: Option<Vec<NamespaceID>>,
    rcuser: Option<String>,
    rcexcludeuser: Option<String>,
    rctag: Option<String>,
    rcprop: Option<Vec<String>>,
    rcshow: Option<Vec<String>>,
    rclimit: usize,
    rctype: Option<Vec<String>>,
    rctoponly: bool,
    rctitle: Option<String>,
    rccontinue: Option<String>,
}

impl ActionApiData for ActionApiListRecentchangesData {}

impl Default for ActionApiListRecentchangesData {
    fn default() -> Self {
        Self {
            rcstart: None,
            rcend: None,
            rcdir: None,
            rcnamespace: None,
            rcuser: None,
            rcexcludeuser: None,
            rctag: None,
            rcprop: None,
            rcshow: None,
            rclimit: 10,
            rctype: None,
            rctoponly: false,
            rctitle: None,
            rccontinue: None,
        }
    }
}

impl ActionApiListRecentchangesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.rcstart, "rcstart", &mut params);
        Self::add_str(&self.rcend, "rcend", &mut params);
        Self::add_str(&self.rcdir, "rcdir", &mut params);
        if let Some(ns) = &self.rcnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("rcnamespace".to_string(), s.join("|"));
        }
        Self::add_str(&self.rcuser, "rcuser", &mut params);
        Self::add_str(&self.rcexcludeuser, "rcexcludeuser", &mut params);
        Self::add_str(&self.rctag, "rctag", &mut params);
        Self::add_vec(&self.rcprop, "rcprop", &mut params);
        Self::add_vec(&self.rcshow, "rcshow", &mut params);
        params.insert("rclimit".to_string(), self.rclimit.to_string());
        Self::add_vec(&self.rctype, "rctype", &mut params);
        Self::add_boolean(self.rctoponly, "rctoponly", &mut params);
        Self::add_str(&self.rctitle, "rctitle", &mut params);
        Self::add_str(&self.rccontinue, "rccontinue", &mut params);
        params
    }
}

/// Builder for the `list=recentchanges` API module; supports pagination via `ActionApiContinuable`.
#[derive(Debug, Clone)]
pub struct ActionApiListRecentchangesBuilder {
    pub(crate) data: ActionApiListRecentchangesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListRecentchangesBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            data: ActionApiListRecentchangesData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Timestamp to start enumerating recent changes from (`rcstart`).
    pub fn rcstart<S: AsRef<str>>(mut self, rcstart: S) -> Self {
        self.data.rcstart = Some(rcstart.as_ref().to_string());
        self
    }

    /// Timestamp to stop enumerating recent changes at (`rcend`).
    pub fn rcend<S: AsRef<str>>(mut self, rcend: S) -> Self {
        self.data.rcend = Some(rcend.as_ref().to_string());
        self
    }

    /// Enumeration direction (`newer` or `older`) (`rcdir`).
    pub fn rcdir<S: AsRef<str>>(mut self, rcdir: S) -> Self {
        self.data.rcdir = Some(rcdir.as_ref().to_string());
        self
    }

    /// Filter changes to these namespaces (`rcnamespace`).
    pub fn rcnamespace(mut self, rcnamespace: &[NamespaceID]) -> Self {
        self.data.rcnamespace = Some(rcnamespace.to_vec());
        self
    }

    /// Filter changes to those made by this user (`rcuser`).
    pub fn rcuser<S: AsRef<str>>(mut self, rcuser: S) -> Self {
        self.data.rcuser = Some(rcuser.as_ref().to_string());
        self
    }

    /// Exclude changes made by this user (`rcexcludeuser`).
    pub fn rcexcludeuser<S: AsRef<str>>(mut self, rcexcludeuser: S) -> Self {
        self.data.rcexcludeuser = Some(rcexcludeuser.as_ref().to_string());
        self
    }

    /// Filter changes to those tagged with this tag (`rctag`).
    pub fn rctag<S: AsRef<str>>(mut self, rctag: S) -> Self {
        self.data.rctag = Some(rctag.as_ref().to_string());
        self
    }

    /// Properties to retrieve for each recent change (`rcprop`).
    pub fn rcprop<S: Into<String> + Clone>(mut self, rcprop: &[S]) -> Self {
        self.data.rcprop = Some(rcprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Show only changes matching these criteria (e.g. `minor`, `bot`) (`rcshow`).
    pub fn rcshow<S: Into<String> + Clone>(mut self, rcshow: &[S]) -> Self {
        self.data.rcshow = Some(rcshow.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of recent changes to return (`rclimit`).
    pub fn rclimit(mut self, rclimit: usize) -> Self {
        self.data.rclimit = rclimit;
        self
    }

    /// Filter to these change types (e.g. `edit`, `new`, `log`) (`rctype`).
    pub fn rctype<S: Into<String> + Clone>(mut self, rctype: &[S]) -> Self {
        self.data.rctype = Some(rctype.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// When set, only list the most recent change for each page (`rctoponly`).
    pub fn rctoponly(mut self, rctoponly: bool) -> Self {
        self.data.rctoponly = rctoponly;
        self
    }

    /// Filter changes to this specific page title (`rctitle`).
    pub fn rctitle<S: AsRef<str>>(mut self, rctitle: S) -> Self {
        self.data.rctitle = Some(rctitle.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListRecentchangesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "recentchanges".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListRecentchangesBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListRecentchangesBuilder {
        ActionApiListRecentchangesBuilder::new()
    }

    #[test]
    fn default_rclimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["rclimit"], "10");
    }

    #[test]
    fn default_rcuser_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("rcuser"));
    }

    #[test]
    fn rcnamespace_set() {
        let params = new_builder().rcnamespace(&[0]).data.params();
        assert_eq!(params["rcnamespace"], "0");
    }

    #[test]
    fn rcuser_set() {
        let params = new_builder().rcuser("ExampleUser").data.params();
        assert_eq!(params["rcuser"], "ExampleUser");
    }

    #[test]
    fn rctype_set() {
        let params = new_builder().rctype(&["edit", "new"]).data.params();
        assert_eq!(params["rctype"], "edit|new");
    }

    #[test]
    fn rcprop_set() {
        let params = new_builder().rcprop(&["ids", "title", "timestamp"]).data.params();
        assert_eq!(params["rcprop"], "ids|title|timestamp");
    }

    #[test]
    fn rclimit_set() {
        let params = new_builder().rclimit(50).data.params();
        assert_eq!(params["rclimit"], "50");
    }

    #[test]
    fn rctoponly_set() {
        let params = new_builder().rctoponly(true).data.params();
        assert!(params.contains_key("rctoponly"));
    }

    #[test]
    fn rcdir_newer() {
        let params = new_builder().rcdir("newer").data.params();
        assert_eq!(params["rcdir"], "newer");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "recentchanges");
    }

    #[tokio::test]
    async fn test_recentchanges() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::recentchanges()
            .rcnamespace(&[0])
            .rclimit(5)
            .rctype(&["edit"])
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["recentchanges"].is_array());
    }
}
