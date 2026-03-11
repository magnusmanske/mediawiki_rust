use std::{collections::HashMap, marker::PhantomData};

use crate::{
    action_api::{
        ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder,
        ActionApiQueryCommonData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
    },
    api::NamespaceID,
};

/// Internal data container for `prop=linkshere` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiQueryLinkshereData {
    common: ActionApiQueryCommonData,
    lhprop: Option<Vec<String>>,
    lhnamespace: Option<Vec<NamespaceID>>,
    lhshow: Option<Vec<String>>,
    lhlimit: usize,
    lhcontinue: Option<String>,
}

impl Default for ActionApiQueryLinkshereData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            lhprop: None,
            lhnamespace: None,
            lhshow: None,
            lhlimit: 10,
            lhcontinue: None,
        }
    }
}

impl ActionApiData for ActionApiQueryLinkshereData {}

impl ActionApiQueryLinkshereData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        if let Some(lhnamespace) = &self.lhnamespace {
            let lhnamespace: Vec<String> = lhnamespace.iter().map(|n| n.to_string()).collect();
            params.insert("lhnamespace".to_string(), lhnamespace.join("|"));
        }
        Self::add_vec(&self.lhprop, "lhprop", &mut params);
        Self::add_vec(&self.lhshow, "lhshow", &mut params);
        params.insert("lhlimit".to_string(), self.lhlimit.to_string());
        Self::add_str(&self.lhcontinue, "lhcontinue", &mut params);
        params
    }
}

/// Builder for the `prop=linkshere` query module; uses the typestate pattern, starting in
/// `NoTitlesOrGenerator` and becoming `Runnable` once titles/pageids/revids/generator is set via `ActionApiQueryCommonBuilder`.
#[derive(Debug, Clone)]
pub struct ActionApiQueryLinkshereBuilder<T> {
    _phantom: PhantomData<T>,
    data: ActionApiQueryLinkshereData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiGenerator for ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "linkshere".to_string());
        params
    }
}

impl<T> ActionApiQueryLinkshereBuilder<T> {
    /// Which properties to retrieve for each linking page (`lhprop`).
    pub fn lhprop<S: Into<String> + Clone>(mut self, lhprop: &[S]) -> Self {
        self.data.lhprop = Some(
            lhprop
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<String>>(),
        );
        self
    }

    /// Only include pages in the given namespaces (`lhnamespace`).
    pub fn lhnamespace(mut self, lhnamespace: &[NamespaceID]) -> Self {
        self.data.lhnamespace = Some(lhnamespace.to_vec());
        self
    }

    /// Filter results to show only pages matching these criteria, e.g. `redirect` or `!redirect` (`lhshow`).
    pub fn lhshow<S: Into<String> + Clone>(mut self, lhshow: &[S]) -> Self {
        self.data.lhshow = Some(
            lhshow
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<String>>(),
        );
        self
    }

    /// Maximum number of linking pages to return (`lhlimit`).
    pub fn lhlimit(mut self, lhlimit: usize) -> Self {
        self.data.lhlimit = lhlimit;
        self
    }
}

impl ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinkshereBuilder {
            _phantom: PhantomData,
            data: ActionApiQueryLinkshereData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryLinkshereBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryLinkshereBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryLinkshereBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "linkshere".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryLinkshereBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Api,
        action_api::{ActionApiQuery, ActionApiQueryCommonBuilder},
    };

    fn new_builder() -> ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinkshereBuilder::new()
    }

    // --- pageids() ---

    #[test]
    fn pageids_single() {
        let params = new_builder().pageids(&[736]).data.params();
        assert_eq!(params["pageids"], "736");
    }

    #[test]
    fn pageids_multiple() {
        let params = new_builder().pageids(&[1, 2, 3]).data.params();
        assert_eq!(params["pageids"], "1|2|3");
    }

    #[test]
    fn pageids_does_not_set_titles() {
        let params = new_builder().pageids(&[42]).data.params();
        assert!(!params.contains_key("titles"));
    }

    #[test]
    fn titles_does_not_set_pageids() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("pageids"));
    }

    // --- revids() ---

    #[test]
    fn revids_single() {
        let params = new_builder().revids(&[12345]).data.params();
        assert_eq!(params["revids"], "12345");
    }

    #[test]
    fn revids_multiple() {
        let params = new_builder().revids(&[1, 2, 3]).data.params();
        assert_eq!(params["revids"], "1|2|3");
    }

    #[test]
    fn revids_does_not_set_titles() {
        let params = new_builder().revids(&[12345]).data.params();
        assert!(!params.contains_key("titles"));
    }

    #[test]
    fn revids_does_not_set_pageids() {
        let params = new_builder().revids(&[12345]).data.params();
        assert!(!params.contains_key("pageids"));
    }

    #[tokio::test]
    async fn test_linkshere() {
        let api = Api::new("https://en.wikipedia.org/w/api.php")
            .await
            .unwrap();
        let result = ActionApiQuery::linkshere()
            .titles(&["Magnus Manske"])
            .lhnamespace(&[0])
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["pages"]["3361346"]["linkshere"].is_array())
    }
}
