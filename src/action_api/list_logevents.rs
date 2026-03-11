use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ActionApiListLogeventsData {
    leprop: Option<Vec<String>>,
    letype: Option<String>,
    leaction: Option<String>,
    lestart: Option<String>,
    leend: Option<String>,
    ledir: Option<String>,
    leuser: Option<String>,
    letitle: Option<String>,
    lenamespace: Option<NamespaceID>,
    letag: Option<String>,
    lelimit: usize,
    lecontinue: Option<String>,
}

impl ActionApiData for ActionApiListLogeventsData {}

impl Default for ActionApiListLogeventsData {
    fn default() -> Self {
        Self {
            leprop: None,
            letype: None,
            leaction: None,
            lestart: None,
            leend: None,
            ledir: None,
            leuser: None,
            letitle: None,
            lenamespace: None,
            letag: None,
            lelimit: 10,
            lecontinue: None,
        }
    }
}

impl ActionApiListLogeventsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.leprop, "leprop", &mut params);
        Self::add_str(&self.letype, "letype", &mut params);
        Self::add_str(&self.leaction, "leaction", &mut params);
        Self::add_str(&self.lestart, "lestart", &mut params);
        Self::add_str(&self.leend, "leend", &mut params);
        Self::add_str(&self.ledir, "ledir", &mut params);
        Self::add_str(&self.leuser, "leuser", &mut params);
        Self::add_str(&self.letitle, "letitle", &mut params);
        if let Some(ns) = self.lenamespace {
            params.insert("lenamespace".to_string(), ns.to_string());
        }
        Self::add_str(&self.letag, "letag", &mut params);
        params.insert("lelimit".to_string(), self.lelimit.to_string());
        Self::add_str(&self.lecontinue, "lecontinue", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiListLogeventsBuilder {
    pub(crate) data: ActionApiListLogeventsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListLogeventsBuilder {
    pub fn new() -> Self {
        Self {
            data: ActionApiListLogeventsData::default(),
            continue_params: HashMap::new(),
        }
    }

    pub fn leprop<S: Into<String> + Clone>(mut self, leprop: &[S]) -> Self {
        self.data.leprop = Some(leprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn letype<S: AsRef<str>>(mut self, letype: S) -> Self {
        self.data.letype = Some(letype.as_ref().to_string());
        self
    }

    pub fn leaction<S: AsRef<str>>(mut self, leaction: S) -> Self {
        self.data.leaction = Some(leaction.as_ref().to_string());
        self
    }

    pub fn lestart<S: AsRef<str>>(mut self, lestart: S) -> Self {
        self.data.lestart = Some(lestart.as_ref().to_string());
        self
    }

    pub fn leend<S: AsRef<str>>(mut self, leend: S) -> Self {
        self.data.leend = Some(leend.as_ref().to_string());
        self
    }

    pub fn ledir<S: AsRef<str>>(mut self, ledir: S) -> Self {
        self.data.ledir = Some(ledir.as_ref().to_string());
        self
    }

    pub fn leuser<S: AsRef<str>>(mut self, leuser: S) -> Self {
        self.data.leuser = Some(leuser.as_ref().to_string());
        self
    }

    pub fn letitle<S: AsRef<str>>(mut self, letitle: S) -> Self {
        self.data.letitle = Some(letitle.as_ref().to_string());
        self
    }

    pub fn lenamespace(mut self, lenamespace: NamespaceID) -> Self {
        self.data.lenamespace = Some(lenamespace);
        self
    }

    pub fn letag<S: AsRef<str>>(mut self, letag: S) -> Self {
        self.data.letag = Some(letag.as_ref().to_string());
        self
    }

    pub fn lelimit(mut self, lelimit: usize) -> Self {
        self.data.lelimit = lelimit;
        self
    }
}

impl ActionApiRunnable for ActionApiListLogeventsBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "logevents".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListLogeventsBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListLogeventsBuilder {
        ActionApiListLogeventsBuilder::new()
    }

    #[test]
    fn default_lelimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["lelimit"], "10");
    }

    #[test]
    fn default_letype_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("letype"));
    }

    #[test]
    fn leprop_set() {
        let params = new_builder().leprop(&["ids", "title", "type", "user"]).data.params();
        assert_eq!(params["leprop"], "ids|title|type|user");
    }

    #[test]
    fn letype_set() {
        let params = new_builder().letype("move").data.params();
        assert_eq!(params["letype"], "move");
    }

    #[test]
    fn leaction_set() {
        let params = new_builder().leaction("move/move").data.params();
        assert_eq!(params["leaction"], "move/move");
    }

    #[test]
    fn leuser_set() {
        let params = new_builder().leuser("ExampleUser").data.params();
        assert_eq!(params["leuser"], "ExampleUser");
    }

    #[test]
    fn letitle_set() {
        let params = new_builder().letitle("Albert Einstein").data.params();
        assert_eq!(params["letitle"], "Albert Einstein");
    }

    #[test]
    fn lenamespace_set() {
        let params = new_builder().lenamespace(0).data.params();
        assert_eq!(params["lenamespace"], "0");
    }

    #[test]
    fn lelimit_set() {
        let params = new_builder().lelimit(50).data.params();
        assert_eq!(params["lelimit"], "50");
    }

    #[test]
    fn ledir_newer() {
        let params = new_builder().ledir("newer").data.params();
        assert_eq!(params["ledir"], "newer");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "logevents");
    }

    #[tokio::test]
    async fn test_logevents() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::logevents()
            .letype("move")
            .lelimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["logevents"].is_array());
    }
}
