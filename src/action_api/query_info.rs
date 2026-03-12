use super::{ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};
use strum::{Display, EnumString};

/// Level of detail to return for tested actions in `prop=info` (`intestactionsdetail`).
#[derive(EnumString, Display, Debug, Clone, Copy)]
pub enum IntestactionsDetail {
    #[strum(to_string = "boolean")]
    Boolean,
    #[strum(to_string = "full")]
    Full,
    #[strum(to_string = "quick")]
    Quick,
}

/// Style of the edit intro to display (`ineditintrostyle`).
#[derive(EnumString, Display, Debug, Clone, Copy, Default, PartialEq)]
pub enum IneditIntroStyle {
    #[strum(to_string = "lessframes")]
    LessFrames,
    #[default]
    #[strum(to_string = "moreframes")]
    MoreFrames,
}

/// Internal data container for `prop=info` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryInfoData {
    common: ActionApiQueryCommonData,
    inprop: Option<Vec<String>>,
    inlinkcontext: Option<String>,
    intestactions: Option<Vec<String>>,
    intestactionsdetail: Option<IntestactionsDetail>,
    indefaultlinkcaption: bool,
    intestactionsautocreate: bool,
    inpreloadcustom: Option<String>,
    inpreloadparams: Option<Vec<String>>,
    inpreloadnewsection: bool,
    ineditintrostyle: IneditIntroStyle,
    ineditintroskip: Option<Vec<String>>,
    ineditintrocustom: Option<String>,
    incontinue: Option<String>, // Internal use only, not to be set by user!
}

impl ActionApiData for ActionApiQueryInfoData {}

impl ActionApiQueryInfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.inprop, "inprop", &mut params);
        if let Some(inlinkcontext) = &self.inlinkcontext {
            params.insert("inlinkcontext".to_string(), inlinkcontext.clone());
        }
        Self::add_vec(&self.intestactions, "intestactions", &mut params);
        if let Some(intestactionsdetail) = &self.intestactionsdetail {
            params.insert(
                "intestactionsdetail".to_string(),
                intestactionsdetail.to_string(),
            );
        }
        Self::add_boolean(
            self.indefaultlinkcaption,
            "indefaultlinkcaption",
            &mut params,
        );
        Self::add_boolean(
            self.intestactionsautocreate,
            "intestactionsautocreate",
            &mut params,
        );
        Self::add_str(&self.inpreloadcustom, "inpreloadcustom", &mut params);
        Self::add_vec(&self.inpreloadparams, "inpreloadparams", &mut params);
        Self::add_boolean(self.inpreloadnewsection, "inpreloadnewsection", &mut params);
        if self.ineditintrostyle != IneditIntroStyle::default() {
            params.insert(
                "ineditintrostyle".to_string(),
                self.ineditintrostyle.to_string(),
            );
        }
        Self::add_vec(&self.ineditintroskip, "ineditintroskip", &mut params);
        Self::add_str(&self.ineditintrocustom, "ineditintrocustom", &mut params);
        Self::add_str(&self.incontinue, "incontinue", &mut params);
        params
    }
}

