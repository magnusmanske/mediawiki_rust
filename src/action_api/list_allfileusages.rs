use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=allfileusages` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListAllfileusagesData {
    afcontinue: Option<String>,
    affrom: Option<String>,
    afto: Option<String>,
    afprefix: Option<String>,
    afunique: bool,
    afprop: Option<Vec<String>>,
    aflimit: usize,
    afdir: Option<String>,
}

impl ActionApiData for ActionApiListAllfileusagesData {}

impl Default for ActionApiListAllfileusagesData {
    fn default() -> Self {
        Self {
            afcontinue: None,
            affrom: None,
            afto: None,
            afprefix: None,
            afunique: false,
            afprop: None,
            aflimit: 10,
            afdir: None,
        }
    }
}

impl ActionApiListAllfileusagesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.afcontinue, "afcontinue", &mut params);
        Self::add_str(&self.affrom, "affrom", &mut params);
        Self::add_str(&self.afto, "afto", &mut params);
        Self::add_str(&self.afprefix, "afprefix", &mut params);
        Self::add_boolean(self.afunique, "afunique", &mut params);
        Self::add_vec(&self.afprop, "afprop", &mut params);
        params.insert("aflimit".to_string(), self.aflimit.to_string());
        Self::add_str(&self.afdir, "afdir", &mut params);
        params
    }
}

/// Builder for `list=allfileusages` — enumerates all file usages.
#[derive(Debug, Clone)]
pub struct ActionApiListAllfileusagesBuilder {
    pub(crate) data: ActionApiListAllfileusagesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListAllfileusagesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListAllfileusagesData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this file title (`affrom`).
    pub fn affrom<S: AsRef<str>>(mut self, affrom: S) -> Self {
        self.data.affrom = Some(affrom.as_ref().to_string());
        self
    }

    /// Stop listing at this file title (`afto`).
    pub fn afto<S: AsRef<str>>(mut self, afto: S) -> Self {
        self.data.afto = Some(afto.as_ref().to_string());
        self
    }

    /// Prefix to search for (`afprefix`).
    pub fn afprefix<S: AsRef<str>>(mut self, afprefix: S) -> Self {
        self.data.afprefix = Some(afprefix.as_ref().to_string());
        self
    }

    /// Only show distinct file titles (`afunique`).
    pub fn afunique(mut self, afunique: bool) -> Self {
        self.data.afunique = afunique;
        self
    }

    /// Properties to return (`afprop`).
    pub fn afprop<S: Into<String> + Clone>(mut self, afprop: &[S]) -> Self {
        self.data.afprop = Some(afprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of items to return (`aflimit`).
    pub fn aflimit(mut self, aflimit: usize) -> Self {
        self.data.aflimit = aflimit;
        self
    }

    /// Direction to list (`afdir`).
    pub fn afdir<S: AsRef<str>>(mut self, afdir: S) -> Self {
        self.data.afdir = Some(afdir.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListAllfileusagesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "allfileusages".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListAllfileusagesBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListAllfileusagesBuilder {
        ActionApiListAllfileusagesBuilder::new()
    }

    #[test]
    fn default_aflimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["aflimit"], "10");
    }

    #[test]
    fn default_affrom_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("affrom"));
    }

    #[test]
    fn afprefix_set() {
        let params = new_builder().afprefix("Logo").data.params();
        assert_eq!(params["afprefix"], "Logo");
    }

    #[test]
    fn aflimit_set() {
        let params = new_builder().aflimit(50).data.params();
        assert_eq!(params["aflimit"], "50");
    }

    #[test]
    fn afunique_flag() {
        let params = new_builder().afunique(true).data.params();
        assert_eq!(params["afunique"], "");
    }

    #[test]
    fn afprop_set() {
        let params = new_builder().afprop(&["ids", "title"]).data.params();
        assert_eq!(params["afprop"], "ids|title");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "allfileusages");
    }
}
