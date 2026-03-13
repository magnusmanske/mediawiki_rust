use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=watchlist` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListWatchlistData {
    wlallrev: bool,
    wlstart: Option<String>,
    wlend: Option<String>,
    wlnamespace: Option<Vec<NamespaceID>>,
    wluser: Option<String>,
    wlexcludeuser: Option<String>,
    wldir: Option<String>,
    wllimit: usize,
    wlprop: Option<Vec<String>>,
    wlshow: Option<Vec<String>>,
    wltype: Option<Vec<String>>,
    wlowner: Option<String>,
    wltoken: Option<String>,
    wlcontinue: Option<String>,
}

impl ActionApiData for ActionApiListWatchlistData {}

impl Default for ActionApiListWatchlistData {
    fn default() -> Self {
        Self {
            wlallrev: false,
            wlstart: None,
            wlend: None,
            wlnamespace: None,
            wluser: None,
            wlexcludeuser: None,
            wldir: None,
            wllimit: 10,
            wlprop: None,
            wlshow: None,
            wltype: None,
            wlowner: None,
            wltoken: None,
            wlcontinue: None,
        }
    }
}

impl ActionApiListWatchlistData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_boolean(self.wlallrev, "wlallrev", &mut params);
        Self::add_str(&self.wlstart, "wlstart", &mut params);
        Self::add_str(&self.wlend, "wlend", &mut params);
        if let Some(ref ns) = self.wlnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("wlnamespace".to_string(), s.join("|"));
        }
        Self::add_str(&self.wluser, "wluser", &mut params);
        Self::add_str(&self.wlexcludeuser, "wlexcludeuser", &mut params);
        Self::add_str(&self.wldir, "wldir", &mut params);
        params.insert("wllimit".to_string(), self.wllimit.to_string());
        Self::add_vec(&self.wlprop, "wlprop", &mut params);
        Self::add_vec(&self.wlshow, "wlshow", &mut params);
        Self::add_vec(&self.wltype, "wltype", &mut params);
        Self::add_str(&self.wlowner, "wlowner", &mut params);
        Self::add_str(&self.wltoken, "wltoken", &mut params);
        Self::add_str(&self.wlcontinue, "wlcontinue", &mut params);
        params
    }
}

/// Builder for `list=watchlist` — gets recent changes to pages in the current user's watchlist.
#[derive(Debug, Clone)]
pub struct ActionApiListWatchlistBuilder {
    pub(crate) data: ActionApiListWatchlistData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListWatchlistBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListWatchlistData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Include all changes for watched pages, not just the latest one (`wlallrev`).
    pub fn wlallrev(mut self, wlallrev: bool) -> Self {
        self.data.wlallrev = wlallrev;
        self
    }

    /// Start listing from this timestamp (`wlstart`).
    pub fn wlstart<S: AsRef<str>>(mut self, wlstart: S) -> Self {
        self.data.wlstart = Some(wlstart.as_ref().to_string());
        self
    }

    /// Stop listing at this timestamp (`wlend`).
    pub fn wlend<S: AsRef<str>>(mut self, wlend: S) -> Self {
        self.data.wlend = Some(wlend.as_ref().to_string());
        self
    }

    /// Filter to these namespaces (`wlnamespace`).
    pub fn wlnamespace(mut self, wlnamespace: &[NamespaceID]) -> Self {
        self.data.wlnamespace = Some(wlnamespace.to_vec());
        self
    }

    /// Only list changes by this user (`wluser`).
    pub fn wluser<S: AsRef<str>>(mut self, wluser: S) -> Self {
        self.data.wluser = Some(wluser.as_ref().to_string());
        self
    }

    /// Exclude changes by this user (`wlexcludeuser`).
    pub fn wlexcludeuser<S: AsRef<str>>(mut self, wlexcludeuser: S) -> Self {
        self.data.wlexcludeuser = Some(wlexcludeuser.as_ref().to_string());
        self
    }

    /// Direction to enumerate (`wldir`).
    pub fn wldir<S: AsRef<str>>(mut self, wldir: S) -> Self {
        self.data.wldir = Some(wldir.as_ref().to_string());
        self
    }

    /// Maximum number of results to return (`wllimit`).
    pub fn wllimit(mut self, wllimit: usize) -> Self {
        self.data.wllimit = wllimit;
        self
    }

    /// Properties to return (`wlprop`).
    pub fn wlprop<S: Into<String> + Clone>(mut self, wlprop: &[S]) -> Self {
        self.data.wlprop = Some(wlprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Filter watchlist entries (`wlshow`).
    pub fn wlshow<S: Into<String> + Clone>(mut self, wlshow: &[S]) -> Self {
        self.data.wlshow = Some(wlshow.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Types of changes to return (`wltype`).
    pub fn wltype<S: Into<String> + Clone>(mut self, wltype: &[S]) -> Self {
        self.data.wltype = Some(wltype.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Username of another user whose watchlist to use (`wlowner`).
    pub fn wlowner<S: AsRef<str>>(mut self, wlowner: S) -> Self {
        self.data.wlowner = Some(wlowner.as_ref().to_string());
        self
    }

    /// Access token for another user's watchlist (`wltoken`).
    pub fn wltoken<S: AsRef<str>>(mut self, wltoken: S) -> Self {
        self.data.wltoken = Some(wltoken.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListWatchlistBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "watchlist".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListWatchlistBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListWatchlistBuilder {
        ActionApiListWatchlistBuilder::new()
    }

    #[test]
    fn default_wllimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["wllimit"], "10");
    }

    #[test]
    fn default_wlallrev_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("wlallrev"));
    }

    #[test]
    fn wlallrev_flag() {
        let params = new_builder().wlallrev(true).data.params();
        assert_eq!(params["wlallrev"], "");
    }

    #[test]
    fn wlnamespace_set() {
        let params = new_builder().wlnamespace(&[0, 4]).data.params();
        assert_eq!(params["wlnamespace"], "0|4");
    }

    #[test]
    fn wllimit_set() {
        let params = new_builder().wllimit(50).data.params();
        assert_eq!(params["wllimit"], "50");
    }

    #[test]
    fn wlprop_set() {
        let params = new_builder().wlprop(&["ids", "title", "flags"]).data.params();
        assert_eq!(params["wlprop"], "ids|title|flags");
    }

    #[test]
    fn wluser_set() {
        let params = new_builder().wluser("ExampleUser").data.params();
        assert_eq!(params["wluser"], "ExampleUser");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "watchlist");
    }
}
