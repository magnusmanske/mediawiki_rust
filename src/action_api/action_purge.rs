use super::{
    ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData, ActionApiRunnable,
    NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `action=purge` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiPurgeData {
    common: ActionApiQueryCommonData,
    forcelinkupdate: bool,
    forcerecursivelinkupdate: bool,
    redirects: bool,
    converttitles: bool,
}

impl ActionApiData for ActionApiPurgeData {}

impl ActionApiPurgeData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_boolean(self.forcelinkupdate, "forcelinkupdate", &mut params);
        Self::add_boolean(
            self.forcerecursivelinkupdate,
            "forcerecursivelinkupdate",
            &mut params,
        );
        Self::add_boolean(self.redirects, "redirects", &mut params);
        Self::add_boolean(self.converttitles, "converttitles", &mut params);
        params
    }
}

/// Builder for `action=purge`. Call `.titles()`, `.pageids()`, or a generator to specify pages, which makes it runnable.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiPurgeBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiPurgeData,
}

impl<T> ActionApiPurgeBuilder<T> {
    /// Whether to update the links tables for the purged pages (`forcelinkupdate`).
    pub fn forcelinkupdate(mut self, forcelinkupdate: bool) -> Self {
        self.data.forcelinkupdate = forcelinkupdate;
        self
    }

    /// Whether to recursively update link tables for pages that use the purged pages as templates (`forcerecursivelinkupdate`).
    pub fn forcerecursivelinkupdate(mut self, forcerecursivelinkupdate: bool) -> Self {
        self.data.forcerecursivelinkupdate = forcerecursivelinkupdate;
        self
    }

    /// Whether to automatically resolve redirects listed in the titles (`redirects`).
    pub fn redirects(mut self, redirects: bool) -> Self {
        self.data.redirects = redirects;
        self
    }

    /// Whether to convert titles to other language variants when appropriate (`converttitles`).
    pub fn converttitles(mut self, converttitles: bool) -> Self {
        self.data.converttitles = converttitles;
        self
    }
}

impl ActionApiPurgeBuilder<NoTitlesOrGenerator> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiPurgeData::default(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiPurgeBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiPurgeBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiPurgeBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiPurgeBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "purge".to_string());
        ret
    }

    fn http_method(&self) -> &'static str {
        "POST"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_api::ActionApiQueryCommonBuilder;

    fn new_builder() -> ActionApiPurgeBuilder<NoTitlesOrGenerator> {
        ActionApiPurgeBuilder::new()
    }

    #[test]
    fn forcelinkupdate_set() {
        let params = new_builder()
            .forcelinkupdate(true)
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["forcelinkupdate"], "");
    }

    #[test]
    fn forcelinkupdate_false_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("forcelinkupdate"));
    }

    #[test]
    fn titles_set() {
        let params = new_builder().titles(&["Main Page", "Talk:Foo"]).data.params();
        assert_eq!(params["titles"], "Main Page|Talk:Foo");
    }

    #[test]
    fn pageids_set() {
        let params = new_builder().pageids(&[1, 2, 3]).data.params();
        assert_eq!(params["pageids"], "1|2|3");
    }

    #[test]
    fn action_is_purge() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "purge");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().titles(&["Foo"]);
        assert_eq!(builder.http_method(), "POST");
    }
}
