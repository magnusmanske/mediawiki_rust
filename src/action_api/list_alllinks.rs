use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

/// Internal data container for `list=alllinks` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListAlllinksData {
    alcontinue: Option<String>,
    alfrom: Option<String>,
    alto: Option<String>,
    alprefix: Option<String>,
    alunique: bool,
    alprop: Option<Vec<String>>,
    alnamespace: NamespaceID,
    allimit: usize,
    aldir: Option<String>,
}

impl ActionApiData for ActionApiListAlllinksData {}

impl Default for ActionApiListAlllinksData {
    fn default() -> Self {
        Self {
            alcontinue: None,
            alfrom: None,
            alto: None,
            alprefix: None,
            alunique: false,
            alprop: None,
            alnamespace: 0,
            allimit: 10,
            aldir: None,
        }
    }
}

impl ActionApiListAlllinksData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.alcontinue, "alcontinue", &mut params);
        Self::add_str(&self.alfrom, "alfrom", &mut params);
        Self::add_str(&self.alto, "alto", &mut params);
        Self::add_str(&self.alprefix, "alprefix", &mut params);
        Self::add_boolean(self.alunique, "alunique", &mut params);
        Self::add_vec(&self.alprop, "alprop", &mut params);
        params.insert("alnamespace".to_string(), self.alnamespace.to_string());
        params.insert("allimit".to_string(), self.allimit.to_string());
        Self::add_str(&self.aldir, "aldir", &mut params);
        params
    }
}

/// Builder for `list=alllinks` — enumerates all links pointing to a given namespace.
#[derive(Debug, Clone)]
pub struct ActionApiListAlllinksBuilder {
    pub(crate) data: ActionApiListAlllinksData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListAlllinksBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListAlllinksData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this title (`alfrom`).
    pub fn alfrom<S: AsRef<str>>(mut self, alfrom: S) -> Self {
        self.data.alfrom = Some(alfrom.as_ref().to_string());
        self
    }

    /// Stop listing at this title (`alto`).
    pub fn alto<S: AsRef<str>>(mut self, alto: S) -> Self {
        self.data.alto = Some(alto.as_ref().to_string());
        self
    }

    /// Prefix to search for (`alprefix`).
    pub fn alprefix<S: AsRef<str>>(mut self, alprefix: S) -> Self {
        self.data.alprefix = Some(alprefix.as_ref().to_string());
        self
    }

    /// Only show distinct linked titles (`alunique`).
    pub fn alunique(mut self, alunique: bool) -> Self {
        self.data.alunique = alunique;
        self
    }

    /// Properties to return (`alprop`).
    pub fn alprop<S: Into<String> + Clone>(mut self, alprop: &[S]) -> Self {
        self.data.alprop = Some(alprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Namespace to enumerate (`alnamespace`).
    pub fn alnamespace(mut self, alnamespace: NamespaceID) -> Self {
        self.data.alnamespace = alnamespace;
        self
    }

    /// Maximum number of items to return (`allimit`).
    pub fn allimit(mut self, allimit: usize) -> Self {
        self.data.allimit = allimit;
        self
    }

    /// Direction to list (`aldir`).
    pub fn aldir<S: AsRef<str>>(mut self, aldir: S) -> Self {
        self.data.aldir = Some(aldir.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListAlllinksBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "alllinks".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListAlllinksBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListAlllinksBuilder {
        ActionApiListAlllinksBuilder::new()
    }

    #[test]
    fn default_alnamespace_is_0() {
        let params = new_builder().data.params();
        assert_eq!(params["alnamespace"], "0");
    }

    #[test]
    fn default_allimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["allimit"], "10");
    }

    #[test]
    fn alprefix_set() {
        let params = new_builder().alprefix("Albert").data.params();
        assert_eq!(params["alprefix"], "Albert");
    }

    #[test]
    fn alnamespace_set() {
        let params = new_builder().alnamespace(4).data.params();
        assert_eq!(params["alnamespace"], "4");
    }

    #[test]
    fn allimit_set() {
        let params = new_builder().allimit(50).data.params();
        assert_eq!(params["allimit"], "50");
    }

    #[test]
    fn alunique_flag() {
        let params = new_builder().alunique(true).data.params();
        assert_eq!(params["alunique"], "");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "alllinks");
    }
}
