use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `meta=languageinfo` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMetaLanguageinfoData {
    liprop: Option<Vec<String>>,
    licode: Option<String>,
    licontinue: Option<String>,
}

impl ActionApiData for ActionApiMetaLanguageinfoData {}

impl ActionApiMetaLanguageinfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.liprop, "liprop", &mut params);
        Self::add_str(&self.licode, "licode", &mut params);
        Self::add_str(&self.licontinue, "licontinue", &mut params);
        params
    }
}

/// Builder for `meta=languageinfo` — returns information about available languages.
#[derive(Debug, Clone)]
pub struct ActionApiMetaLanguageinfoBuilder {
    pub(crate) data: ActionApiMetaLanguageinfoData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiMetaLanguageinfoBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiMetaLanguageinfoData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Which properties to get for each language (`liprop`).
    pub fn liprop<S: Into<String> + Clone>(mut self, liprop: &[S]) -> Self {
        self.data.liprop = Some(liprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// BCP-47 language code to filter by (`licode`).
    pub fn licode<S: AsRef<str>>(mut self, licode: S) -> Self {
        self.data.licode = Some(licode.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiMetaLanguageinfoBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("meta".to_string(), "languageinfo".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiMetaLanguageinfoBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiMetaLanguageinfoBuilder {
        ActionApiMetaLanguageinfoBuilder::new()
    }

    #[test]
    fn default_liprop_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("liprop"));
    }

    #[test]
    fn liprop_set() {
        let params = new_builder().liprop(&["code", "name", "autonym"]).data.params();
        assert_eq!(params["liprop"], "code|name|autonym");
    }

    #[test]
    fn licode_set() {
        let params = new_builder().licode("de").data.params();
        assert_eq!(params["licode"], "de");
    }

    #[test]
    fn runnable_params_contain_action_meta() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["meta"], "languageinfo");
    }
}
