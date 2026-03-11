use super::{
    ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder,
    ActionApiQueryCommonData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=templates` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiQueryTemplatesData {
    common: ActionApiQueryCommonData,
    tlnamespace: Option<Vec<NamespaceID>>,
    tllimit: usize,
    tlcontinue: Option<String>,
    tltemplates: Option<Vec<String>>,
    tldir: Option<String>,
}

impl ActionApiData for ActionApiQueryTemplatesData {}

impl Default for ActionApiQueryTemplatesData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            tlnamespace: None,
            tllimit: 10,
            tlcontinue: None,
            tltemplates: None,
            tldir: None,
        }
    }
}

impl ActionApiQueryTemplatesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        if let Some(ns) = &self.tlnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("tlnamespace".to_string(), s.join("|"));
        }
        params.insert("tllimit".to_string(), self.tllimit.to_string());
        Self::add_str(&self.tlcontinue, "tlcontinue", &mut params);
        Self::add_vec(&self.tltemplates, "tltemplates", &mut params);
        Self::add_str(&self.tldir, "tldir", &mut params);
        params
    }
}

/// Builder for the `prop=templates` query module; uses the typestate pattern, starting in
/// `NoTitlesOrGenerator` and becoming `Runnable` once titles/pageids/revids/generator is set via `ActionApiQueryCommonBuilder`.
#[derive(Debug, Clone)]
pub struct ActionApiQueryTemplatesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryTemplatesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryTemplatesBuilder<T> {
    /// Filter templates to only those in the given namespaces (`tlnamespace`).
    pub fn tlnamespace(mut self, tlnamespace: &[NamespaceID]) -> Self {
        self.data.tlnamespace = Some(tlnamespace.to_vec());
        self
    }

    /// Maximum number of templates to return (`tllimit`).
    pub fn tllimit(mut self, tllimit: usize) -> Self {
        self.data.tllimit = tllimit;
        self
    }

    /// Only list these specific templates (`tltemplates`).
    pub fn tltemplates<S: Into<String> + Clone>(mut self, tltemplates: &[S]) -> Self {
        self.data.tltemplates = Some(tltemplates.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Direction to list templates in, either `ascending` or `descending` (`tldir`).
    pub fn tldir<S: AsRef<str>>(mut self, tldir: S) -> Self {
        self.data.tldir = Some(tldir.as_ref().to_string());
        self
    }
}

impl ActionApiQueryTemplatesBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryTemplatesData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiQueryTemplatesBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "templates".to_string());
        params
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryTemplatesBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryTemplatesBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryTemplatesBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryTemplatesBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "templates".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryTemplatesBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryTemplatesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryTemplatesBuilder::new()
    }

    #[test]
    fn default_tllimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["tllimit"], "10");
    }

    #[test]
    fn default_tlnamespace_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("tlnamespace"));
    }

    #[test]
    fn tlnamespace_set() {
        let params = new_builder().tlnamespace(&[10]).titles(&["Foo"]).data.params();
        assert_eq!(params["tlnamespace"], "10");
    }

    #[test]
    fn tllimit_set() {
        let params = new_builder().tllimit(50).titles(&["Foo"]).data.params();
        assert_eq!(params["tllimit"], "50");
    }

    #[test]
    fn tltemplates_filter() {
        let params = new_builder()
            .tltemplates(&["Template:Infobox", "Template:Cite"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["tltemplates"], "Template:Infobox|Template:Cite");
    }

    #[test]
    fn tldir_descending() {
        let params = new_builder().tldir("descending").titles(&["Foo"]).data.params();
        assert_eq!(params["tldir"], "descending");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "templates");
    }

    #[tokio::test]
    async fn test_templates() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::templates()
            .titles(&["Albert Einstein"])
            .tlnamespace(&[10])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
