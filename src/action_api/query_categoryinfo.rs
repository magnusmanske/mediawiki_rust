use super::{
    ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData, ActionApiRunnable,
    NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

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

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiQueryCategoryinfoBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryCategoryinfoData,
}

impl<T> ActionApiQueryCategoryinfoBuilder<T> {
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
        }
    }
}

impl ActionApiRunnable for ActionApiQueryCategoryinfoBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "categoryinfo".to_string());
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
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::categoryinfo()
            .titles(&["Category:Physics"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
