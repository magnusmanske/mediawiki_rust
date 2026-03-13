use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=watchlistraw` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListWatchlistrawData {
    wrcontinue: Option<String>,
    wrnamespace: Option<Vec<NamespaceID>>,
    wrlimit: usize,
    wrprop: Option<Vec<String>>,
    wrshow: Option<Vec<String>>,
    wrowner: Option<String>,
    wrtoken: Option<String>,
    wrdir: Option<String>,
    wrfromtitle: Option<String>,
    wrtotitle: Option<String>,
}

impl ActionApiData for ActionApiListWatchlistrawData {}

impl Default for ActionApiListWatchlistrawData {
    fn default() -> Self {
        Self {
            wrcontinue: None,
            wrnamespace: None,
            wrlimit: 10,
            wrprop: None,
            wrshow: None,
            wrowner: None,
            wrtoken: None,
            wrdir: None,
            wrfromtitle: None,
            wrtotitle: None,
        }
    }
}

impl ActionApiListWatchlistrawData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.wrcontinue, "wrcontinue", &mut params);
        if let Some(ref ns) = self.wrnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("wrnamespace".to_string(), s.join("|"));
        }
        params.insert("wrlimit".to_string(), self.wrlimit.to_string());
        Self::add_vec(&self.wrprop, "wrprop", &mut params);
        Self::add_vec(&self.wrshow, "wrshow", &mut params);
        Self::add_str(&self.wrowner, "wrowner", &mut params);
        Self::add_str(&self.wrtoken, "wrtoken", &mut params);
        Self::add_str(&self.wrdir, "wrdir", &mut params);
        Self::add_str(&self.wrfromtitle, "wrfromtitle", &mut params);
        Self::add_str(&self.wrtotitle, "wrtotitle", &mut params);
        params
    }
}

/// Builder for `list=watchlistraw` — gets all pages on the current user's watchlist.
#[derive(Debug, Clone)]
pub struct ActionApiListWatchlistrawBuilder {
    pub(crate) data: ActionApiListWatchlistrawData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListWatchlistrawBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListWatchlistrawData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Filter to these namespaces (`wrnamespace`).
    pub fn wrnamespace(mut self, wrnamespace: &[NamespaceID]) -> Self {
        self.data.wrnamespace = Some(wrnamespace.to_vec());
        self
    }

    /// Maximum number of results to return (`wrlimit`).
    pub fn wrlimit(mut self, wrlimit: usize) -> Self {
        self.data.wrlimit = wrlimit;
        self
    }

    /// Properties to return (`wrprop`).
    pub fn wrprop<S: Into<String> + Clone>(mut self, wrprop: &[S]) -> Self {
        self.data.wrprop = Some(wrprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Filter entries (`wrshow`).
    pub fn wrshow<S: Into<String> + Clone>(mut self, wrshow: &[S]) -> Self {
        self.data.wrshow = Some(wrshow.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Username of another user whose watchlist to use (`wrowner`).
    pub fn wrowner<S: AsRef<str>>(mut self, wrowner: S) -> Self {
        self.data.wrowner = Some(wrowner.as_ref().to_string());
        self
    }

    /// Access token for another user's watchlist (`wrtoken`).
    pub fn wrtoken<S: AsRef<str>>(mut self, wrtoken: S) -> Self {
        self.data.wrtoken = Some(wrtoken.as_ref().to_string());
        self
    }

    /// Direction to enumerate (`wrdir`).
    pub fn wrdir<S: AsRef<str>>(mut self, wrdir: S) -> Self {
        self.data.wrdir = Some(wrdir.as_ref().to_string());
        self
    }

    /// Title to start listing from (`wrfromtitle`).
    pub fn wrfromtitle<S: AsRef<str>>(mut self, wrfromtitle: S) -> Self {
        self.data.wrfromtitle = Some(wrfromtitle.as_ref().to_string());
        self
    }

    /// Title to stop listing at (`wrtotitle`).
    pub fn wrtotitle<S: AsRef<str>>(mut self, wrtotitle: S) -> Self {
        self.data.wrtotitle = Some(wrtotitle.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListWatchlistrawBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "watchlistraw".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListWatchlistrawBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListWatchlistrawBuilder {
        ActionApiListWatchlistrawBuilder::new()
    }

    #[test]
    fn default_wrlimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["wrlimit"], "10");
    }

    #[test]
    fn default_wrnamespace_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("wrnamespace"));
    }

    #[test]
    fn wrnamespace_set() {
        let params = new_builder().wrnamespace(&[0, 4]).data.params();
        assert_eq!(params["wrnamespace"], "0|4");
    }

    #[test]
    fn wrlimit_set() {
        let params = new_builder().wrlimit(50).data.params();
        assert_eq!(params["wrlimit"], "50");
    }

    #[test]
    fn wrprop_set() {
        let params = new_builder().wrprop(&["changed"]).data.params();
        assert_eq!(params["wrprop"], "changed");
    }

    #[test]
    fn wrfromtitle_set() {
        let params = new_builder().wrfromtitle("Albert").data.params();
        assert_eq!(params["wrfromtitle"], "Albert");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "watchlistraw");
    }
}
