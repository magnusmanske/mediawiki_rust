use super::{ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `list=backlinks` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListBacklinksData {
    bltitle: Option<String>,
    blpageid: Option<u64>,
    blcontinue: Option<String>,
    blnamespace: Option<Vec<NamespaceID>>,
    bldir: Option<String>,
    blfilterredir: Option<String>,
    bllimit: usize,
    blredirect: bool,
}

impl ActionApiData for ActionApiListBacklinksData {}

impl Default for ActionApiListBacklinksData {
    fn default() -> Self {
        Self {
            bltitle: None,
            blpageid: None,
            blcontinue: None,
            blnamespace: None,
            bldir: None,
            blfilterredir: None,
            bllimit: 10,
            blredirect: false,
        }
    }
}

impl ActionApiListBacklinksData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.bltitle, "bltitle", &mut params);
        if let Some(blpageid) = self.blpageid {
            params.insert("blpageid".to_string(), blpageid.to_string());
        }
        Self::add_str(&self.blcontinue, "blcontinue", &mut params);
        if let Some(ns) = &self.blnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("blnamespace".to_string(), s.join("|"));
        }
        Self::add_str(&self.bldir, "bldir", &mut params);
        Self::add_str(&self.blfilterredir, "blfilterredir", &mut params);
        params.insert("bllimit".to_string(), self.bllimit.to_string());
        Self::add_boolean(self.blredirect, "blredirect", &mut params);
        params
    }
}

/// Builder for the `list=backlinks` API module; supports pagination via `ActionApiContinuable`.
#[derive(Debug, Clone)]
pub struct ActionApiListBacklinksBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListBacklinksData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListBacklinksBuilder<T> {
    /// Filter results to pages in these namespaces (`blnamespace`).
    pub fn blnamespace(mut self, blnamespace: &[NamespaceID]) -> Self {
        self.data.blnamespace = Some(blnamespace.to_vec());
        self
    }

    /// Sort direction for listing (`bldir`).
    pub fn bldir<S: AsRef<str>>(mut self, bldir: S) -> Self {
        self.data.bldir = Some(bldir.as_ref().to_string());
        self
    }

    /// Filter by redirect status of linking pages (`blfilterredir`).
    pub fn blfilterredir<S: AsRef<str>>(mut self, blfilterredir: S) -> Self {
        self.data.blfilterredir = Some(blfilterredir.as_ref().to_string());
        self
    }

    /// Maximum number of backlinks to return (`bllimit`).
    pub fn bllimit(mut self, bllimit: usize) -> Self {
        self.data.bllimit = bllimit;
        self
    }

    /// When set, also list pages that link to redirects pointing at the target (`blredirect`).
    pub fn blredirect(mut self, blredirect: bool) -> Self {
        self.data.blredirect = blredirect;
        self
    }
}

impl ActionApiListBacklinksBuilder<NoTitlesOrGenerator> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListBacklinksData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Title of the page to find backlinks for (`bltitle`).
    pub fn bltitle<S: AsRef<str>>(mut self, bltitle: S) -> ActionApiListBacklinksBuilder<Runnable> {
        self.data.bltitle = Some(bltitle.as_ref().to_string());
        ActionApiListBacklinksBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    /// Page ID of the page to find backlinks for (`blpageid`).
    pub fn blpageid(mut self, blpageid: u64) -> ActionApiListBacklinksBuilder<Runnable> {
        self.data.blpageid = Some(blpageid);
        ActionApiListBacklinksBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiListBacklinksBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "backlinks".to_string());
        params
    }
}

impl ActionApiRunnable for ActionApiListBacklinksBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "backlinks".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListBacklinksBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListBacklinksBuilder<NoTitlesOrGenerator> {
        ActionApiListBacklinksBuilder::new()
    }

    #[test]
    fn default_bllimit_is_10() {
        let params = new_builder().bltitle("Foo").data.params();
        assert_eq!(params["bllimit"], "10");
    }

    #[test]
    fn bltitle_set() {
        let params = new_builder().bltitle("Albert Einstein").data.params();
        assert_eq!(params["bltitle"], "Albert Einstein");
    }

    #[test]
    fn blpageid_set() {
        let params = new_builder().blpageid(736).data.params();
        assert_eq!(params["blpageid"], "736");
    }

    #[test]
    fn bltitle_does_not_set_pageid() {
        let params = new_builder().bltitle("Foo").data.params();
        assert!(!params.contains_key("blpageid"));
    }

    #[test]
    fn blnamespace_set() {
        let params = new_builder().blnamespace(&[0, 4]).bltitle("Foo").data.params();
        assert_eq!(params["blnamespace"], "0|4");
    }

    #[test]
    fn blfilterredir_set() {
        let params = new_builder().blfilterredir("nonredirects").bltitle("Foo").data.params();
        assert_eq!(params["blfilterredir"], "nonredirects");
    }

    #[test]
    fn bllimit_set() {
        let params = new_builder().bllimit(50).bltitle("Foo").data.params();
        assert_eq!(params["bllimit"], "50");
    }

    #[test]
    fn blredirect_true() {
        let params = new_builder().blredirect(true).bltitle("Foo").data.params();
        assert!(params.contains_key("blredirect"));
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().bltitle("Albert Einstein");
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "backlinks");
    }

    #[tokio::test]
    async fn test_backlinks() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::backlinks()
            .bltitle("Albert Einstein")
            .blnamespace(&[0])
            .bllimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["backlinks"].is_array());
    }
}
