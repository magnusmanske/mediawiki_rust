use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiThankData {
    rev: Option<u64>,
    log: Option<u64>,
    token: Option<String>,
    source: Option<String>,
}

impl ActionApiData for ActionApiThankData {}

impl ActionApiThankData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "thank".to_string());
        if let Some(v) = self.rev {
            params.insert("rev".to_string(), v.to_string());
        }
        if let Some(v) = self.log {
            params.insert("log".to_string(), v.to_string());
        }
        Self::add_str(&self.token, "token", &mut params);
        Self::add_str(&self.source, "source", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiThankBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiThankData,
}

impl<T> ActionApiThankBuilder<T> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }

    pub fn source<S: AsRef<str>>(mut self, source: S) -> Self {
        self.data.source = Some(source.as_ref().to_string());
        self
    }
}

impl ActionApiThankBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiThankData::default(),
        }
    }

    pub fn rev(mut self, rev: u64) -> ActionApiThankBuilder<Runnable> {
        self.data.rev = Some(rev);
        ActionApiThankBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn log(mut self, log: u64) -> ActionApiThankBuilder<Runnable> {
        self.data.log = Some(log);
        ActionApiThankBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiThankBuilder<Runnable> {
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

    fn new_builder() -> ActionApiThankBuilder<NoTarget> {
        ActionApiThankBuilder::new()
    }

    #[test]
    fn rev_set() {
        let params = new_builder().rev(12345).data.params();
        assert_eq!(params["rev"], "12345");
    }

    #[test]
    fn log_set() {
        let params = new_builder().log(67890).data.params();
        assert_eq!(params["log"], "67890");
    }

    #[test]
    fn source_set() {
        let params = new_builder().rev(1).source("diff").data.params();
        assert_eq!(params["source"], "diff");
    }

    #[test]
    fn token_set() {
        let params = new_builder().rev(1).token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_thank() {
        let params = new_builder().rev(1).data.params();
        assert_eq!(params["action"], "thank");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().rev(1);
        assert_eq!(builder.http_method(), "POST");
    }
}
