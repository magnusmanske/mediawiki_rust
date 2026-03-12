use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `action=compare` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiCompareData {
    fromtitle: Option<String>,
    fromid: Option<u64>,
    fromrev: Option<u64>,
    fromtext: Option<String>,
    fromcontentmodel: Option<String>,
    totitle: Option<String>,
    toid: Option<u64>,
    torev: Option<u64>,
    torelative: Option<String>,
    totext: Option<String>,
    tocontentmodel: Option<String>,
    prop: Option<Vec<String>>,
    difftype: Option<String>,
}

impl ActionApiData for ActionApiCompareData {}

impl ActionApiCompareData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "compare".to_string());
        Self::add_str(&self.fromtitle, "fromtitle", &mut params);
        if let Some(v) = self.fromid {
            params.insert("fromid".to_string(), v.to_string());
        }
        if let Some(v) = self.fromrev {
            params.insert("fromrev".to_string(), v.to_string());
        }
        Self::add_str(&self.fromtext, "fromtext", &mut params);
        Self::add_str(&self.fromcontentmodel, "fromcontentmodel", &mut params);
        Self::add_str(&self.totitle, "totitle", &mut params);
        if let Some(v) = self.toid {
            params.insert("toid".to_string(), v.to_string());
        }
        if let Some(v) = self.torev {
            params.insert("torev".to_string(), v.to_string());
        }
        Self::add_str(&self.torelative, "torelative", &mut params);
        Self::add_str(&self.totext, "totext", &mut params);
        Self::add_str(&self.tocontentmodel, "tocontentmodel", &mut params);
        Self::add_vec(&self.prop, "prop", &mut params);
        Self::add_str(&self.difftype, "difftype", &mut params);
        params
    }
}

/// Builder for `action=compare`. Set source and target revisions/titles/text, then call `.run()`.
#[derive(Debug, Clone)]
pub struct ActionApiCompareBuilder {
    pub(crate) data: ActionApiCompareData,
}

impl ActionApiCompareBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            data: ActionApiCompareData::default(),
        }
    }

    /// Title of the first page to compare (`fromtitle`).
    pub fn fromtitle<S: AsRef<str>>(mut self, fromtitle: S) -> Self {
        self.data.fromtitle = Some(fromtitle.as_ref().to_string());
        self
    }

    /// Page ID of the first page to compare (`fromid`).
    pub fn fromid(mut self, fromid: u64) -> Self {
        self.data.fromid = Some(fromid);
        self
    }

    /// Revision ID of the first revision to compare (`fromrev`).
    pub fn fromrev(mut self, fromrev: u64) -> Self {
        self.data.fromrev = Some(fromrev);
        self
    }

    /// Wikitext to use as the first content to compare (`fromtext`).
    pub fn fromtext<S: AsRef<str>>(mut self, fromtext: S) -> Self {
        self.data.fromtext = Some(fromtext.as_ref().to_string());
        self
    }

    /// Content model of the first content (`fromcontentmodel`).
    pub fn fromcontentmodel<S: AsRef<str>>(mut self, fromcontentmodel: S) -> Self {
        self.data.fromcontentmodel = Some(fromcontentmodel.as_ref().to_string());
        self
    }

    /// Title of the second page to compare (`totitle`).
    pub fn totitle<S: AsRef<str>>(mut self, totitle: S) -> Self {
        self.data.totitle = Some(totitle.as_ref().to_string());
        self
    }

    /// Page ID of the second page to compare (`toid`).
    pub fn toid(mut self, toid: u64) -> Self {
        self.data.toid = Some(toid);
        self
    }

    /// Revision ID of the second revision to compare (`torev`).
    pub fn torev(mut self, torev: u64) -> Self {
        self.data.torev = Some(torev);
        self
    }

    /// Use a revision relative to the from-revision, e.g. `"prev"` or `"next"` (`torelative`).
    pub fn torelative<S: AsRef<str>>(mut self, torelative: S) -> Self {
        self.data.torelative = Some(torelative.as_ref().to_string());
        self
    }

    /// Wikitext to use as the second content to compare (`totext`).
    pub fn totext<S: AsRef<str>>(mut self, totext: S) -> Self {
        self.data.totext = Some(totext.as_ref().to_string());
        self
    }

    /// Content model of the second content (`tocontentmodel`).
    pub fn tocontentmodel<S: AsRef<str>>(mut self, tocontentmodel: S) -> Self {
        self.data.tocontentmodel = Some(tocontentmodel.as_ref().to_string());
        self
    }

    /// Properties to include in the diff output (`prop`).
    pub fn prop<S: Into<String> + Clone>(mut self, prop: &[S]) -> Self {
        self.data.prop = Some(prop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Format in which to return the diff, e.g. `"table"` or `"inline"` (`difftype`).
    pub fn difftype<S: AsRef<str>>(mut self, difftype: S) -> Self {
        self.data.difftype = Some(difftype.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiCompareBuilder {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApi};

    #[test]
    fn fromrev_set() {
        let params = ActionApiCompareBuilder::new()
            .fromrev(1)
            .torev(2)
            .data
            .params();
        assert_eq!(params["fromrev"], "1");
        assert_eq!(params["torev"], "2");
    }

    #[test]
    fn fromtitle_totitle() {
        let params = ActionApiCompareBuilder::new()
            .fromtitle("Foo")
            .totitle("Bar")
            .data
            .params();
        assert_eq!(params["fromtitle"], "Foo");
        assert_eq!(params["totitle"], "Bar");
    }

    #[test]
    fn prop_set() {
        let params = ActionApiCompareBuilder::new()
            .fromrev(1)
            .torev(2)
            .prop(&["diff", "ids"])
            .data
            .params();
        assert_eq!(params["prop"], "diff|ids");
    }

    #[test]
    fn torelative_prev() {
        let params = ActionApiCompareBuilder::new()
            .fromrev(100)
            .torelative("prev")
            .data
            .params();
        assert_eq!(params["torelative"], "prev");
    }

    #[test]
    fn difftype_set() {
        let params = ActionApiCompareBuilder::new()
            .fromrev(1)
            .torev(2)
            .difftype("inline")
            .data
            .params();
        assert_eq!(params["difftype"], "inline");
    }

    #[test]
    fn action_is_compare() {
        let params = ActionApiCompareBuilder::new().data.params();
        assert_eq!(params["action"], "compare");
    }

    #[tokio::test]
    async fn test_compare() {
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("action", "compare"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "compare": {
                    "fromid": 736, "fromrevid": 1000, "fromns": 0, "fromtitle": "Albert Einstein",
                    "toid": 736, "torevid": 999, "tons": 0, "totitle": "Albert Einstein",
                    "body": "<tr><td>diff content</td></tr>"
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApi::compare()
            .fromtitle("Albert Einstein")
            .torelative("prev")
            .prop(&["diff", "ids"])
            .run(&api)
            .await
            .unwrap();
        assert!(result["compare"].is_object());
    }
}
