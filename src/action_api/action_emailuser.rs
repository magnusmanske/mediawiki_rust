use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiEmailuserData {
    target: Option<String>,
    subject: Option<String>,
    text: Option<String>,
    ccme: bool,
    token: Option<String>,
}

impl ActionApiData for ActionApiEmailuserData {}

impl ActionApiEmailuserData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "emailuser".to_string());
        Self::add_str(&self.target, "target", &mut params);
        Self::add_str(&self.subject, "subject", &mut params);
        Self::add_str(&self.text, "text", &mut params);
        Self::add_boolean(self.ccme, "ccme", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiEmailuserBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiEmailuserData,
}

impl<T> ActionApiEmailuserBuilder<T> {
    pub fn subject<S: AsRef<str>>(mut self, subject: S) -> Self {
        self.data.subject = Some(subject.as_ref().to_string());
        self
    }

    pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.data.text = Some(text.as_ref().to_string());
        self
    }

    pub fn ccme(mut self, ccme: bool) -> Self {
        self.data.ccme = ccme;
        self
    }

    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }
}

impl ActionApiEmailuserBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiEmailuserData::default(),
        }
    }

    pub fn target<S: AsRef<str>>(mut self, target: S) -> ActionApiEmailuserBuilder<Runnable> {
        self.data.target = Some(target.as_ref().to_string());
        ActionApiEmailuserBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiEmailuserBuilder<Runnable> {
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

    fn new_builder() -> ActionApiEmailuserBuilder<NoTarget> {
        ActionApiEmailuserBuilder::new()
    }

    #[test]
    fn target_set() {
        let params = new_builder().target("SomeUser").data.params();
        assert_eq!(params["target"], "SomeUser");
    }

    #[test]
    fn subject_set() {
        let params = new_builder()
            .target("SomeUser")
            .subject("Hello")
            .data
            .params();
        assert_eq!(params["subject"], "Hello");
    }

    #[test]
    fn text_set() {
        let params = new_builder()
            .target("SomeUser")
            .text("Message body")
            .data
            .params();
        assert_eq!(params["text"], "Message body");
    }

    #[test]
    fn ccme_flag() {
        let params = new_builder().target("SomeUser").ccme(true).data.params();
        assert_eq!(params["ccme"], "");
    }

    #[test]
    fn ccme_false_absent() {
        let params = new_builder().target("SomeUser").data.params();
        assert!(!params.contains_key("ccme"));
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .target("SomeUser")
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_emailuser() {
        let params = new_builder().target("SomeUser").data.params();
        assert_eq!(params["action"], "emailuser");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().target("SomeUser");
        assert_eq!(builder.http_method(), "POST");
    }
}
