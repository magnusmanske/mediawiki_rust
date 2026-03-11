use super::{
    ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData, ActionApiRunnable,
    NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone, Default)]
pub struct ActionApiWatchData {
    common: ActionApiQueryCommonData,
    expiry: Option<String>,
    unwatch: bool,
    token: Option<String>,
}

impl ActionApiData for ActionApiWatchData {}

impl ActionApiWatchData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_str(&self.expiry, "expiry", &mut params);
        Self::add_boolean(self.unwatch, "unwatch", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiWatchBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWatchData,
}

impl<T> ActionApiWatchBuilder<T> {
    pub fn expiry<S: AsRef<str>>(mut self, expiry: S) -> Self {
        self.data.expiry = Some(expiry.as_ref().to_string());
        self
    }

    pub fn unwatch(mut self, unwatch: bool) -> Self {
        self.data.unwatch = unwatch;
        self
    }

    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }
}

impl ActionApiWatchBuilder<NoTitlesOrGenerator> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWatchData::default(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiWatchBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiWatchBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiWatchBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWatchBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "watch".to_string());
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

    fn new_builder() -> ActionApiWatchBuilder<NoTitlesOrGenerator> {
        ActionApiWatchBuilder::new()
    }

    #[test]
    fn unwatch_set() {
        let params = new_builder().unwatch(true).titles(&["Foo"]).data.params();
        assert_eq!(params["unwatch"], "");
    }

    #[test]
    fn unwatch_false_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("unwatch"));
    }

    #[test]
    fn expiry_set() {
        let params = new_builder()
            .expiry("1 month")
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["expiry"], "1 month");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .token("watch+\\")
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["token"], "watch+\\");
    }

    #[test]
    fn titles_set() {
        let params = new_builder().titles(&["Main Page"]).data.params();
        assert_eq!(params["titles"], "Main Page");
    }

    #[test]
    fn action_is_watch() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "watch");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().titles(&["Foo"]);
        assert_eq!(builder.http_method(), "POST");
    }
}
