use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=pageswithprop` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiListPageswithpropData {
    pwppropname: Option<String>,
    pwpprop: Option<Vec<String>>,
    pwpdir: Option<String>,
    pwplimit: usize,
    pwpcontinue: Option<String>,
}

impl ActionApiData for ActionApiListPageswithpropData {}

impl ActionApiListPageswithpropData {
    pub(crate) fn new() -> Self {
        Self {
            pwppropname: None,
            pwpprop: None,
            pwpdir: None,
            pwplimit: 10,
            pwpcontinue: None,
        }
    }

    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.pwppropname, "pwppropname", &mut params);
        Self::add_vec(&self.pwpprop, "pwpprop", &mut params);
        Self::add_str(&self.pwpdir, "pwpdir", &mut params);
        params.insert("pwplimit".to_string(), self.pwplimit.to_string());
        Self::add_str(&self.pwpcontinue, "pwpcontinue", &mut params);
        params
    }
}

/// Builder for `list=pageswithprop` — lists all pages using a certain page property.
#[derive(Debug, Clone)]
pub struct ActionApiListPageswithpropBuilder {
    pub(crate) data: ActionApiListPageswithpropData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListPageswithpropBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListPageswithpropData::new(),
            continue_params: HashMap::new(),
        }
    }

    /// Page property for which to enumerate pages (`pwppropname`).
    pub fn pwppropname<S: AsRef<str>>(mut self, pwppropname: S) -> Self {
        self.data.pwppropname = Some(pwppropname.as_ref().to_string());
        self
    }

    /// Properties to return (`pwpprop`).
    pub fn pwpprop<S: Into<String> + Clone>(mut self, pwpprop: &[S]) -> Self {
        self.data.pwpprop = Some(pwpprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Direction to sort (`pwpdir`).
    pub fn pwpdir<S: AsRef<str>>(mut self, pwpdir: S) -> Self {
        self.data.pwpdir = Some(pwpdir.as_ref().to_string());
        self
    }

    /// Maximum number of pages to return (`pwplimit`).
    pub fn pwplimit(mut self, pwplimit: usize) -> Self {
        self.data.pwplimit = pwplimit;
        self
    }
}

impl ActionApiRunnable for ActionApiListPageswithpropBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "pageswithprop".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListPageswithpropBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListPageswithpropBuilder {
        ActionApiListPageswithpropBuilder::new()
    }

    #[test]
    fn default_pwplimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["pwplimit"], "10");
    }

    #[test]
    fn default_pwppropname_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("pwppropname"));
    }

    #[test]
    fn pwppropname_set() {
        let params = new_builder().pwppropname("wikibase_item").data.params();
        assert_eq!(params["pwppropname"], "wikibase_item");
    }

    #[test]
    fn pwpprop_set() {
        let params = new_builder().pwpprop(&["ids", "title", "value"]).data.params();
        assert_eq!(params["pwpprop"], "ids|title|value");
    }

    #[test]
    fn pwpdir_set() {
        let params = new_builder().pwpdir("descending").data.params();
        assert_eq!(params["pwpdir"], "descending");
    }

    #[test]
    fn pwplimit_set() {
        let params = new_builder().pwplimit(50).data.params();
        assert_eq!(params["pwplimit"], "50");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "pageswithprop");
    }
}
