use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `meta=userinfo` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMetaUserinfoData {
    uiprop: Option<Vec<String>>,
    uiattachedwiki: Option<String>,
}

impl ActionApiData for ActionApiMetaUserinfoData {}

impl ActionApiMetaUserinfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.uiprop, "uiprop", &mut params);
        Self::add_str(&self.uiattachedwiki, "uiattachedwiki", &mut params);
        params
    }
}

/// Builder for `meta=userinfo` — returns information about the current user.
#[derive(Debug, Clone)]
pub struct ActionApiMetaUserinfoBuilder {
    pub(crate) data: ActionApiMetaUserinfoData,
}

impl ActionApiMetaUserinfoBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiMetaUserinfoData::default(),
        }
    }

    /// Which pieces of information to include (`uiprop`).
    pub fn uiprop<S: Into<String> + Clone>(mut self, uiprop: &[S]) -> Self {
        self.data.uiprop = Some(uiprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Wiki to check if the current user is attached to (`uiattachedwiki`).
    pub fn uiattachedwiki<S: AsRef<str>>(mut self, uiattachedwiki: S) -> Self {
        self.data.uiattachedwiki = Some(uiattachedwiki.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiMetaUserinfoBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("meta".to_string(), "userinfo".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiMetaUserinfoBuilder {
        ActionApiMetaUserinfoBuilder::new()
    }

    #[test]
    fn default_uiprop_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("uiprop"));
    }

    #[test]
    fn uiprop_set() {
        let params = new_builder().uiprop(&["groups", "rights"]).data.params();
        assert_eq!(params["uiprop"], "groups|rights");
    }

    #[test]
    fn uiattachedwiki_set() {
        let params = new_builder().uiattachedwiki("enwiki").data.params();
        assert_eq!(params["uiattachedwiki"], "enwiki");
    }

    #[test]
    fn runnable_params_contain_action_meta() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["meta"], "userinfo");
    }
}
