use super::{ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiListEmbeddedinData {
    eititle: Option<String>,
    eipageid: Option<u64>,
    eicontinue: Option<String>,
    einamespace: Option<Vec<NamespaceID>>,
    eidir: Option<String>,
    eifilterredir: Option<String>,
    eilimit: usize,
}

impl ActionApiData for ActionApiListEmbeddedinData {}

impl Default for ActionApiListEmbeddedinData {
    fn default() -> Self {
        Self {
            eititle: None,
            eipageid: None,
            eicontinue: None,
            einamespace: None,
            eidir: None,
            eifilterredir: None,
            eilimit: 10,
        }
    }
}

impl ActionApiListEmbeddedinData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.eititle, "eititle", &mut params);
        if let Some(eipageid) = self.eipageid {
            params.insert("eipageid".to_string(), eipageid.to_string());
        }
        Self::add_str(&self.eicontinue, "eicontinue", &mut params);
        if let Some(ns) = &self.einamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("einamespace".to_string(), s.join("|"));
        }
        Self::add_str(&self.eidir, "eidir", &mut params);
        Self::add_str(&self.eifilterredir, "eifilterredir", &mut params);
        params.insert("eilimit".to_string(), self.eilimit.to_string());
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiListEmbeddedinBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListEmbeddedinData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListEmbeddedinBuilder<T> {
    pub fn einamespace(mut self, einamespace: &[NamespaceID]) -> Self {
        self.data.einamespace = Some(einamespace.to_vec());
        self
    }

    pub fn eidir<S: AsRef<str>>(mut self, eidir: S) -> Self {
        self.data.eidir = Some(eidir.as_ref().to_string());
        self
    }

    pub fn eifilterredir<S: AsRef<str>>(mut self, eifilterredir: S) -> Self {
        self.data.eifilterredir = Some(eifilterredir.as_ref().to_string());
        self
    }

    pub fn eilimit(mut self, eilimit: usize) -> Self {
        self.data.eilimit = eilimit;
        self
    }
}

impl ActionApiListEmbeddedinBuilder<NoTitlesOrGenerator> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListEmbeddedinData::default(),
            continue_params: HashMap::new(),
        }
    }

    pub fn eititle<S: AsRef<str>>(mut self, eititle: S) -> ActionApiListEmbeddedinBuilder<Runnable> {
        self.data.eititle = Some(eititle.as_ref().to_string());
        ActionApiListEmbeddedinBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    pub fn eipageid(mut self, eipageid: u64) -> ActionApiListEmbeddedinBuilder<Runnable> {
        self.data.eipageid = Some(eipageid);
        ActionApiListEmbeddedinBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiListEmbeddedinBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "embeddedin".to_string());
        params
    }
}

impl ActionApiRunnable for ActionApiListEmbeddedinBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "embeddedin".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListEmbeddedinBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListEmbeddedinBuilder<NoTitlesOrGenerator> {
        ActionApiListEmbeddedinBuilder::new()
    }

    #[test]
    fn default_eilimit_is_10() {
        let params = new_builder().eititle("Template:Foo").data.params();
        assert_eq!(params["eilimit"], "10");
    }

    #[test]
    fn eititle_set() {
        let params = new_builder().eititle("Template:Infobox scientist").data.params();
        assert_eq!(params["eititle"], "Template:Infobox scientist");
    }

    #[test]
    fn eipageid_set() {
        let params = new_builder().eipageid(12345).data.params();
        assert_eq!(params["eipageid"], "12345");
    }

    #[test]
    fn einamespace_set() {
        let params = new_builder().einamespace(&[0]).eititle("Template:Foo").data.params();
        assert_eq!(params["einamespace"], "0");
    }

    #[test]
    fn eifilterredir_set() {
        let params = new_builder().eifilterredir("nonredirects").eititle("Template:Foo").data.params();
        assert_eq!(params["eifilterredir"], "nonredirects");
    }

    #[test]
    fn eilimit_set() {
        let params = new_builder().eilimit(50).eititle("Template:Foo").data.params();
        assert_eq!(params["eilimit"], "50");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().eititle("Template:Infobox scientist");
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "embeddedin");
    }

    #[tokio::test]
    async fn test_embeddedin() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::embeddedin()
            .eititle("Template:Infobox scientist")
            .eilimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["embeddedin"].is_array());
    }
}
