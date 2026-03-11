use super::{
    ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryFileusageData {
    common: ActionApiQueryCommonData,
    fuprop: Option<Vec<String>>,
    funamespace: Option<Vec<NamespaceID>>,
    fushow: Option<Vec<String>>,
    fulimit: usize,
    fucontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryFileusageData {}

impl Default for ActionApiQueryFileusageData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            fuprop: None,
            funamespace: None,
            fushow: None,
            fulimit: 10,
            fucontinue: None,
        }
    }
}

impl ActionApiQueryFileusageData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.fuprop, "fuprop", &mut params);
        if let Some(ns) = &self.funamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("funamespace".to_string(), s.join("|"));
        }
        Self::add_vec(&self.fushow, "fushow", &mut params);
        params.insert("fulimit".to_string(), self.fulimit.to_string());
        Self::add_str(&self.fucontinue, "fucontinue", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiQueryFileusageBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryFileusageData,
}

impl<T> ActionApiQueryFileusageBuilder<T> {
    pub fn fuprop<S: Into<String> + Clone>(mut self, fuprop: &[S]) -> Self {
        self.data.fuprop = Some(fuprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn funamespace(mut self, funamespace: &[NamespaceID]) -> Self {
        self.data.funamespace = Some(funamespace.to_vec());
        self
    }

    pub fn fushow<S: Into<String> + Clone>(mut self, fushow: &[S]) -> Self {
        self.data.fushow = Some(fushow.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn fulimit(mut self, fulimit: usize) -> Self {
        self.data.fulimit = fulimit;
        self
    }
}

impl ActionApiQueryFileusageBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryFileusageData::default(),
        }
    }
}

impl ActionApiGenerator for ActionApiQueryFileusageBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "fileusage".to_string());
        params
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryFileusageBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryFileusageBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryFileusageBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryFileusageBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "fileusage".to_string());
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

    fn new_builder() -> ActionApiQueryFileusageBuilder<NoTitlesOrGenerator> {
        ActionApiQueryFileusageBuilder::new()
    }

    #[test]
    fn default_fulimit_is_10() {
        let params = new_builder().titles(&["File:Foo.jpg"]).data.params();
        assert_eq!(params["fulimit"], "10");
    }

    #[test]
    fn default_fuprop_absent() {
        let params = new_builder().titles(&["File:Foo.jpg"]).data.params();
        assert!(!params.contains_key("fuprop"));
    }

    #[test]
    fn fuprop_set() {
        let params = new_builder()
            .fuprop(&["pageid", "title"])
            .titles(&["File:Foo.jpg"])
            .data
            .params();
        assert_eq!(params["fuprop"], "pageid|title");
    }

    #[test]
    fn funamespace_set() {
        let params = new_builder().funamespace(&[0, 4]).titles(&["File:Foo.jpg"]).data.params();
        assert_eq!(params["funamespace"], "0|4");
    }

    #[test]
    fn fushow_set() {
        let params = new_builder().fushow(&["!redirect"]).titles(&["File:Foo.jpg"]).data.params();
        assert_eq!(params["fushow"], "!redirect");
    }

    #[test]
    fn fulimit_set() {
        let params = new_builder().fulimit(100).titles(&["File:Foo.jpg"]).data.params();
        assert_eq!(params["fulimit"], "100");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["File:Foo.jpg"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "fileusage");
    }

    #[tokio::test]
    async fn test_fileusage() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::fileusage()
            .titles(&["File:Einstein1921 by F Schmutzer 2.jpg"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
