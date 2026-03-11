use super::{
    ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryRedirectsData {
    common: ActionApiQueryCommonData,
    rdprop: Option<Vec<String>>,
    rdnamespace: Option<Vec<NamespaceID>>,
    rdshow: Option<Vec<String>>,
    rdlimit: usize,
    rdcontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryRedirectsData {}

impl Default for ActionApiQueryRedirectsData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            rdprop: None,
            rdnamespace: None,
            rdshow: None,
            rdlimit: 10,
            rdcontinue: None,
        }
    }
}

impl ActionApiQueryRedirectsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.rdprop, "rdprop", &mut params);
        if let Some(ns) = &self.rdnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("rdnamespace".to_string(), s.join("|"));
        }
        Self::add_vec(&self.rdshow, "rdshow", &mut params);
        params.insert("rdlimit".to_string(), self.rdlimit.to_string());
        Self::add_str(&self.rdcontinue, "rdcontinue", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiQueryRedirectsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryRedirectsData,
}

impl<T> ActionApiQueryRedirectsBuilder<T> {
    pub fn rdprop<S: Into<String> + Clone>(mut self, rdprop: &[S]) -> Self {
        self.data.rdprop = Some(rdprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn rdnamespace(mut self, rdnamespace: &[NamespaceID]) -> Self {
        self.data.rdnamespace = Some(rdnamespace.to_vec());
        self
    }

    pub fn rdshow<S: Into<String> + Clone>(mut self, rdshow: &[S]) -> Self {
        self.data.rdshow = Some(rdshow.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn rdlimit(mut self, rdlimit: usize) -> Self {
        self.data.rdlimit = rdlimit;
        self
    }
}

impl ActionApiQueryRedirectsBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryRedirectsData::default(),
        }
    }
}

impl ActionApiGenerator for ActionApiQueryRedirectsBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "redirects".to_string());
        params
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryRedirectsBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryRedirectsBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryRedirectsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryRedirectsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "redirects".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Api,
        action_api::{ActionApiQuery, ActionApiQueryCommonBuilder, NoTitlesOrGenerator},
    };

    fn new_builder() -> ActionApiQueryRedirectsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryRedirectsBuilder::new()
    }

    #[test]
    fn default_rdlimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["rdlimit"], "10");
    }

    #[test]
    fn default_rdprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("rdprop"));
    }

    #[test]
    fn rdprop_set() {
        let params = new_builder()
            .rdprop(&["pageid", "title", "fragment"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["rdprop"], "pageid|title|fragment");
    }

    #[test]
    fn rdnamespace_set() {
        let params = new_builder().rdnamespace(&[0]).titles(&["Foo"]).data.params();
        assert_eq!(params["rdnamespace"], "0");
    }

    #[test]
    fn rdshow_fragment() {
        let params = new_builder().rdshow(&["!fragment"]).titles(&["Foo"]).data.params();
        assert_eq!(params["rdshow"], "!fragment");
    }

    #[test]
    fn rdlimit_set() {
        let params = new_builder().rdlimit(100).titles(&["Foo"]).data.params();
        assert_eq!(params["rdlimit"], "100");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "redirects");
    }

    #[tokio::test]
    async fn test_redirects() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::redirects()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
