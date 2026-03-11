use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=pageprops` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryPagepropsData {
    common: ActionApiQueryCommonData,
    ppcontinue: Option<String>,
    ppprop: Option<Vec<String>>,
}

impl ActionApiData for ActionApiQueryPagepropsData {}

impl ActionApiQueryPagepropsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_str(&self.ppcontinue, "ppcontinue", &mut params);
        Self::add_vec(&self.ppprop, "ppprop", &mut params);
        params
    }
}

/// Builder for the `prop=pageprops` query module; uses the typestate pattern, starting in
/// `NoTitlesOrGenerator` and becoming `Runnable` once titles/pageids/revids/generator is set via `ActionApiQueryCommonBuilder`.
#[derive(Debug, Clone)]
pub struct ActionApiQueryPagepropsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryPagepropsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryPagepropsBuilder<T> {
    /// Only return these page properties; limits results to the named properties (`ppprop`).
    pub fn ppprop<S: Into<String> + Clone>(mut self, ppprop: &[S]) -> Self {
        self.data.ppprop = Some(ppprop.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiQueryPagepropsBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryPagepropsData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryPagepropsBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryPagepropsBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryPagepropsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryPagepropsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "pageprops".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryPagepropsBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryPagepropsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryPagepropsBuilder::new()
    }

    #[test]
    fn default_ppprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("ppprop"));
    }

    #[test]
    fn default_ppcontinue_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("ppcontinue"));
    }

    #[test]
    fn ppprop_single() {
        let params = new_builder().ppprop(&["wikibase_item"]).titles(&["Foo"]).data.params();
        assert_eq!(params["ppprop"], "wikibase_item");
    }

    #[test]
    fn ppprop_multiple() {
        let params = new_builder()
            .ppprop(&["wikibase_item", "defaultsort"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["ppprop"], "wikibase_item|defaultsort");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "pageprops");
    }

    #[tokio::test]
    async fn test_pageprops() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::pageprops()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
