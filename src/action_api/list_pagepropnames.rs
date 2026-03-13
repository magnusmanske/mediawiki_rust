use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=pagepropnames` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListPagepropnamesData {
    ppnlimit: usize,
    ppncontinue: Option<String>,
}

impl ActionApiData for ActionApiListPagepropnamesData {}

impl Default for ActionApiListPagepropnamesData {
    fn default() -> Self {
        Self {
            ppnlimit: 10,
            ppncontinue: None,
        }
    }
}

impl ActionApiListPagepropnamesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("ppnlimit".to_string(), self.ppnlimit.to_string());
        Self::add_str(&self.ppncontinue, "ppncontinue", &mut params);
        params
    }
}

/// Builder for `list=pagepropnames` — lists all page property names in use on the wiki.
#[derive(Debug, Clone)]
pub struct ActionApiListPagepropnamesBuilder {
    pub(crate) data: ActionApiListPagepropnamesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListPagepropnamesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListPagepropnamesData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Maximum number of property names to return (`ppnlimit`).
    pub fn ppnlimit(mut self, ppnlimit: usize) -> Self {
        self.data.ppnlimit = ppnlimit;
        self
    }
}

impl ActionApiRunnable for ActionApiListPagepropnamesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "pagepropnames".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListPagepropnamesBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListPagepropnamesBuilder {
        ActionApiListPagepropnamesBuilder::new()
    }

    #[test]
    fn default_ppnlimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["ppnlimit"], "10");
    }

    #[test]
    fn ppnlimit_set() {
        let params = new_builder().ppnlimit(50).data.params();
        assert_eq!(params["ppnlimit"], "50");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "pagepropnames");
    }
}
