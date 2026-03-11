use super::{ActionApiData, ActionApiRunnable};
use crate::api::NamespaceID;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ActionApiListAllpagesData {
    apfrom: Option<String>,
    apcontinue: Option<String>,
    apto: Option<String>,
    apprefix: Option<String>,
    apnamespace: NamespaceID,
    apfilterredir: Option<String>,
    apfilterlanglinks: Option<String>,
    apminsize: Option<u32>,
    apmaxsize: Option<u32>,
    apprtype: Option<Vec<String>>,
    apprlevel: Option<Vec<String>>,
    apprfiltercascade: Option<String>,
    apprexpiry: Option<String>,
    aplimit: usize,
    apdir: Option<String>,
}

impl ActionApiData for ActionApiListAllpagesData {}

impl Default for ActionApiListAllpagesData {
    fn default() -> Self {
        Self {
            apfrom: None,
            apcontinue: None,
            apto: None,
            apprefix: None,
            apnamespace: 0,
            apfilterredir: None,
            apfilterlanglinks: None,
            apminsize: None,
            apmaxsize: None,
            apprtype: None,
            apprlevel: None,
            apprfiltercascade: None,
            apprexpiry: None,
            aplimit: 10,
            apdir: None,
        }
    }
}

impl ActionApiListAllpagesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.apfrom, "apfrom", &mut params);
        Self::add_str(&self.apcontinue, "apcontinue", &mut params);
        Self::add_str(&self.apto, "apto", &mut params);
        Self::add_str(&self.apprefix, "apprefix", &mut params);
        params.insert("apnamespace".to_string(), self.apnamespace.to_string());
        Self::add_str(&self.apfilterredir, "apfilterredir", &mut params);
        Self::add_str(&self.apfilterlanglinks, "apfilterlanglinks", &mut params);
        if let Some(v) = self.apminsize {
            params.insert("apminsize".to_string(), v.to_string());
        }
        if let Some(v) = self.apmaxsize {
            params.insert("apmaxsize".to_string(), v.to_string());
        }
        Self::add_vec(&self.apprtype, "apprtype", &mut params);
        Self::add_vec(&self.apprlevel, "apprlevel", &mut params);
        Self::add_str(&self.apprfiltercascade, "apprfiltercascade", &mut params);
        Self::add_str(&self.apprexpiry, "apprexpiry", &mut params);
        params.insert("aplimit".to_string(), self.aplimit.to_string());
        Self::add_str(&self.apdir, "apdir", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiListAllpagesBuilder {
    pub(crate) data: ActionApiListAllpagesData,
}

impl ActionApiListAllpagesBuilder {
    pub fn new() -> Self {
        Self {
            data: ActionApiListAllpagesData::default(),
        }
    }

    pub fn apfrom<S: AsRef<str>>(mut self, apfrom: S) -> Self {
        self.data.apfrom = Some(apfrom.as_ref().to_string());
        self
    }

    pub fn apto<S: AsRef<str>>(mut self, apto: S) -> Self {
        self.data.apto = Some(apto.as_ref().to_string());
        self
    }

    pub fn apprefix<S: AsRef<str>>(mut self, apprefix: S) -> Self {
        self.data.apprefix = Some(apprefix.as_ref().to_string());
        self
    }

    pub fn apnamespace(mut self, apnamespace: NamespaceID) -> Self {
        self.data.apnamespace = apnamespace;
        self
    }

    pub fn apfilterredir<S: AsRef<str>>(mut self, apfilterredir: S) -> Self {
        self.data.apfilterredir = Some(apfilterredir.as_ref().to_string());
        self
    }

    pub fn apfilterlanglinks<S: AsRef<str>>(mut self, apfilterlanglinks: S) -> Self {
        self.data.apfilterlanglinks = Some(apfilterlanglinks.as_ref().to_string());
        self
    }

    pub fn apminsize(mut self, apminsize: u32) -> Self {
        self.data.apminsize = Some(apminsize);
        self
    }

    pub fn apmaxsize(mut self, apmaxsize: u32) -> Self {
        self.data.apmaxsize = Some(apmaxsize);
        self
    }

    pub fn apprtype<S: Into<String> + Clone>(mut self, apprtype: &[S]) -> Self {
        self.data.apprtype = Some(apprtype.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn apprlevel<S: Into<String> + Clone>(mut self, apprlevel: &[S]) -> Self {
        self.data.apprlevel = Some(apprlevel.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn apprfiltercascade<S: AsRef<str>>(mut self, apprfiltercascade: S) -> Self {
        self.data.apprfiltercascade = Some(apprfiltercascade.as_ref().to_string());
        self
    }

    pub fn apprexpiry<S: AsRef<str>>(mut self, apprexpiry: S) -> Self {
        self.data.apprexpiry = Some(apprexpiry.as_ref().to_string());
        self
    }

    pub fn aplimit(mut self, aplimit: usize) -> Self {
        self.data.aplimit = aplimit;
        self
    }

    pub fn apdir<S: AsRef<str>>(mut self, apdir: S) -> Self {
        self.data.apdir = Some(apdir.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListAllpagesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "allpages".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListAllpagesBuilder {
        ActionApiListAllpagesBuilder::new()
    }

    #[test]
    fn default_apnamespace_is_0() {
        let params = new_builder().data.params();
        assert_eq!(params["apnamespace"], "0");
    }

    #[test]
    fn default_aplimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["aplimit"], "10");
    }

    #[test]
    fn default_apfrom_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("apfrom"));
    }

    #[test]
    fn apprefix_set() {
        let params = new_builder().apprefix("Albert").data.params();
        assert_eq!(params["apprefix"], "Albert");
    }

    #[test]
    fn apnamespace_set() {
        let params = new_builder().apnamespace(4).data.params();
        assert_eq!(params["apnamespace"], "4");
    }

    #[test]
    fn apfilterredir_nonredirects() {
        let params = new_builder().apfilterredir("nonredirects").data.params();
        assert_eq!(params["apfilterredir"], "nonredirects");
    }

    #[test]
    fn apminsize_set() {
        let params = new_builder().apminsize(1000).data.params();
        assert_eq!(params["apminsize"], "1000");
    }

    #[test]
    fn apmaxsize_set() {
        let params = new_builder().apmaxsize(5000).data.params();
        assert_eq!(params["apmaxsize"], "5000");
    }

    #[test]
    fn aplimit_set() {
        let params = new_builder().aplimit(50).data.params();
        assert_eq!(params["aplimit"], "50");
    }

    #[test]
    fn apdir_descending() {
        let params = new_builder().apdir("descending").data.params();
        assert_eq!(params["apdir"], "descending");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "allpages");
    }

    #[tokio::test]
    async fn test_allpages() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::allpages()
            .apprefix("Albert")
            .aplimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["allpages"].is_array());
    }
}
