use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=allusers` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListAllusersData {
    aufrom: Option<String>,
    auto: Option<String>,
    auprefix: Option<String>,
    audir: Option<String>,
    augroup: Option<Vec<String>>,
    auexcludegroup: Option<Vec<String>>,
    aurights: Option<Vec<String>>,
    auprop: Option<Vec<String>>,
    aulimit: usize,
    auwitheditsonly: bool,
    auactiveusers: bool,
}

impl ActionApiData for ActionApiListAllusersData {}

impl Default for ActionApiListAllusersData {
    fn default() -> Self {
        Self {
            aufrom: None,
            auto: None,
            auprefix: None,
            audir: None,
            augroup: None,
            auexcludegroup: None,
            aurights: None,
            auprop: None,
            aulimit: 10,
            auwitheditsonly: false,
            auactiveusers: false,
        }
    }
}

impl ActionApiListAllusersData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.aufrom, "aufrom", &mut params);
        Self::add_str(&self.auto, "auto", &mut params);
        Self::add_str(&self.auprefix, "auprefix", &mut params);
        Self::add_str(&self.audir, "audir", &mut params);
        Self::add_vec(&self.augroup, "augroup", &mut params);
        Self::add_vec(&self.auexcludegroup, "auexcludegroup", &mut params);
        Self::add_vec(&self.aurights, "aurights", &mut params);
        Self::add_vec(&self.auprop, "auprop", &mut params);
        params.insert("aulimit".to_string(), self.aulimit.to_string());
        Self::add_boolean(self.auwitheditsonly, "auwitheditsonly", &mut params);
        Self::add_boolean(self.auactiveusers, "auactiveusers", &mut params);
        params
    }
}

/// Builder for `list=allusers` — enumerates all registered users.
#[derive(Debug, Clone)]
pub struct ActionApiListAllusersBuilder {
    pub(crate) data: ActionApiListAllusersData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListAllusersBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListAllusersData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this username (`aufrom`).
    pub fn aufrom<S: AsRef<str>>(mut self, aufrom: S) -> Self {
        self.data.aufrom = Some(aufrom.as_ref().to_string());
        self
    }

    /// Stop listing at this username (`auto`).
    pub fn auto<S: AsRef<str>>(mut self, auto: S) -> Self {
        self.data.auto = Some(auto.as_ref().to_string());
        self
    }

    /// List only users with this prefix (`auprefix`).
    pub fn auprefix<S: AsRef<str>>(mut self, auprefix: S) -> Self {
        self.data.auprefix = Some(auprefix.as_ref().to_string());
        self
    }

    /// Direction to sort: `"ascending"` or `"descending"` (`audir`).
    pub fn audir<S: AsRef<str>>(mut self, audir: S) -> Self {
        self.data.audir = Some(audir.as_ref().to_string());
        self
    }

    /// Include only users in these groups (`augroup`).
    pub fn augroup<S: Into<String> + Clone>(mut self, augroup: &[S]) -> Self {
        self.data.augroup = Some(augroup.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Exclude users in these groups (`auexcludegroup`).
    pub fn auexcludegroup<S: Into<String> + Clone>(mut self, auexcludegroup: &[S]) -> Self {
        self.data.auexcludegroup =
            Some(auexcludegroup.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Include only users with these rights (`aurights`).
    pub fn aurights<S: Into<String> + Clone>(mut self, aurights: &[S]) -> Self {
        self.data.aurights = Some(aurights.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Which user information to include (`auprop`).
    pub fn auprop<S: Into<String> + Clone>(mut self, auprop: &[S]) -> Self {
        self.data.auprop = Some(auprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of users to return (`aulimit`).
    pub fn aulimit(mut self, aulimit: usize) -> Self {
        self.data.aulimit = aulimit;
        self
    }

    /// Only list users with edits (`auwitheditsonly`).
    pub fn auwitheditsonly(mut self, auwitheditsonly: bool) -> Self {
        self.data.auwitheditsonly = auwitheditsonly;
        self
    }

    /// Only list active users (`auactiveusers`).
    pub fn auactiveusers(mut self, auactiveusers: bool) -> Self {
        self.data.auactiveusers = auactiveusers;
        self
    }
}

impl ActionApiRunnable for ActionApiListAllusersBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "allusers".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListAllusersBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListAllusersBuilder {
        ActionApiListAllusersBuilder::new()
    }

    #[test]
    fn default_aulimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["aulimit"], "10");
    }

    #[test]
    fn default_aufrom_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("aufrom"));
    }

    #[test]
    fn aufrom_set() {
        let params = new_builder().aufrom("Admin").data.params();
        assert_eq!(params["aufrom"], "Admin");
    }

    #[test]
    fn auprefix_set() {
        let params = new_builder().auprefix("Bot").data.params();
        assert_eq!(params["auprefix"], "Bot");
    }

    #[test]
    fn augroup_set() {
        let params = new_builder().augroup(&["sysop", "bureaucrat"]).data.params();
        assert_eq!(params["augroup"], "sysop|bureaucrat");
    }

    #[test]
    fn aulimit_set() {
        let params = new_builder().aulimit(50).data.params();
        assert_eq!(params["aulimit"], "50");
    }

    #[test]
    fn auwitheditsonly_flag() {
        let params = new_builder().auwitheditsonly(true).data.params();
        assert_eq!(params["auwitheditsonly"], "");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "allusers");
    }
}
