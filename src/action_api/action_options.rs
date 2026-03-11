use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoToken = NoTitlesOrGenerator;

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

#[derive(Debug, Clone)]
pub struct ActionApiOptionsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiOptionsData,
}

impl<T> ActionApiOptionsBuilder<T> {
    pub fn reset(mut self, reset: bool) -> Self {
        self.data.reset = reset;
        self
    }

    pub fn resetkinds<S: Into<String> + Clone>(mut self, resetkinds: &[S]) -> Self {
        self.data.resetkinds = Some(resetkinds.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn change<S: Into<String> + Clone>(mut self, change: &[S]) -> Self {
        self.data.change = Some(change.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn optionname<S: AsRef<str>>(mut self, optionname: S) -> Self {
        self.data.optionname = Some(optionname.as_ref().to_string());
        self
    }

    pub fn optionvalue<S: AsRef<str>>(mut self, optionvalue: S) -> Self {
        self.data.optionvalue = Some(optionvalue.as_ref().to_string());
        self
    }

    pub fn global<S: AsRef<str>>(mut self, global: S) -> Self {
        self.data.global = Some(global.as_ref().to_string());
        self
    }
}

impl ActionApiOptionsBuilder<NoToken> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiOptionsData::default(),
        }
    }

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
