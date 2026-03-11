use super::{
    ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder,
    ActionApiQueryCommonData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=links` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiQueryLinksData {
    common: ActionApiQueryCommonData,
    plnamespace: Option<Vec<NamespaceID>>,
    pllimit: usize,
    plcontinue: Option<String>,
    pltitles: Option<Vec<String>>,
    pldir: Option<String>,
}

impl ActionApiData for ActionApiQueryLinksData {}

impl Default for ActionApiQueryLinksData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            plnamespace: None,
            pllimit: 10,
            plcontinue: None,
            pltitles: None,
            pldir: None,
        }
    }
}

impl ActionApiQueryLinksData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        if let Some(ns) = &self.plnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("plnamespace".to_string(), s.join("|"));
        }
        params.insert("pllimit".to_string(), self.pllimit.to_string());
        Self::add_str(&self.plcontinue, "plcontinue", &mut params);
        Self::add_vec(&self.pltitles, "pltitles", &mut params);
        Self::add_str(&self.pldir, "pldir", &mut params);
        params
    }
}

/// Builder for the `prop=links` query module; uses the typestate pattern, starting in
/// `NoTitlesOrGenerator` and becoming `Runnable` once titles/pageids/revids/generator is set via `ActionApiQueryCommonBuilder`.
#[derive(Debug, Clone)]
pub struct ActionApiQueryLinksBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryLinksData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryLinksBuilder<T> {
    /// Filter links to only those in the given namespaces (`plnamespace`).
    pub fn plnamespace(mut self, plnamespace: &[NamespaceID]) -> Self {
        self.data.plnamespace = Some(plnamespace.to_vec());
        self
    }

    /// Maximum number of links to return (`pllimit`).
    pub fn pllimit(mut self, pllimit: usize) -> Self {
        self.data.pllimit = pllimit;
        self
    }

    /// Only list links to these specific titles (`pltitles`).
    pub fn pltitles<S: Into<String> + Clone>(mut self, pltitles: &[S]) -> Self {
        self.data.pltitles = Some(pltitles.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Direction to list links in, either `ascending` or `descending` (`pldir`).
    pub fn pldir<S: AsRef<str>>(mut self, pldir: S) -> Self {
        self.data.pldir = Some(pldir.as_ref().to_string());
        self
    }
}

impl ActionApiQueryLinksBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryLinksData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiQueryLinksBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "links".to_string());
        params
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryLinksBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryLinksBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryLinksBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryLinksBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "links".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryLinksBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryLinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinksBuilder::new()
    }

    #[test]
    fn default_pllimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["pllimit"], "10");
    }

    #[test]
    fn default_plnamespace_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("plnamespace"));
    }

    #[test]
    fn plnamespace_set() {
        let params = new_builder().plnamespace(&[0, 4]).titles(&["Foo"]).data.params();
        assert_eq!(params["plnamespace"], "0|4");
    }

    #[test]
    fn pllimit_set() {
        let params = new_builder().pllimit(50).titles(&["Foo"]).data.params();
        assert_eq!(params["pllimit"], "50");
    }

    #[test]
    fn pltitles_filter() {
        let params = new_builder()
            .pltitles(&["Berlin", "Paris"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["pltitles"], "Berlin|Paris");
    }

    #[test]
    fn pldir_descending() {
        let params = new_builder().pldir("descending").titles(&["Foo"]).data.params();
        assert_eq!(params["pldir"], "descending");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "links");
    }

    #[tokio::test]
    async fn test_links() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::links()
            .titles(&["Albert Einstein"])
            .plnamespace(&[0])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
