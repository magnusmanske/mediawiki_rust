use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `meta=tokens` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMetaTokensData {
    token_type: Option<Vec<String>>,
}

impl ActionApiData for ActionApiMetaTokensData {}

impl ActionApiMetaTokensData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(ref v) = self.token_type {
            params.insert("type".to_string(), v.join("|"));
        }
        params
    }
}

/// Builder for `meta=tokens` — gets tokens for data-modifying actions.
#[derive(Debug, Clone)]
pub struct ActionApiMetaTokensBuilder {
    pub(crate) data: ActionApiMetaTokensData,
}

impl ActionApiMetaTokensBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiMetaTokensData::default(),
        }
    }

    /// Types of token to request (`type`).
    pub fn token_type<S: Into<String> + Clone>(mut self, token_type: &[S]) -> Self {
        self.data.token_type = Some(token_type.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiRunnable for ActionApiMetaTokensBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("meta".to_string(), "tokens".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiMetaTokensBuilder {
        ActionApiMetaTokensBuilder::new()
    }

    #[test]
    fn default_type_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("type"));
    }

    #[test]
    fn token_type_single() {
        let params = new_builder().token_type(&["csrf"]).data.params();
        assert_eq!(params["type"], "csrf");
    }

    #[test]
    fn token_type_multiple() {
        let params = new_builder()
            .token_type(&["csrf", "patrol", "rollback"])
            .data
            .params();
        assert_eq!(params["type"], "csrf|patrol|rollback");
    }

    #[test]
    fn runnable_params_contain_action_meta() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["meta"], "tokens");
    }
}
