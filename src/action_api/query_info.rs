use std::{collections::HashMap, marker::PhantomData};

use crate::action_api::{ActionApiData, ActionApiRunnable};

#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryInfoData {
    titles: Option<Vec<String>>,
    pageids: Option<Vec<u64>>,
    inprop: Option<Vec<String>>,
    inlinkcontext: Option<String>,
    intestactions: Option<Vec<String>>,
    intestactionsdetail: Option<String>,
    indefaultlinkcaption: Option<bool>,
}

impl ActionApiData for ActionApiQueryInfoData {}

impl ActionApiQueryInfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.titles, "titles", &mut params);
        if let Some(pageids) = &self.pageids {
            let pageids: Vec<String> = pageids.iter().map(|id| id.to_string()).collect();
            params.insert("pageids".to_string(), pageids.join("|"));
        }
        Self::add_vec(&self.inprop, "inprop", &mut params);
        if let Some(inlinkcontext) = &self.inlinkcontext {
            params.insert("inlinkcontext".to_string(), inlinkcontext.clone());
        }
        Self::add_vec(&self.intestactions, "intestactions", &mut params);
        if let Some(intestactionsdetail) = &self.intestactionsdetail {
            params.insert(
                "intestactionsdetail".to_string(),
                intestactionsdetail.clone(),
            );
        }
        if let Some(indefaultlinkcaption) = &self.indefaultlinkcaption {
            Self::add_boolean(*indefaultlinkcaption, "indefaultlinkcaption", &mut params);
        }
        params
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NoTitlesOrPageids;

#[derive(Debug, Copy, Clone)]
pub struct Runnable;

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct ActionApiQueryInfoBuilder<T> {
    _phantom: PhantomData<T>,
    data: ActionApiQueryInfoData,
}

impl<T> ActionApiQueryInfoBuilder<T> {
    pub fn inprop<S: Into<String> + Clone>(mut self, inprop: &[S]) -> Self {
        self.data.inprop = Some(inprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn inlinkcontext<S: AsRef<str>>(mut self, inlinkcontext: S) -> Self {
        self.data.inlinkcontext = Some(inlinkcontext.as_ref().to_string());
        self
    }

    pub fn intestactions<S: Into<String> + Clone>(mut self, intestactions: &[S]) -> Self {
        self.data.intestactions = Some(intestactions.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn intestactionsdetail<S: AsRef<str>>(mut self, intestactionsdetail: S) -> Self {
        self.data.intestactionsdetail = Some(intestactionsdetail.as_ref().to_string());
        self
    }
}

impl<NoTitlesOrPageids> ActionApiQueryInfoBuilder<NoTitlesOrPageids> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryInfoData::default(),
        }
    }

    pub fn titles<S: Into<String> + Clone>(
        mut self,
        titles: &[S],
    ) -> ActionApiQueryInfoBuilder<Runnable> {
        self.data.titles = Some(titles.iter().map(|s| s.clone().into()).collect());
        ActionApiQueryInfoBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn pageids(mut self, pageids: &[u64]) -> ActionApiQueryInfoBuilder<Runnable> {
        self.data.pageids = Some(pageids.to_vec());
        ActionApiQueryInfoBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl<Runnable> ActionApiRunnable for ActionApiQueryInfoBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "info".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiQuery};

    fn new_builder() -> ActionApiQueryInfoBuilder<NoTitlesOrPageids> {
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
            .intestactionsdetail("boolean")
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["intestactionsdetail"], "boolean");
    }

    #[test]
    fn intestactionsdetail_full() {
        let params = new_builder()
            .intestactionsdetail("full")
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
            .intestactionsdetail("full")
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
        let api = Api::new("https://en.wikipedia.org/w/api.php")
            .await
            .unwrap();
        let result = ActionApiQuery::info()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["pages"].is_object());
        // Albert Einstein page should be present
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }

    #[tokio::test]
    async fn test_info_by_title_with_inprop() {
        let api = Api::new("https://en.wikipedia.org/w/api.php")
            .await
            .unwrap();
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
        let api = Api::new("https://en.wikipedia.org/w/api.php")
            .await
            .unwrap();
        // Page ID 736 is Albert Einstein on English Wikipedia
        let result = ActionApiQuery::info()
            .pageids(&[736])
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["pages"]["736"].is_object());
    }
}
