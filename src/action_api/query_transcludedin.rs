use super::{
    ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder,
    ActionApiQueryCommonData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=transcludedin` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiQueryTranscludedinData {
    common: ActionApiQueryCommonData,
    tiprop: Option<Vec<String>>,
    tinamespace: Option<Vec<NamespaceID>>,
    tishow: Option<Vec<String>>,
    tilimit: usize,
    ticontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryTranscludedinData {}

impl Default for ActionApiQueryTranscludedinData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            tiprop: None,
            tinamespace: None,
            tishow: None,
            tilimit: 10,
            ticontinue: None,
        }
    }
}

impl ActionApiQueryTranscludedinData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.tiprop, "tiprop", &mut params);
        if let Some(ns) = &self.tinamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("tinamespace".to_string(), s.join("|"));
        }
        Self::add_vec(&self.tishow, "tishow", &mut params);
        params.insert("tilimit".to_string(), self.tilimit.to_string());
        Self::add_str(&self.ticontinue, "ticontinue", &mut params);
        params
    }
}

/// Builder for the `prop=transcludedin` query module; uses the typestate pattern, starting in
/// `NoTitlesOrGenerator` and becoming `Runnable` once titles/pageids/revids/generator is set via `ActionApiQueryCommonBuilder`.
#[derive(Debug, Clone)]
pub struct ActionApiQueryTranscludedinBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryTranscludedinData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryTranscludedinBuilder<T> {
    /// Which properties to retrieve for each transcluding page (`tiprop`).
    pub fn tiprop<S: Into<String> + Clone>(mut self, tiprop: &[S]) -> Self {
        self.data.tiprop = Some(tiprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Only include pages that transclude the target from these namespaces (`tinamespace`).
    pub fn tinamespace(mut self, tinamespace: &[NamespaceID]) -> Self {
        self.data.tinamespace = Some(tinamespace.to_vec());
        self
    }

    /// Filter transcluding pages to show only those matching these criteria, e.g. `redirect` or `!redirect` (`tishow`).
    pub fn tishow<S: Into<String> + Clone>(mut self, tishow: &[S]) -> Self {
        self.data.tishow = Some(tishow.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of transcluding pages to return (`tilimit`).
    pub fn tilimit(mut self, tilimit: usize) -> Self {
        self.data.tilimit = tilimit;
        self
    }
}

impl ActionApiQueryTranscludedinBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryTranscludedinData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiQueryTranscludedinBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "transcludedin".to_string());
        params
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryTranscludedinBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryTranscludedinBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryTranscludedinBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryTranscludedinBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "transcludedin".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryTranscludedinBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryTranscludedinBuilder<NoTitlesOrGenerator> {
        ActionApiQueryTranscludedinBuilder::new()
    }

    #[test]
    fn default_tilimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["tilimit"], "10");
    }

    #[test]
    fn default_tiprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("tiprop"));
    }

    #[test]
    fn tiprop_set() {
        let params = new_builder()
            .tiprop(&["pageid", "title", "redirect"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["tiprop"], "pageid|title|redirect");
    }

    #[test]
    fn tinamespace_set() {
        let params = new_builder()
            .tinamespace(&[0, 4])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["tinamespace"], "0|4");
    }

    #[test]
    fn tishow_redirect_only() {
        let params = new_builder()
            .tishow(&["!redirect"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["tishow"], "!redirect");
    }

    #[test]
    fn tilimit_set() {
        let params = new_builder().tilimit(50).titles(&["Foo"]).data.params();
        assert_eq!(params["tilimit"], "50");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "transcludedin");
    }

    #[tokio::test]
    async fn test_transcludedin() {
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("prop", "transcludedin"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "100": {
                            "pageid": 100, "ns": 10, "title": "Template:Infobox scientist",
                            "transcludedin": [
                                {"pageid": 736, "ns": 0, "title": "Albert Einstein"},
                                {"pageid": 500, "ns": 0, "title": "Marie Curie"}
                            ]
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::transcludedin()
            .titles(&["Template:Infobox scientist"])
            .tinamespace(&[0])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
