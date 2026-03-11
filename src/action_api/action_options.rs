use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoToken = NoTitlesOrGenerator;

/// Internal data container for `action=options` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiOptionsData {
    reset: bool,
    resetkinds: Option<Vec<String>>,
    change: Option<Vec<String>>,
    optionname: Option<String>,
    optionvalue: Option<String>,
    global: Option<String>,
    token: Option<String>,
}

impl ActionApiData for ActionApiOptionsData {}

impl ActionApiOptionsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "options".to_string());
        Self::add_boolean(self.reset, "reset", &mut params);
        Self::add_vec(&self.resetkinds, "resetkinds", &mut params);
        Self::add_vec(&self.change, "change", &mut params);
        Self::add_str(&self.optionname, "optionname", &mut params);
        Self::add_str(&self.optionvalue, "optionvalue", &mut params);
        Self::add_str(&self.global, "global", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for `action=options`. Call `.token()` to set the CSRF token and make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiOptionsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiOptionsData,
}

impl<T> ActionApiOptionsBuilder<T> {
    /// Whether to reset all preferences to their default values (`reset`).
    pub fn reset(mut self, reset: bool) -> Self {
        self.data.reset = reset;
        self
    }

    /// List of types of options to reset when using `reset` (`resetkinds`).
    pub fn resetkinds<S: Into<String> + Clone>(mut self, resetkinds: &[S]) -> Self {
        self.data.resetkinds = Some(resetkinds.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// List of `name=value` pairs representing preference changes (`change`).
    pub fn change<S: Into<String> + Clone>(mut self, change: &[S]) -> Self {
        self.data.change = Some(change.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Name of the option to set (used together with `optionvalue`) (`optionname`).
    pub fn optionname<S: AsRef<str>>(mut self, optionname: S) -> Self {
        self.data.optionname = Some(optionname.as_ref().to_string());
        self
    }

    /// Value of the option named by `optionname` (`optionvalue`).
    pub fn optionvalue<S: AsRef<str>>(mut self, optionvalue: S) -> Self {
        self.data.optionvalue = Some(optionvalue.as_ref().to_string());
        self
    }

    /// Whether to apply option changes globally across all wikis (`global`).
    pub fn global<S: AsRef<str>>(mut self, global: S) -> Self {
        self.data.global = Some(global.as_ref().to_string());
        self
    }
}

impl ActionApiOptionsBuilder<NoToken> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiOptionsData::default(),
        }
    }

    /// CSRF token required to change preferences (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiOptionsBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiOptionsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiOptionsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }

    fn http_method(&self) -> &'static str {
        "POST"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiOptionsBuilder<NoToken> {
        ActionApiOptionsBuilder::new()
    }

    #[test]
    fn reset_flag() {
        let params = new_builder().token("csrf+\\").reset(true).data.params();
        assert_eq!(params["reset"], "");
    }

    #[test]
    fn reset_false_absent() {
        let params = new_builder().token("csrf+\\").data.params();
        assert!(!params.contains_key("reset"));
    }

    #[test]
    fn change_set() {
        let params = new_builder()
            .token("csrf+\\")
            .change(&["skin=vector", "language=en"])
            .data
            .params();
        assert_eq!(params["change"], "skin=vector|language=en");
    }

    #[test]
    fn optionname_set() {
        let params = new_builder()
            .token("csrf+\\")
            .optionname("skin")
            .data
            .params();
        assert_eq!(params["optionname"], "skin");
    }

    #[test]
    fn token_set() {
        let params = new_builder().token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_options() {
        let params = new_builder().token("csrf+\\").data.params();
        assert_eq!(params["action"], "options");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().token("csrf+\\");
        assert_eq!(builder.http_method(), "POST");
    }
}
