use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=categoryinfo` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryCategoryinfoData {
    common: ActionApiQueryCommonData,
    cicontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryCategoryinfoData {}

impl ActionApiQueryCategoryinfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_str(&self.cicontinue, "cicontinue", &mut params);
        params
    }
}

/// Builder for the `prop=categoryinfo` query module.
///
/// Starts in `NoTitlesOrGenerator` state and becomes `Runnable` after titles, pageids, revids,
/// or a generator is set via `ActionApiQueryCommonBuilder`.
#[derive(Debug, Clone)]
pub struct ActionApiQueryCategoryinfoBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryCategoryinfoData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryCategoryinfoBuilder<T> {
    /// Continuation token for resuming a previous query (`cicontinue`).
    pub fn cicontinue<S: AsRef<str>>(mut self, cicontinue: S) -> Self {
        self.data.cicontinue = Some(cicontinue.as_ref().to_string());
        self
    }
}

impl ActionApiQueryCategoryinfoBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryCategoryinfoData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryCategoryinfoBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryCategoryinfoBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryCategoryinfoBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryCategoryinfoBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "categoryinfo".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryCategoryinfoBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryCategoryinfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryCategoryinfoBuilder::new()
    }

    #[test]
    fn default_cicontinue_absent() {
        let params = new_builder().titles(&["Category:Foo"]).data.params();
        assert!(!params.contains_key("cicontinue"));
    }

    #[test]
    fn cicontinue_set() {
        let params = new_builder()
            .cicontinue("token123")
            .titles(&["Category:Foo"])
            .data
            .params();
        assert_eq!(params["cicontinue"], "token123");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Category:Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "categoryinfo");
    }

    #[tokio::test]
    async fn test_categoryinfo() {
        use wiremock::{Mock, ResponseTemplate};
        use wiremock::matchers::query_param;
        let server = crate::test_helpers::test_helpers::start_enwiki_mock().await;
        Mock::given(query_param("prop", "categoryinfo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "3235": {
                            "pageid": 3235, "ns": 14, "title": "Category:Physics",
                            "categoryinfo": {"size": 100, "subcats": 20, "files": 5}
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::categoryinfo()
            .titles(&["Category:Physics"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
