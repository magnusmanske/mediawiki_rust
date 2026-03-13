use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `meta=filerepoinfo` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMetaFilerepoinfoData {
    friprop: Option<Vec<String>>,
}

impl ActionApiData for ActionApiMetaFilerepoinfoData {}

impl ActionApiMetaFilerepoinfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.friprop, "friprop", &mut params);
        params
    }
}

/// Builder for `meta=filerepoinfo` — returns meta information about image repositories.
#[derive(Debug, Clone)]
pub struct ActionApiMetaFilerepoinfoBuilder {
    pub(crate) data: ActionApiMetaFilerepoinfoData,
}

impl ActionApiMetaFilerepoinfoBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiMetaFilerepoinfoData::default(),
        }
    }

    /// Which repository properties to get (`friprop`).
    pub fn friprop<S: Into<String> + Clone>(mut self, friprop: &[S]) -> Self {
        self.data.friprop = Some(friprop.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiRunnable for ActionApiMetaFilerepoinfoBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("meta".to_string(), "filerepoinfo".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiMetaFilerepoinfoBuilder {
        ActionApiMetaFilerepoinfoBuilder::new()
    }

    #[test]
    fn default_friprop_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("friprop"));
    }

    #[test]
    fn friprop_set() {
        let params = new_builder().friprop(&["name", "url", "local"]).data.params();
        assert_eq!(params["friprop"], "name|url|local");
    }

    #[test]
    fn runnable_params_contain_action_meta() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["meta"], "filerepoinfo");
    }
}
