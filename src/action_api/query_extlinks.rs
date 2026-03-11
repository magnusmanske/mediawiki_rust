use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryExtlinksData {
    common: ActionApiQueryCommonData,
    ellimit: usize,
    elcontinue: Option<String>,
    elprotocol: Option<String>,
    elquery: Option<String>,
}

impl ActionApiData for ActionApiQueryExtlinksData {}

impl Default for ActionApiQueryExtlinksData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            ellimit: 10,
            elcontinue: None,
            elprotocol: None,
            elquery: None,
        }
    }
}

impl ActionApiQueryExtlinksData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        params.insert("ellimit".to_string(), self.ellimit.to_string());
        Self::add_str(&self.elcontinue, "elcontinue", &mut params);
        Self::add_str(&self.elprotocol, "elprotocol", &mut params);
        Self::add_str(&self.elquery, "elquery", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiQueryExtlinksBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryExtlinksData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryExtlinksBuilder<T> {
    pub fn ellimit(mut self, ellimit: usize) -> Self {
        self.data.ellimit = ellimit;
        self
    }

    pub fn elprotocol<S: AsRef<str>>(mut self, elprotocol: S) -> Self {
        self.data.elprotocol = Some(elprotocol.as_ref().to_string());
        self
    }

    pub fn elquery<S: AsRef<str>>(mut self, elquery: S) -> Self {
        self.data.elquery = Some(elquery.as_ref().to_string());
        self
    }
}

impl ActionApiQueryExtlinksBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryExtlinksData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryExtlinksBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryExtlinksBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryExtlinksBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryExtlinksBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "extlinks".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryExtlinksBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Api,
        action_api::{ActionApiQuery, ActionApiQueryCommonBuilder, NoTitlesOrGenerator},
    };

    fn new_builder() -> ActionApiQueryExtlinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryExtlinksBuilder::new()
    }

    #[test]
    fn default_ellimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["ellimit"], "10");
    }

    #[test]
    fn default_elprotocol_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("elprotocol"));
    }

    #[test]
    fn default_elquery_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("elquery"));
    }

    #[test]
    fn ellimit_set() {
        let params = new_builder().ellimit(50).titles(&["Foo"]).data.params();
        assert_eq!(params["ellimit"], "50");
    }

    #[test]
    fn elprotocol_set() {
        let params = new_builder().elprotocol("https").titles(&["Foo"]).data.params();
        assert_eq!(params["elprotocol"], "https");
    }

    #[test]
    fn elquery_set() {
        let params = new_builder().elquery("example.com").titles(&["Foo"]).data.params();
        assert_eq!(params["elquery"], "example.com");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "extlinks");
    }

    #[tokio::test]
    async fn test_extlinks() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::extlinks()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
