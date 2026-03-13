use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=tags` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListTagsData {
    tgcontinue: Option<String>,
    tglimit: usize,
    tgprop: Option<Vec<String>>,
}

impl ActionApiData for ActionApiListTagsData {}

impl Default for ActionApiListTagsData {
    fn default() -> Self {
        Self {
            tgcontinue: None,
            tglimit: 10,
            tgprop: None,
        }
    }
}

impl ActionApiListTagsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.tgcontinue, "tgcontinue", &mut params);
        params.insert("tglimit".to_string(), self.tglimit.to_string());
        Self::add_vec(&self.tgprop, "tgprop", &mut params);
        params
    }
}

/// Builder for `list=tags` — lists change tags.
#[derive(Debug, Clone)]
pub struct ActionApiListTagsBuilder {
    pub(crate) data: ActionApiListTagsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListTagsBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListTagsData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Maximum number of tags to return (`tglimit`).
    pub fn tglimit(mut self, tglimit: usize) -> Self {
        self.data.tglimit = tglimit;
        self
    }

    /// Properties to return for each tag (`tgprop`).
    pub fn tgprop<S: Into<String> + Clone>(mut self, tgprop: &[S]) -> Self {
        self.data.tgprop = Some(tgprop.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiRunnable for ActionApiListTagsBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "tags".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListTagsBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListTagsBuilder {
        ActionApiListTagsBuilder::new()
    }

    #[test]
    fn default_tglimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["tglimit"], "10");
    }

    #[test]
    fn tglimit_set() {
        let params = new_builder().tglimit(50).data.params();
        assert_eq!(params["tglimit"], "50");
    }

    #[test]
    fn tgprop_set() {
        let params = new_builder()
            .tgprop(&["name", "displayname", "hitcount"])
            .data
            .params();
        assert_eq!(params["tgprop"], "name|displayname|hitcount");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "tags");
    }
}
