use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub(crate) type NoText = super::NoTitlesOrGenerator;

/// Internal data container for `action=expandtemplates` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiExpandtemplatesData {
    title: Option<String>,
    text: Option<String>,
    revid: Option<u64>,
    prop: Option<Vec<String>>,
    includecomments: bool,
    templatesandboxprefix: Option<Vec<String>>,
    templatesandboxtitle: Option<String>,
    templatesandboxtext: Option<String>,
    templatesandboxcontentmodel: Option<String>,
    templatesandboxcontentformat: Option<String>,
}

impl ActionApiData for ActionApiExpandtemplatesData {}

impl ActionApiExpandtemplatesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "expandtemplates".to_string());
        Self::add_str(&self.title, "title", &mut params);
        Self::add_str(&self.text, "text", &mut params);
        if let Some(revid) = self.revid {
            params.insert("revid".to_string(), revid.to_string());
        }
        Self::add_vec(&self.prop, "prop", &mut params);
        Self::add_boolean(self.includecomments, "includecomments", &mut params);
        Self::add_vec(
            &self.templatesandboxprefix,
            "templatesandboxprefix",
            &mut params,
        );
        Self::add_str(
            &self.templatesandboxtitle,
            "templatesandboxtitle",
            &mut params,
        );
        Self::add_str(
            &self.templatesandboxtext,
            "templatesandboxtext",
            &mut params,
        );
        Self::add_str(
            &self.templatesandboxcontentmodel,
            "templatesandboxcontentmodel",
            &mut params,
        );
        Self::add_str(
            &self.templatesandboxcontentformat,
            "templatesandboxcontentformat",
            &mut params,
        );
        params
    }
}

/// Builder for `action=expandtemplates`. Call `.text()` to set the wikitext to expand, making it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiExpandtemplatesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiExpandtemplatesData,
}

impl<T> ActionApiExpandtemplatesBuilder<T> {
    /// Page title used as context for template expansion (`title`).
    pub fn title<S: AsRef<str>>(mut self, title: S) -> Self {
        self.data.title = Some(title.as_ref().to_string());
        self
    }

    /// Revision ID used to set `{{REVISIONID}}` and similar variables (`revid`).
    pub fn revid(mut self, revid: u64) -> Self {
        self.data.revid = Some(revid);
        self
    }

    /// Output properties to include in the response (`prop`).
    pub fn prop<S: Into<String> + Clone>(mut self, prop: &[S]) -> Self {
        self.data.prop = Some(prop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Whether to include HTML comments in the output (`includecomments`).
    pub fn includecomments(mut self, includecomments: bool) -> Self {
        self.data.includecomments = includecomments;
        self
    }
}

impl ActionApiExpandtemplatesBuilder<NoText> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiExpandtemplatesData::default(),
        }
    }

    /// Wikitext to expand (`text`).
    pub fn text<S: AsRef<str>>(mut self, text: S) -> ActionApiExpandtemplatesBuilder<Runnable> {
        self.data.text = Some(text.as_ref().to_string());
        ActionApiExpandtemplatesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiExpandtemplatesBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApi};

    fn new_builder() -> ActionApiExpandtemplatesBuilder<NoText> {
        ActionApiExpandtemplatesBuilder::new()
    }

    #[test]
    fn text_set() {
        let params = new_builder().text("{{SITENAME}}").data.params();
        assert_eq!(params["text"], "{{SITENAME}}");
    }

    #[test]
    fn title_set() {
        let params = new_builder().title("Test").text("{{foo}}").data.params();
        assert_eq!(params["title"], "Test");
    }

    #[test]
    fn prop_set() {
        let params = new_builder()
            .prop(&["wikitext"])
            .text("{{foo}}")
            .data
            .params();
        assert_eq!(params["prop"], "wikitext");
    }

    #[test]
    fn includecomments_set() {
        let params = new_builder()
            .includecomments(true)
            .text("{{foo}}")
            .data
            .params();
        assert!(params.contains_key("includecomments"));
    }

    #[test]
    fn action_is_expandtemplates() {
        let params = new_builder().text("{{foo}}").data.params();
        assert_eq!(params["action"], "expandtemplates");
    }

    #[tokio::test]
    async fn test_expandtemplates() {
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("action", "expandtemplates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "expandtemplates": {
                    "wikitext": "Wikipedia"
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApi::expandtemplates()
            .text("{{SITENAME}}")
            .prop(&["wikitext"])
            .run(&api)
            .await
            .unwrap();
        assert!(result["expandtemplates"]["wikitext"].is_string());
    }
}