/// Builder for the `prop=info` query module.
///
/// Starts in `NoTitlesOrGenerator` state and becomes `Runnable` after titles, pageids, revids,
/// or a generator is set via `ActionApiQueryCommonBuilder`.
#[derive(Debug, Clone)]
pub struct ActionApiQueryInfoBuilder<T> {
    _phantom: PhantomData<T>,
    data: ActionApiQueryInfoData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryInfoBuilder<T> {
    /// Which additional properties to retrieve for each page (`inprop`).
    pub fn inprop<S: Into<String> + Clone>(mut self, inprop: &[S]) -> Self {
        self.data.inprop = Some(inprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Title of the page used as the context when determining link CSS classes (`inlinkcontext`).
    pub fn inlinkcontext<S: AsRef<str>>(mut self, inlinkcontext: S) -> Self {
        self.data.inlinkcontext = Some(inlinkcontext.as_ref().to_string());
        self
    }

    /// Test whether the current user can perform these actions on each page (`intestactions`).
    pub fn intestactions<S: Into<String> + Clone>(mut self, intestactions: &[S]) -> Self {
        self.data.intestactions = Some(intestactions.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Level of detail to return for each tested action (`intestactionsdetail`).
    pub fn intestactionsdetail(mut self, intestactionsdetail: IntestactionsDetail) -> Self {
        self.data.intestactionsdetail = Some(intestactionsdetail);
        self
    }
}

impl ActionApiQueryInfoBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryInfoData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryInfoBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryInfoBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryInfoBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryInfoBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "info".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryInfoBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryInfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryInfoBuilder::new()
    }

    // --- Default state ---

    #[test]
    fn default_no_titles() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("pageids"));
    }

    #[test]
    fn default_no_pageids() {
        let params = new_builder().pageids(&[42]).data.params();
        assert!(!params.contains_key("titles"));
    }

    #[test]
    fn default_inprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("inprop"));
    }

    #[test]
    fn default_inlinkcontext_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("inlinkcontext"));
    }

    #[test]
    fn default_intestactions_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("intestactions"));
    }

    #[test]
    fn default_intestactionsdetail_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("intestactionsdetail"));
    }

    // --- titles() ---

    #[test]
    fn titles_single() {
        let params = new_builder().titles(&["Albert Einstein"]).data.params();
        assert_eq!(params["titles"], "Albert Einstein");
    }

    #[test]
    fn titles_multiple() {
        let params = new_builder()
            .titles(&["Albert Einstein", "Marie Curie", "Newton"])
            .data
            .params();
        assert_eq!(params["titles"], "Albert Einstein|Marie Curie|Newton");
    }

    #[test]
    fn titles_does_not_set_pageids() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("pageids"));
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

    // --- inprop() ---

    #[test]
    fn inprop_single() {
        let params = new_builder()
            .inprop(&["protection"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["inprop"], "protection");
    }

    #[test]
    fn inprop_multiple() {
        let params = new_builder()
            .inprop(&["protection", "url", "displaytitle"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["inprop"], "protection|url|displaytitle");
    }

    #[test]
    fn inprop_accepts_owned_strings() {
        let props = vec!["url".to_string(), "talkid".to_string()];
        let params = new_builder().inprop(&props).titles(&["Foo"]).data.params();
        assert_eq!(params["inprop"], "url|talkid");
    }

    // --- inlinkcontext() ---

    #[test]
    fn inlinkcontext_set() {
        let params = new_builder()
            .inlinkcontext("Main Page")
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["inlinkcontext"], "Main Page");
    }

    // --- intestactions() ---

    #[test]
    fn intestactions_single() {
        let params = new_builder()
            .intestactions(&["edit"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["intestactions"], "edit");
    }

    #[test]
    fn intestactions_multiple() {
        let params = new_builder()
            .intestactions(&["edit", "move", "delete"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["intestactions"], "edit|move|delete");
    }

    // --- intestactionsdetail() ---

    #[test]
    fn intestactionsdetail_boolean() {
        let params = new_builder()
            .intestactionsdetail(IntestactionsDetail::Boolean)
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["intestactionsdetail"], "boolean");
    }

    #[test]
    fn intestactionsdetail_full() {
        let params = new_builder()
            .intestactionsdetail(IntestactionsDetail::Full)
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["intestactionsdetail"], "full");
    }

    // --- ActionApiRunnable::params() ---

    #[test]
    fn runnable_params_contain_action_query() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "info");
    }

    #[test]
    fn runnable_params_contain_titles() {
        let builder = new_builder().titles(&["Bar"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["titles"], "Bar");
    }

    // --- Builder chaining ---

    #[test]
    fn chaining_all_options() {
        let builder = new_builder()
            .inprop(&["protection", "url"])
            .inlinkcontext("Main Page")
            .intestactions(&["edit"])
            .intestactionsdetail(IntestactionsDetail::Full)
            .titles(&["Albert Einstein"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "info");
        assert_eq!(params["titles"], "Albert Einstein");
        assert_eq!(params["inprop"], "protection|url");
        assert_eq!(params["inlinkcontext"], "Main Page");
        assert_eq!(params["intestactions"], "edit");
        assert_eq!(params["intestactionsdetail"], "full");
    }

    // --- Integration test ---

    #[tokio::test]
    async fn test_info_by_title() {
        use wiremock::{Mock, ResponseTemplate};
        use wiremock::matchers::query_param;
        let server = crate::test_helpers::test_helpers::start_enwiki_mock().await;
        Mock::given(query_param("prop", "info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "736": {
                            "pageid": 736, "ns": 0, "title": "Albert Einstein",
                            "contentmodel": "wikitext", "pagelanguage": "en",
                            "touched": "2024-01-01T00:00:00Z", "lastrevid": 1001,
                            "length": 12345
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::info()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["pages"].is_object());
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }

    #[tokio::test]
    async fn test_info_by_title_with_inprop() {
        use wiremock::{Mock, ResponseTemplate};
        use wiremock::matchers::query_param;
        let server = crate::test_helpers::test_helpers::start_enwiki_mock().await;
        Mock::given(query_param("prop", "info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "736": {
                            "pageid": 736, "ns": 0, "title": "Albert Einstein",
                            "fullurl": "https://en.wikipedia.org/wiki/Albert_Einstein",
                            "editurl": "https://en.wikipedia.org/w/index.php?title=Albert_Einstein&action=edit",
                            "protection": [{"type": "edit", "level": "autoconfirmed"}]
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::info()
            .titles(&["Albert Einstein"])
            .inprop(&["protection", "url"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        let page = pages.values().next().unwrap();
        assert!(page["fullurl"].is_string());
    }

    #[tokio::test]
    async fn test_info_by_pageid() {
        use wiremock::{Mock, ResponseTemplate};
        use wiremock::matchers::query_param;
        let server = crate::test_helpers::test_helpers::start_enwiki_mock().await;
        Mock::given(query_param("prop", "info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "736": {
                            "pageid": 736, "ns": 0, "title": "Albert Einstein",
                            "contentmodel": "wikitext"
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::info()
            .pageids(&[736])
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["pages"]["736"].is_object());
    }
}
