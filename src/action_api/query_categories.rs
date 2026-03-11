use super::{
    ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder,
    ActionApiQueryCommonData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryCategoriesData {
    common: ActionApiQueryCommonData,
    clprop: Option<Vec<String>>,
    clshow: Option<Vec<String>>,
    cllimit: usize,
    clcontinue: Option<String>,
    clcategories: Option<Vec<String>>,
    cldir: Option<String>,
    clnamespace: Option<Vec<NamespaceID>>,
}

impl ActionApiData for ActionApiQueryCategoriesData {}

impl Default for ActionApiQueryCategoriesData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            clprop: None,
            clshow: None,
            cllimit: 10,
            clcontinue: None,
            clcategories: None,
            cldir: None,
            clnamespace: None,
        }
    }
}

impl ActionApiQueryCategoriesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.clprop, "clprop", &mut params);
        Self::add_vec(&self.clshow, "clshow", &mut params);
        params.insert("cllimit".to_string(), self.cllimit.to_string());
        Self::add_str(&self.clcontinue, "clcontinue", &mut params);
        Self::add_vec(&self.clcategories, "clcategories", &mut params);
        Self::add_str(&self.cldir, "cldir", &mut params);
        if let Some(ns) = &self.clnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("clnamespace".to_string(), s.join("|"));
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiQueryCategoriesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryCategoriesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryCategoriesBuilder<T> {
    pub fn clprop<S: Into<String> + Clone>(mut self, clprop: &[S]) -> Self {
        self.data.clprop = Some(clprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn clshow<S: Into<String> + Clone>(mut self, clshow: &[S]) -> Self {
        self.data.clshow = Some(clshow.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn cllimit(mut self, cllimit: usize) -> Self {
        self.data.cllimit = cllimit;
        self
    }

    pub fn clcategories<S: Into<String> + Clone>(mut self, clcategories: &[S]) -> Self {
        self.data.clcategories = Some(clcategories.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn cldir<S: AsRef<str>>(mut self, cldir: S) -> Self {
        self.data.cldir = Some(cldir.as_ref().to_string());
        self
    }

    pub fn clnamespace(mut self, clnamespace: &[NamespaceID]) -> Self {
        self.data.clnamespace = Some(clnamespace.to_vec());
        self
    }
}

impl ActionApiQueryCategoriesBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryCategoriesData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiQueryCategoriesBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "categories".to_string());
        params
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryCategoriesBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryCategoriesBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryCategoriesBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryCategoriesBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "categories".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryCategoriesBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryCategoriesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryCategoriesBuilder::new()
    }

    #[test]
    fn default_cllimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["cllimit"], "10");
    }

    #[test]
    fn default_clprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("clprop"));
    }

    #[test]
    fn default_clshow_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("clshow"));
    }

    #[test]
    fn clprop_single() {
        let params = new_builder().clprop(&["sortkey"]).titles(&["Foo"]).data.params();
        assert_eq!(params["clprop"], "sortkey");
    }

    #[test]
    fn clprop_multiple() {
        let params = new_builder()
            .clprop(&["sortkey", "timestamp", "hidden"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["clprop"], "sortkey|timestamp|hidden");
    }

    #[test]
    fn clshow_set() {
        let params = new_builder().clshow(&["!hidden"]).titles(&["Foo"]).data.params();
        assert_eq!(params["clshow"], "!hidden");
    }

    #[test]
    fn cllimit_set() {
        let params = new_builder().cllimit(50).titles(&["Foo"]).data.params();
        assert_eq!(params["cllimit"], "50");
    }

    #[test]
    fn clcategories_filter() {
        let params = new_builder()
            .clcategories(&["Category:Foo", "Category:Bar"])
            .titles(&["Baz"])
            .data
            .params();
        assert_eq!(params["clcategories"], "Category:Foo|Category:Bar");
    }

    #[test]
    fn cldir_descending() {
        let params = new_builder().cldir("descending").titles(&["Foo"]).data.params();
        assert_eq!(params["cldir"], "descending");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "categories");
    }

    #[tokio::test]
    async fn test_categories() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::categories()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
